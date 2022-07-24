pub mod loader;
pub mod decode;

use std::io::{self, Write};

#[derive(Copy, Clone)]
#[repr(u8)]
enum InstructionType {
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
    Addi,
    Mod,
    And,
    Not,
    Or,
    Xor,
    Andi,
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
}

impl InstructionType {
    pub fn to_mnemonic(self) -> &'static str {
        use InstructionType::*;
        match self {
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
            Addi => "add",
            Mod => "mod",
            And => "and",
            Not => "not",
            Or => "or",
            Xor => "xor",
            Andi => "and",
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
        }
    }
}

struct Instruction {
    ty: InstructionType,
    rd: u8,
    rs: u8,
    rt: u8,
    immediate: i16,
}

mod write_enable_consts {
    pub const NZP_WRITE_ENABLE: u16 = 1;
    pub const REGISTER_FILE_WRITE_ENABLE: u16 = 2;
    pub const DATA_WRITE_ENABLE: u16 = 4;
}
use write_enable_consts::*;

struct Trace {
    current_pc: u16,
    current_instruction: u16,
    write_enable_flags: u16,
    register_write_value: u16,
    nzp_value: u16,
    data_access_address: u16,
    data_access_value: u16,
    register_write_register: u8,
}

impl Trace {
    pub const fn new() -> Self {
        Trace {
            current_pc: 0,
            current_instruction: 0,
            write_enable_flags: 0,
            register_write_value: 0,
            nzp_value: 0,
            data_access_address: 0,
            data_access_value: 0,
            register_write_register: 0,
        }
    }
    
    pub fn write_to_file(self, writer: &mut impl Write) -> io::Result<()> {
        
        let nzp_write_enable = (self.write_enable_flags & NZP_WRITE_ENABLE) >> 0;
        let register_file_write_enable = (self.write_enable_flags & REGISTER_FILE_WRITE_ENABLE) >> 1;
        let data_write_enable = (self.write_enable_flags & DATA_WRITE_ENABLE) >> 2;
        
        writeln!(writer, "{:04X} {:016b} {} {} {:04X} {} {} {} {:04X} {:04X}",
            self.current_pc,
            self.current_instruction,
            register_file_write_enable,
            self.register_write_register,
            self.register_write_value,
            nzp_write_enable,
            self.nzp_value,
            data_write_enable,
            self.data_access_address,
            self.data_access_value,
        )
    }
}

const MEMORY_SIZE: usize = 1 << 16;

struct Machine {
    pc: u16,
    psr: u16,
    registers: [i16; 8],
    memory: Box<[u16; MEMORY_SIZE]>,
}

impl Machine {
    pub fn new() -> Self {
        let memory = {
            let vec = vec![0; MEMORY_SIZE];
            let boxed_slice: Box<[u16]> = vec.into_boxed_slice();
            let ptr = Box::into_raw(boxed_slice) as *mut [u16; MEMORY_SIZE];
            // Is it worth having unsave to save a single usize on the Machine ?
            unsafe { Box::from_raw(ptr) }
        };
        Machine {
            pc: 0x8200,
            psr: 0x8002,
            registers: [0; 8],
            memory,
        }
    }
    
    pub fn pc(&self) -> u16 {
        self.pc
    }
    
