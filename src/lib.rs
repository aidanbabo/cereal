use std::collections::HashMap;

pub mod lexer;
pub mod parser;

use lexer::Lexer;
use parser::{Parser, Block, BlockType, Data, InstructionWithLabel};

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

use std::path::Path;
pub fn parse_string<'container, 'source>(
    filename: &Path, 
    string: &'source str, 
    blocks: &'container mut Vec<Block<'source>>, 
    constants: &'container mut HashMap<&'source str, i32>
) -> Result<(), ()> 
    {
    let lexer = Lexer::new(string);
    
    let mut tokens = vec![];
    let mut errors = vec![];
    
    // println!("SOURCE:");
    // println!("{}", string);

    for token in lexer {
        match token {
            Ok(token) => if errors.is_empty() { tokens.push(token) },
            Err(error) => errors.push(error),
        }
    }
    
    // println!();
    if !errors.is_empty() {
        for (line, error) in errors {
            println!("ERROR in file {:?} on line {}: {}", filename, line, error);
        }
        return Err(());
    }

    /*
    println!("TOKENS:");
    for token in &tokens {
        println!("{:?}", token);
    }
    */
    
    let parser = Parser::new(tokens, constants);
    
    let mut errors = vec![];
    for block in parser {
        match block {
            Ok(block) => if errors.is_empty() { blocks.push(block) },
            Err(error) => { errors.push(error); break; } ,
        }
    }
    
    // println!();

    if !errors.is_empty() {
        for error in errors {
            println!("ERROR in file {:?}: {}", filename, error);
        }
        return Err(());
    }

    /*
    println!("BLOCKS:");
    for block in &blocks {
        println!("{:?}", block);
    }
    */
    
    Ok(())
}

pub fn compile_and_link(blocks: &mut [Block], constants: &HashMap<&str, i32>) -> Result<Vec<u8>, ()> {
    
    // println!("PRINTED:");
    // cereal::parser::print_blocks(&blocks, &parser.constants).unwrap();
    
    if let Err(errors) = expand_psuedo_instructions(blocks, constants) {
        for error in errors {
            println!("ERROR: {}", error);
        }
        return Err(());
    }
    
    // println!("EXPANDED:");
    // cereal::parser::print_blocks(&blocks, &parser.constants).unwrap();

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
    // cereal::parser::print_blocks(&blocks, &parser.constants).unwrap();
    
    let bytes = write_object_code(&blocks, &labels);
    Ok(bytes)
}

pub fn expand_psuedo_instructions(blocks: &mut [Block], constants: &HashMap<&str, i32>) -> Result<(), Vec<String>> {
    
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

pub fn patch<'a>(blocks: &mut [Block<'a>]) -> Result<HashMap<&'a str, u16>, Vec<String>> {

    let mut addresses = HashMap::new();
    let mut errors = vec![];
    let mut code_addr = 0;
    let mut data_addr = 0x2000;
    
    for block in &mut* blocks {
        let (addr, size) = match &block.ty {
            BlockType::Code(instructions) => (&mut code_addr, instructions.len() as u16),
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
                (&mut data_addr, size)
            }
        };

        if let Some(a) = block.addr {
            *addr = a;
        }

        if block.aligned && *addr & 0xf != 0 {
            *addr |= 0xf;
            *addr += 1;
        }
        
        block.addr = Some(*addr);
        
        if let BlockType::Data(_) = &block.ty {
            println!("{:?}", block.labels);
            println!("{:x}", block.addr.unwrap());
        }
        
        for label in &block.labels {
            if let Some(old_addr) = addresses.insert(*label, *addr) {
                errors.push(format!("Label '{}' is already defined at address {:x}", label, old_addr));
            }
        }
        *addr += size;
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
                                    // @Todo Idk what more to say, but this feels like a shitty error message
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

pub fn write_object_code(blocks: &[Block], labels: &HashMap<&str, u16>) -> Vec<u8> {
    const CODE_HEADER   : u16 = 0xCADE;
    const DATA_HEADER   : u16 = 0xDADA;
    const SYMBOL_HEADER : u16 = 0xC3B7;
    // const FILE_HEADER   : u16 = 0xF17E;
    // const LINE_HEADER   : u16 = 0x715E;
    
    fn write_be(bytes: &mut Vec<u8>, short: u16) {
        bytes.push(((short & 0xff00) >> 8) as u8);
        bytes.push((short & 0xff) as u8);
    }

    fn write_be_signed(bytes: &mut Vec<u8>, short: i16) {
        bytes.push(((short >> 8) & 0xff) as u8);
        bytes.push((short & 0xff) as u8);
    }

    let mut bytes = Vec::new();
    
    for block in blocks {
        let address = block.addr.unwrap();
        match &block.ty {
            BlockType::Code(instructions) => {
                if instructions.is_empty() { continue; }
                write_be(&mut bytes, CODE_HEADER);
                write_be(&mut bytes, address);
                write_be(&mut bytes, instructions.len() as u16);
                bytes.reserve(instructions.len() * 2);
                
                for instruction in instructions {
                    let encoded = encode_instruction(instruction);
                    write_be(&mut bytes, encoded);
                }
                
            },
            BlockType::Data(data) => {
                if data.is_empty() { continue; }
                
                println!("Writing data address {:x} labeled {:?}", address, &block.labels);

                let mut size = 0;
                for datum in data {
                    let new = match datum {
                        Data::Block(s) => *s,
                        Data::Word(_) => 1,
                        Data::Stringz(s) => s.len() as u16 + 1,
                    };
                    size += new;
                }

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
    
    for (label, address) in labels {
        write_be(&mut bytes, SYMBOL_HEADER);
        write_be(&mut bytes, *address);
        write_be(&mut bytes, label.len() as u16);
        bytes.extend_from_slice(label.as_bytes());
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
