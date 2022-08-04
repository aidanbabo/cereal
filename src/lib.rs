use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::marker::PhantomData;
use std::fs::{self, File};
use std::io::Write;

pub mod printer;
pub mod assembler;
pub mod ir;
pub mod c;
pub mod char_utils;
pub mod simulator;

const CODE_HEADER   : u16 = 0xCADE;
const DATA_HEADER   : u16 = 0xDADA;
const SYMBOL_HEADER : u16 = 0xC3B7;
const FILE_HEADER   : u16 = 0xF17E;
const LINE_HEADER   : u16 = 0x715E;

    
#[derive(Clone, Copy, Debug)]
pub struct Span<'source> {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    _phantom: PhantomData<&'source ()>,
}

impl<'source> Span<'source> {
    pub fn new(_s: &'source str, start: usize, end: usize, line: usize) -> Self {
        Span {
            start,
            end,
            line,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Spanned<'source, T> {
    t: T,
    pub span: Span<'source>,
}

impl<'source, T> Spanned<'source, T> {
    pub fn new(t: T, span: Span<'source>) -> Self {
        Spanned {
            t,
            span,
        }
    }
}

trait Spannable<'source> {
    fn spanned(self, span: Span<'source>) -> Spanned<'source, Self> where Self: Sized;
}

impl<'source, T> Spannable<'source> for T {
    fn spanned(self, span: Span<'source>) -> Spanned<'source, T> {
        Spanned::new(self, span)
    }
}

use std::ops;
impl<'source, T> ops::Deref for Spanned<'source, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<'source, T> ops::DerefMut for Spanned<'source, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

pub type S<'source, T> = Spanned<'source, T>;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Reg {
    Rd,
    Rs,
    Rt,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Operand {
    Register {
        register: Reg
    },
    Immediate {
        signed: bool,
        bits: u8,
    },
    Label,
    RegisterOrImmediate {
        register: Reg,
        signed: bool,
        bits: u8,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstructionType {
    Nop,
    Brp,
    Brz,
    Brzp,
    Brn,
    Brnp,
    Brnz,
    Brnzp,
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    And,
    Not,
    Or,
    Xor,
    Ldr,
    Str,
    Const,
    Hiconst,
    Cmp,
    Cmpu,
    Cmpi,
    Cmpiu,
    Sll,
    Sra,
    Srl,
    Jsrr,
    Jsr,
    Jmpr,
    Jmp,
    Trap,
    Rti,
    Ret,
    Lea,
    Lc,
}

#[derive(Clone, Copy, Debug)]
pub struct InstructionWithLabel<'a> {
    pub ty: InstructionType,
    pub rd: i8,
    pub rs: i8,
    pub rt: i8,
    pub immediate: i32,
    pub label: Option<&'a str>,
}

#[derive(Debug)]
pub enum Data<'a> {
    Block(u16),
    Stringz(Cow<'a, str>),
    Word(i16),
}

#[derive(Debug)]
pub enum BlockType<'a> {
    Code(Vec<InstructionWithLabel<'a>>),
    Data(Vec<Data<'a>>),
}

#[derive(Debug)]
pub struct Block<'a> {
    pub addr: Option<u16>,
    pub aligned: bool,
    pub labels: Vec<&'a str>,
    pub ty: BlockType<'a>,
}

impl<'a> Block<'a> {
    fn is_empty(&self) -> bool {
        self.addr.is_none() && self.labels.is_empty() && match &self.ty {
            BlockType::Code(instructions) => instructions.is_empty(),
            BlockType::Data(data) => data.is_empty(),
        }
    }
    
    fn size(&self) -> u16 {
        match &self.ty {
            BlockType::Code(instructions) => instructions.len() as u16,
            BlockType::Data(data) => {
                let mut size = 0;
                for datum in data {
                    let new = match datum {
                        Data::Block(s) => *s,
                        Data::Word(_) => 1,
                        Data::Stringz(s) => s.len() as u16 + 1,
                    };
                    size += new;
                }
                size
            }
        }
    }
}

impl InstructionType {
    pub fn encoding_base(self) -> u16 {
        use InstructionType::*;
        match self {
            Nop     => 0x0000,
            Brp     => 0x0200,
            Brz     => 0x0400,
            Brzp    => 0x0600,
            Brn     => 0x0800,
            Brnp    => 0x0a00,
            Brnz    => 0x0c00,
            Brnzp   => 0x0e00,
            Add     => 0x1000,
            Mul     => 0x1008,
            Sub     => 0x1010,
            Div     => 0x1018,
            Mod     => 0xa030,
            And     => 0x5000,
            Not     => 0x5008,
            Or      => 0x5010,
            Xor     => 0x5018,
            Ldr     => 0x6000,
            Str     => 0x7000,
            Const   => 0x9000,
            Hiconst => 0xd000,
            Cmp     => 0x2000,
            Cmpu    => 0x2080,
            Cmpi    => 0x2100,
            Cmpiu   => 0x2180,
            Sll     => 0xa000,
            Sra     => 0xa010,
            Srl     => 0xa020,
            Jsrr    => 0x4000,
            Jsr     => 0x4800,
            Jmpr    => 0xc000,
            Jmp     => 0xc800,
            Trap    => 0xf000,
            Rti     => 0x8000,
            Ret | Lea | Lc => panic!("Internal error: {} should never get to the code generation stage!", self),
        }
    }
}

use std::fmt;

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InstructionType::*;

        let s = match *self {
            Nop => "nop",
            Brp => "brp",
            Brz => "brz",
            Brzp => "brzp",
            Brn => "brn",
            Brnp => "brnp",
            Brnz => "brnz",
            Brnzp => "brnzp",
            Add => "add",
            Mul => "mul",
            Sub => "sub",
            Div => "div",
            Mod => "mod",
            And => "and",
            Not => "not",
            Or => "or",
            Xor => "xor",
            Ldr => "ldr",
            Str => "str",
            Const => "const",
            Hiconst => "hiconst",
            Cmp => "cmp",
            Cmpu => "cmpu",
            Cmpi => "cmpi",
            Cmpiu => "cmpiu",
            Sll => "sll",
            Sra => "sra",
            Srl => "srl",
            Jsrr => "jsrr",
            Jsr => "jsr",
            Jmpr => "jmpr",
            Jmp => "jmp",
            Trap => "trap",
            Rti => "rti",
            Ret => "ret",
            Lea => "lea",
            Lc => "lc",
        };
        write!(f, "{}", s)
    }
}

pub fn number_fits(i: i32, signed: bool, bits: u8) -> bool {
    let mut min = 0i32;
    let mut max = 1i32 << bits;
    if signed {
        let change = 1i32 << (bits - 1);
        min -= change;
        max -= change;
    }
    i >= min && i < max
}

pub fn assemble<'container, 'source>(
    filename: &Path, 
    string: &'source str, 
    blocks: &'container mut Vec<Block<'source>>, 
    constants: &'container mut HashMap<&'source str, i32>
) -> Result<(), ()> {
    assembler::parse_string(filename, string, blocks, constants)
}

pub fn compile_c<'container, 'source>(
    filename: &Path, 
    string: &'source str, 
    blocks: &'container mut Vec<Block<'source>>, 
    constants: &'container mut HashMap<&'source str, i32>
) -> Result<(), ()> {
    c::compile(filename, string, blocks, constants)
}

pub struct Options {
    pub output_path: PathBuf,
    pub debug_info: bool,
    pub input_paths: Vec<PathBuf>,
}

pub fn compile(options: Options) -> Result<(), ()> {

    let mut blocks = Vec::new();
    let mut constants = HashMap::new();
    let mut file_strings = Vec::new();
    
    for path in &options.input_paths {
        let string = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to open file '{:?}': {}", path, e);
                return Err(());
            }
        };
        file_strings.push(string);
    }

    for i in 0..options.input_paths.len() {
        let path = &options.input_paths[i];
        let extension = if let Some(e) = path.extension() {
            e
        } else {
            println!("Cannot read file path extension for file '{:?}'", path);
            return Err(());
        };

        if extension == "asm" {
            match assemble(path, &file_strings[i], &mut blocks, &mut constants) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
        } else if extension == "c" {
            match compile_c(path, &file_strings[i], &mut blocks, &mut constants) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
        } else {
            println!("ERROR: Only accepting .asm and .c files as inputs. Cannot compile '{:?}'", path);
            return Err(());
        }
    }
    
    let bytes = match link(&mut blocks, &constants, options.debug_info) {
        Ok(bytes) => bytes,
        Err(())=> return Err(()),
    };

    let mut file = match File::create(&options.output_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Could not create object file '{:?}': {}.", &options.output_path, error);
            return Err(());
        }
    };