    fn execute_instruction(&mut self, instruction: Instruction, trace: Option<&mut Trace>) -> Result<(), ()> {
        use core::ops::{BitAnd, Not, BitOr, BitXor};
        
        const P: u16 = 1;
        const Z: u16 = 2;
        const N: u16 = 4;
        const OS_MODE: u16 = 0x8000;
        
        fn nzp(machine: &mut Machine, mut trace: Option<&mut Trace>, value: i16) {
            use core::cmp::Ordering::*;
            let bits = match value.cmp(&0) {
                Less => N,
                Equal => Z,
                Greater => P,
            };
            machine.psr = (machine.psr & OS_MODE) | bits;
            if let Some(trace) = trace.as_deref_mut() {
                trace.nzp_value = bits;
            }
        }
        
        fn write_to_register(machine: &mut Machine, mut trace: Option<&mut Trace>, register: u8, value: i16) {
            machine.registers[register as usize] = value;
            nzp(machine, trace.as_deref_mut(), value);
            if let Some(trace) = trace.as_deref_mut() {
                trace.register_write_value = value as u16;
            }
        }
        
        fn branch_on(machine: &Machine, immediate: i16, bits: u16) -> u16 {
            if machine.psr & bits > 0 {
                (machine.pc as i32 + 1 + immediate as i32) as u16 
            } else {
                machine.pc + 1
            }
        }
        
        // @Speed do these get inlines properly?
        fn binary(machine: &mut Machine, mut trace: Option<&mut Trace>, instruction: Instruction, f: fn(i16, i16) -> i16) {
            let rs = machine.registers[instruction.rs as usize];
            let rt = machine.registers[instruction.rt as usize];
            let rd = f(rs, rt);
            write_to_register(machine, trace.as_deref_mut(), instruction.rd, rd);
            machine.pc += 1;
        }

        // @Speed do these get inlines properly?
        fn binary_immediate(machine: &mut Machine, mut trace: Option<&mut Trace>, instruction: Instruction, f: fn(i16, i16) -> i16) {
            let rs = machine.registers[instruction.rs as usize];
            let immediate = instruction.immediate;
            let rd = f(rs, immediate);
            write_to_register(machine, trace.as_deref_mut(), instruction.rd, rd);
            machine.pc += 1;
        }
        
        fn cmp<T: Ord>(machine: &mut Machine, mut trace: Option<&mut Trace>, a: T, b: T) {
            use core::cmp::Ordering::*;
            let bits = match a.cmp(&b) {
                Less => N,
                Equal => Z,
                Greater => P,
            };
            machine.psr = (machine.psr & OS_MODE) | bits;
            if let Some(trace) = trace.as_deref_mut() {
                trace.nzp_value = bits;
            }
            machine.pc += 1;
        }
        
        // @Todo checks !
        fn load(machine: &mut Machine, mut trace: Option<&mut Trace>, instruction: Instruction) -> Result<(), ()> {
            let address = (machine.registers[instruction.rs as usize] + instruction.immediate) as u16;
            let value = machine.memory[address as usize] as i16;
            write_to_register(machine, trace.as_deref_mut(), instruction.rd, value);
            machine.pc += 1;

            if let Some(trace) = trace.as_deref_mut() {
                trace.data_access_address = address;
                trace.data_access_value = value as u16;
            }

            Ok(())
        }

        // @Todo checks !
        fn store(machine: &mut Machine, mut trace: Option<&mut Trace>, instruction: Instruction) -> Result<(), ()> {
            let address = (machine.registers[instruction.rs as usize] + instruction.immediate) as u16;
            let value = machine.registers[instruction.rt as usize] as u16;
            machine.memory[address as usize] = value;
            machine.pc += 1;

            if let Some(trace) = trace.as_deref_mut() {
                trace.data_access_address = address;
                trace.data_access_value = value;
            }

            Ok(())
        }

        match instruction.ty {
            InstructionType::Nop   => self.pc = branch_on(self, instruction.immediate, 0),
            InstructionType::Brp   => self.pc = branch_on(self, instruction.immediate, P),
            InstructionType::Brz   => self.pc = branch_on(self, instruction.immediate, Z),
            InstructionType::Brzp  => self.pc = branch_on(self, instruction.immediate, Z | P),
            InstructionType::Brn   => self.pc = branch_on(self, instruction.immediate, N),
            InstructionType::Brnp  => self.pc = branch_on(self, instruction.immediate, N | P),
            InstructionType::Brnz  => self.pc = branch_on(self, instruction.immediate, N | Z),
            InstructionType::Brnzp => self.pc = branch_on(self, instruction.immediate, N | Z | P),
            InstructionType::Add => binary(self, trace, instruction, i16::wrapping_add),
            InstructionType::Mul => binary(self, trace, instruction, i16::wrapping_mul),
            InstructionType::Sub => binary(self, trace, instruction, i16::wrapping_sub),
            InstructionType::Div => binary(self, trace, instruction, move |a, b| i16::checked_div(a, b).unwrap_or(0)),
            InstructionType::Addi => binary_immediate(self, trace, instruction, i16::wrapping_add),
            InstructionType::Mod => binary(self, trace, instruction, move |a, b| i16::checked_rem(a, b).unwrap_or(0)),
            InstructionType::And => binary(self, trace, instruction, i16::bitand),
            InstructionType::Not => binary(self, trace, instruction, move |a, _| i16::not(a)),
            InstructionType::Or  => binary(self, trace, instruction, i16::bitor),
            InstructionType::Xor => binary(self, trace, instruction, i16::bitxor),
            InstructionType::Andi => binary_immediate(self, trace, instruction, i16::bitand),
            InstructionType::Ldr => load(self, trace, instruction)?,
            InstructionType::Str => store(self, trace, instruction)?,
            InstructionType::Const => {
                let value = instruction.immediate;
                write_to_register(self, trace, instruction.rd, value);
                self.pc += 1;
            },
            InstructionType::Hiconst => {
                let rd = self.registers[instruction.rd as usize];
                let value = (instruction.immediate << 8) | rd & 0xff;
                write_to_register(self, trace, instruction.rd, value);
                self.pc += 1;
            },
            InstructionType::Cmp => {
                let rs = self.registers[instruction.rs as usize];
                let rt = self.registers[instruction.rt as usize];
                cmp(self, trace, rs, rt);
            },
            InstructionType::Cmpu => {
                let rs = self.registers[instruction.rs as usize] as u16;
                let rt = self.registers[instruction.rt as usize] as u16;
                cmp(self, trace, rs, rt);
            },
            InstructionType::Cmpi => {
                let rs = self.registers[instruction.rs as usize];
                let immediate = instruction.immediate;
                cmp(self, trace, rs, immediate);
            },
            InstructionType::Cmpiu => {
                let rs = self.registers[instruction.rs as usize] as u16;
                let immediate = instruction.immediate as u16;
                cmp(self, trace, rs, immediate);
            },
            InstructionType::Sll => binary_immediate(self, trace, instruction, move |a, b| a << b),
            InstructionType::Sra => binary_immediate(self, trace, instruction, move |a, b| a >> b),
            InstructionType::Srl => binary_immediate(self, trace, instruction, move |a, b| ((a as u16) >> (b as u16)) as i16),
            InstructionType::Jsrr => {
                let rs = self.registers[instruction.rs as usize] as u16;
                self.pc += 1;
                write_to_register(self, trace, 7, self.pc as i16);
                self.pc = rs;
            },
            InstructionType::Jsr => {
                let immediate = instruction.immediate as u16;
                self.pc += 1;
                write_to_register(self, trace, 7, self.pc as i16);
                self.pc = (self.pc & OS_MODE) | (immediate << 4);
            },
            InstructionType::Jmpr => {
                let rs = self.registers[instruction.rs as usize] as u16;
                self.pc = rs;
            },
            InstructionType::Jmp => {
                let immediate = instruction.immediate;
                self.pc = (self.pc as i16 + 1 + immediate) as u16;
            },
            InstructionType::Trap => {
                self.pc += 1;
                let value = self.pc as i16;
                write_to_register(self, trace, 7, value);
                self.pc = 0x8000 | instruction.immediate as u16;
                self.psr |= OS_MODE;
            },
            InstructionType::Rti => {
                self.pc = self.registers[7] as u16;
                self.psr &= !OS_MODE;
            },
        }
        Ok(())
    }
    
    pub fn step(&mut self, mut trace: Option<&mut Trace>) -> Result<(), ()> {
        let pc = self.pc;
        let instruction_word = self.memory[pc as usize];
        let instruction = decode::decode(instruction_word, trace.as_deref_mut()).expect("Valid Instruction");

        if let Some(trace) = trace.as_deref_mut() {
            trace.current_pc = pc;
            trace.current_instruction = instruction_word;
        }
        
        self.execute_instruction(instruction, trace.as_deref_mut()).expect("No execution errors.");

        Ok(())
    }
}

use std::path::PathBuf;
pub struct Options {
    pub trace_path: Option<PathBuf>,
    pub input_paths: Vec<PathBuf>,
    pub step_cap: Option<u64>,
}

pub fn run(options: Options) {
    let mut machine = Machine::new();
    for path in &options.input_paths {
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("There was an error opening file {:?}: {}", path, e);
                continue;
            }
        };
        // loader::load(&bytes, &mut machine, Some(&mut std::io::stdout())).expect("Load failure");
        loader::load(&bytes, &mut machine, None).expect("Load failure");
    }
    
    let mut trace_file = options.trace_path.as_ref().map(|path| {
        let file = std::fs::File::create(path).expect("Invalid file");
        std::io::BufWriter::new(file)
    });
    
    let mut steps = 0;
    while machine.pc() != 0x80ff {
        steps += 1;
        match options.step_cap {
            Some(cap) if steps > cap => break,     // @Todo error
            _ => {},
        }

        let mut trace = if options.trace_path.is_some() {
            Some(Trace::new())
        } else {
            None
        };
        match machine.step(trace.as_mut()) {
            Ok(()) => {},
            Err(()) => {
                eprintln!("Error");
                break;
            }
        }
        if let Some(trace) = trace {
            trace.write_to_file(trace_file.as_mut().unwrap()).expect("Failed to write to a file");
        }
    }
}