    if let Err(error) = file.write_all(&bytes[..]) {
        println!("Failed to write to object file '{:?}': {}.", &options.output_path, error);
        return Err(());
    }
    
    Ok(())
}

pub fn link(blocks: &mut [Block], constants: &HashMap<&str, i32>, debug_info: bool) -> Result<Vec<u8>, ()> {
    
    // println!("PRINTED:");
    // printer::print_blocks(blocks, constants).unwrap();
    
    if let Err(errors) = expand_psuedo_instructions(blocks, constants) {
        for error in errors {
            println!("ERROR: {}", error);
        }
        return Err(());
    }
    
    // println!("EXPANDED:");
    // printer::print_blocks(blocks, constants).unwrap();

    let labels = match patch(blocks) {
        Ok(labels) => labels,
        Err(errors) => {
            for error in errors {
                println!("ERROR: {}", error);
            }
            return Err(());
        }
    };

    // println!("PATCHED:");
    // printer::print_blocks(blocks, constants).unwrap();
    
    let bytes = write_object_code(&blocks, &labels, debug_info);
    Ok(bytes)
}

fn expand_psuedo_instructions(blocks: &mut [Block], constants: &HashMap<&str, i32>) -> Result<(), Vec<String>> {
    
    let mut errors = vec![];
    
    for block in blocks {
        if let BlockType::Code(instructions) = &mut block.ty {
            for i in (0..instructions.len()).rev() {
                let mut instruction = &mut instructions[i];
                match instruction.ty {
                    InstructionType::Lea => {
                        instruction.ty = InstructionType::Const;
                        let mut instruction = instruction.clone();
                        instruction.ty = InstructionType::Hiconst;
                        instructions.insert(i + 1, instruction);
                    }
                    InstructionType::Lc => {
                        let value = if let Some(value) = constants.get(instruction.label.unwrap()) {
                            value
                        } else {
                            errors.push(format!("No such label '{}'.", instruction.label.unwrap()));
                            continue;
                        };
                        
                        instruction.ty = InstructionType::Const;
                        instruction.immediate = value & 0x000001ff;
                        instruction.label = None;
                        
                        let high_bits = (value & 0x0000ff00) >> 8;
                        if high_bits == 0 || high_bits == 0xff {
                            continue;
                        }
                        
                        let mut instruction = instruction.clone();
                        instruction.ty = InstructionType::Hiconst;
                        instruction.immediate = high_bits;
                        instructions.insert(i + 1, instruction);
                    }
                    InstructionType::Ret => {
                        instruction.ty = InstructionType::Jmpr;
                        instruction.rs = 7;
                    }
                    _ => {}
                }
            }
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn patch<'a>(blocks: &mut [Block<'a>]) -> Result<HashMap<&'a str, u16>, Vec<String>> {
    
    struct Region<'s> {
        label: &'s str,
        start: u16,
        end: u16,
    }

    let mut addresses = HashMap::new();
    let mut errors = Vec::new();
    let mut code_addr = 0;
    let mut data_addr = 0x2000;
    
    // we could probably make this n*log(n) instead of n^2
    let mut regions = Vec::new();
    
    for block in &mut* blocks {
        let size = block.size();
        let addr = match &block.ty {
            BlockType::Code(_) => &mut code_addr,
            BlockType::Data(_) => &mut data_addr,
        };

        if let Some(a) = block.addr {
            *addr = a;
        }

        if block.aligned && *addr & 0xf != 0 {
            *addr |= 0xf;
            *addr += 1;
        }
        
        block.addr = Some(*addr);
        
        for label in &block.labels {
            if let Some(old_addr) = addresses.insert(*label, *addr) {
                errors.push(format!("Label '{}' is already defined at address {:x}.", label, old_addr));
            }
        }
        
        let end = *addr + size;
        let label = block.labels.get(0).unwrap_or(&"Unlabeled");
        let region = Region {
            label, 
            start: *addr, 
            end,
        };
        
        for &Region{ label, start, end } in &regions {
            if region.end <= start || end <= region.start {
                continue;
            }
            errors.push(format!("Overlapping blocks: Block {} is {:x}-{:x} and block {} is {:x}-{:x}.", label, start, end, region.label, region.start, region.end));
        }
        
        match &block.ty {
            BlockType::Code(_) => {
                let in_user = region.end <= 0x2000;
                let in_os = region.start >= 0x8000 && region.end <= 0xA000;
                if !(in_user || in_os) {
                    errors.push(format!("Code block labeled {} is not in the correct section of memory. Range {:x}-{:x}.", region.label, region.start, region.end));
                }
            }
            BlockType::Data(_) => {
                let in_user = region.start >= 0x2000 && region.end <= 0x8000;
                let in_os = region.start >= 0xA000;
                if !(in_user || in_os) {
                    errors.push(format!("Data block labeled {} is not in the correct section of memory. Range {:x}-{:x}.", region.label, region.start, region.end));
                }
            }
        }
        
        regions.push(region);

        *addr = end;
    }
    
    if !errors.is_empty() {
        return Err(errors);
    }
    
    use InstructionType::*;
    for block in blocks {
        if let BlockType::Code(instructions) = &mut block.ty {
            let top_addr = block.addr.unwrap() as i32;
            for (i, instruction) in instructions.into_iter().enumerate() {
                if let Some(label) = instruction.label {
                    instruction.label = None;
                    if let Some(address) = addresses.get(&label) {
                        let current = top_addr + i as i32;
                        match instruction.ty {
                            Brp | Brz | Brzp | Brn | Brnp | Brnz | Brnzp | Jmp => {
                                instruction.immediate = (*address) as i32 - current - 1;
                                if !number_fits(instruction.immediate, true, if matches!(instruction.ty, Jmp) { 11 } else { 9 }) {
                                    errors.push(format!("Jump to label '{}' is too far.", label));
                                    continue;
                                }
                            },
                            Jsr => {
                                if address & 0x0f != 0 {
                                    errors.push(format!("Cannot jump to subroutine of not aligned label '{}'.", label));
                                    continue;
                                }
                                let address = address >> 4;
                                instruction.immediate = address as i32;
                                if !number_fits(instruction.immediate, true, 11){
                                    errors.push(format!("Jump to subroutine to label '{}' is too far. You cannot jump to subroutines in user/os space if you are in os/user space.", label));
                                    continue;
                                }
                            }
                            Const => instruction.immediate = (*address as i32) & 0x1ff,
                            Hiconst => instruction.immediate = ((*address as i32) & 0xff00) >> 8,
                            _ => {},
                        }
                    } else {
                        errors.push(format!("Label '{}' is not defined.", label));
                        continue;
                    }
                }
            }
        }
    }
    
    if errors.is_empty() {
        Ok(addresses)
    } else {
        Err(errors)
    }
}

fn write_object_code(blocks: &[Block], labels: &HashMap<&str, u16>, debug_info: bool) -> Vec<u8> {
    fn write_be(bytes: &mut Vec<u8>, short: u16) {
        bytes.push(((short & 0xff00) >> 8) as u8);
        bytes.push((short & 0xff) as u8);
    }

    fn write_be_signed(bytes: &mut Vec<u8>, short: i16) {
        bytes.push(((short >> 8) & 0xff) as u8);
        bytes.push((short & 0xff) as u8);
    }

    let mut bytes = Vec::new();
    
    if debug_info {
        for (label, address) in labels {
            write_be(&mut bytes, SYMBOL_HEADER);
            write_be(&mut bytes, *address);
            write_be(&mut bytes, label.len() as u16);
            bytes.extend_from_slice(label.as_bytes());
        }
    }
    
    for block in blocks {
        let address = block.addr.unwrap();
        let size = block.size();
        match &block.ty {
            BlockType::Code(instructions) => {
                if instructions.is_empty() { continue; }
                write_be(&mut bytes, CODE_HEADER);
                write_be(&mut bytes, address);
                write_be(&mut bytes, size);
                bytes.reserve(size as usize * 2);
                
                for instruction in instructions {
                    let encoded = encode_instruction(instruction);
                    write_be(&mut bytes, encoded);
                }
                
            },
            BlockType::Data(data) => {
                if data.is_empty() { continue; }
                write_be(&mut bytes, DATA_HEADER);
                write_be(&mut bytes, address);
                write_be(&mut bytes, size);
                bytes.reserve(size as usize * 2);
                
                for datum in data {
                    match datum {
                        Data::Block(s) => {
                            for _ in 0..*s {
                                write_be(&mut bytes, 0);
                            }
                        }
                        Data::Word(w) => {
                            write_be_signed(&mut bytes, *w);
                        }
                        Data::Stringz(s) => {
                            for b in s.as_bytes() {
                                write_be(&mut bytes, *b as u16);
                            }
                            write_be(&mut bytes, 0);
                        }
                    }
                }
            },
        }
    }
    
    bytes
}

fn encode_instruction(instruction: &InstructionWithLabel) -> u16 {
    use InstructionType::*;
    
    let mut encoded = instruction.ty.encoding_base();
    match instruction.ty {
        Nop | Rti => {},
        Brp | Brz | Brzp | Brn | Brnp | Brnz | Brnzp => {
            encoded |= instruction.immediate as u16 & 0x1ff;
        }
        Mul | Sub | Div | Mod | Or | Xor => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= (instruction.rt as u16) << 0;
        }
        Add | And => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            if instruction.rt != -1 {
                encoded |= (instruction.rt as u16) << 0;
            } else {
                encoded |= (instruction.immediate & 0x1f) as u16;
                encoded |= 1 << 5;
            }
        },
        Not => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
        },
        Ldr => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= (instruction.immediate & 0x3f) as u16;
        },
        Str => {
            encoded |= (instruction.rt as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= (instruction.immediate & 0x3f) as u16;
        },
        Const => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.immediate & 0x1ff) as u16;
        },
        Hiconst => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.immediate & 0xff) as u16;
        },
        Cmp | Cmpu => {
            encoded |= (instruction.rs as u16) << 9;
            encoded |= (instruction.rt as u16) << 0;
        },
        Cmpi | Cmpiu => {
            encoded |= (instruction.rs as u16) << 9;
            encoded |= (instruction.immediate & 0x7f) as u16;
        },
        Sll | Sra | Srl => {
            encoded |= (instruction.rd as u16) << 9;
            encoded |= (instruction.rs as u16) << 6;
            encoded |= (instruction.immediate & 0xf) as u16;
        }
        Jsrr | Jmpr => {
            encoded |= (instruction.rs as u16) << 6;
        },
        Jsr | Jmp => {
            encoded |= (instruction.immediate & 0x7ff) as u16;
        },
        Trap => {
            encoded |= (instruction.immediate & 0xff) as u16;
        },
        _ => unreachable!(),
    }
    encoded
}

fn instruction_operands(instruction_type: InstructionType, ops: &mut [Operand]) -> &[Operand] {
    use InstructionType::*;
    use Operand::*;
    use Reg::*;
    let specs: &'static [Operand] = match instruction_type {
        Nop | Ret | Rti => &[],
        Brp | Brz | Brzp | Brn | Brnp | Brnz | Brnzp | Jsr | Jmp => &[Label],
        Lea | Lc => &[
            Register { register: Rd }, 
            Label,
        ],
        And | Add => &[
            Register { register: Rd },
            Register { register: Rs },
            RegisterOrImmediate { register: Rt, signed: true, bits: 5 },
        ],
        Mul | Sub | Div | Mod | Or | Xor => &[
            Register { register: Rd },
            Register { register: Rs },
            Register { register: Rt },
        ],
        Sll | Sra | Srl => &[
            Register { register: Rd },
            Register { register: Rs },
            Immediate { signed: false, bits: 4 },
        ],
        Not => &[
            Register { register: Rd },
            Register { register: Rs },
        ],
        Ldr => &[
            Register { register: Rd },
            Register { register: Rs },
            Immediate { signed: true, bits: 6 },
        ],
        Str => &[
            Register { register: Rt },
            Register { register: Rs },
            Immediate { signed: true, bits: 6 },
        ],
        Const => &[
            Register { register: Rd },
            Immediate { signed: true, bits: 9 },
        ],
        Hiconst => &[
            Register { register: Rd },
            Immediate { signed: false, bits: 8 },
        ],
        Cmp | Cmpu => &[
            Register { register: Rs },
            Register { register: Rt },
        ],
        Cmpi => &[
            Register { register: Rs },
            Immediate { signed: true, bits: 7 },
        ],
        Cmpiu => &[
            Register { register: Rs },
            Immediate { signed: false, bits: 7 },
        ],
        Jsrr | Jmpr => &[
            Register { register: Rs },
        ],
        Trap => &[
            Immediate { signed: false, bits: 8 },
        ],
    };
    for i in 0..specs.len() {
        ops[i] = specs[i];
    }
    &ops[..specs.len()]
}

mod insn {
    use super::*;
    use InstructionType::*;

    pub fn add(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Add,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn sub(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Sub,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn mul(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Mul,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn div(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Div,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn mod_(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Mod,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn addi(rd: i8, rs: i8, immediate: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Add,
            rd,
            rs,
            rt: -1,
            immediate,
            label: None,
        }
    }

    pub fn and(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: And,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn not(rd: i8, rs: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Not,
            rd,
            rs,
            rt: -1,
            immediate: -1,
            label: None,
        }
    }

    pub fn xor(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Xor,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn or(rd: i8, rs: i8, rt: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Or,
            rd,
            rs,
            rt,
            immediate: -1,
            label: None,
        }
    }

    pub fn ldr(value: i8, addr: i8, offset: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Ldr,
            rd: value,
            rs: addr,
            rt: -1,
            immediate: offset,
            label: None,
        }
    }

    pub fn str(value: i8, addr: i8, offset: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Str,
            rd: -1,
            rs: addr,
            rt: value,
            immediate: offset,
            label: None,
        }
    }

    pub fn konst(dest: i8, value: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Const,
            rd: dest,
            rs: -1,
            rt: -1,
            immediate: value,
            label: None,
        }
    }

    pub fn hiconst(dest: i8, value: i32) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Hiconst,
            rd: dest,
            rs: -1,
            rt: -1,
            immediate: value,
            label: None,
        }
    }

    pub fn jsr<'s>(dest: &'s str) -> InstructionWithLabel<'s> {
        InstructionWithLabel {
            ty: Jsr,
            rd: -1,
            rs: -1,
            rt: -1,
            immediate: -1,
            label: Some(dest),
        }
    }

    pub fn jmpr(dest: i8) -> InstructionWithLabel<'static> {
        InstructionWithLabel {
            ty: Jmpr,
            rd: -1,
            rs: dest,
            rt: -1,
            immediate: -1,
            label: None,
        }
    }

    pub fn lea<'s>(reg: i8, label: &'s str) -> InstructionWithLabel<'s> {
        InstructionWithLabel {
            ty: Lea,
            rd: reg,
            rs: -1,
            rt: -1,
            immediate: -1,
            label: Some(label),
        }
    }
}

