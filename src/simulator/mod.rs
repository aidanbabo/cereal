pub mod loader;
pub mod decode;

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

pub struct Trace {
    current_pc: u16,
    current_instruction: u16,
    write_enable_flags: u16,
    register_write_value: u16,
    nzp_value: u16,
    data_write_address: u16,
    data_write_value: u16,
    register_write_register: u8,
}

const MEMORY_SIZE: usize = 1 << 16;

pub struct Machine {
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
    
    fn execute_instruction(&mut self, instruction: Instruction, trace: Option<&mut Trace>) -> Result<(), ()> {
        use core::ops::{BitAnd, Not, BitOr, BitXor};
        
        const P: u16 = 1;
        const Z: u16 = 2;
        const N: u16 = 4;
        const OS_MODE: u16 = 0x8000;
        
        fn nzp(machine: &mut Machine, value: i16) {
            use core::cmp::Ordering::*;
            let bits = match value.cmp(&0) {
                Less => N,
                Equal => Z,
                Greater => P,
            };
            machine.psr = (machine.psr & OS_MODE) | bits;
        }
        
        fn branch_on(machine: &Machine, bits: u16, immediate: i16) -> u16 {
            if machine.psr & bits > 0 {
                (machine.pc as i32 + 1 + immediate as i32) as u16 
            } else {
                machine.pc + 1
            }
        }
        
        // @Compilation do these ruin compile times ?
        fn binary<F: FnOnce(i16, i16) -> i16>(machine: &mut Machine, mut trace: Option<&mut Trace>, instruction: Instruction, f: F) {
            let rs = machine.registers[instruction.rs as usize];
            let rt = machine.registers[instruction.rt as usize];
            let rd = f(rs, rt);
            nzp(machine, rd);
            machine.registers[instruction.rd as usize] = rd;
            machine.pc += 1;

            if let Some(trace) = trace.as_deref_mut() {
                trace.register_write_value = rd as u16;
            }
        }

        fn binary_immediate<F: FnOnce(i16, i16) -> i16>(machine: &mut Machine, mut trace: Option<&mut Trace>, instruction: Instruction, f: F) {
            let rs = machine.registers[instruction.rs as usize];
            let immediate = instruction.immediate;
            let rd = f(rs, immediate);
            nzp(machine, rd);
            machine.registers[instruction.rd as usize] = rd;
            machine.pc += 1;

            if let Some(trace) = trace.as_deref_mut() {
                trace.register_write_value = rd as u16;
            }
        }
        
        fn cmp<T: Ord>(machine: &mut Machine, a: T, b: T) {
            use core::cmp::Ordering::*;
            let bits = match a.cmp(&b) {
                Less => N,
                Equal => Z,
                Greater => P,
            };
            machine.psr = (machine.psr & OS_MODE) | bits;
            machine.pc += 1;
        }
        
        // @Todo checks !
        fn load(machine: &mut Machine, _: Option<&mut Trace>, instruction: Instruction) -> Result<(), ()> {
            let address = (machine.registers[instruction.rs as usize] + instruction.immediate) as u16;
            machine.registers[instruction.rd as usize] = machine.memory[address as usize] as i16;
            machine.pc += 1;
            Ok(())
        }

        // @Todo checks !
        fn store(machine: &mut Machine, mut trace: Option<&mut Trace>, instruction: Instruction) -> Result<(), ()> {
            let address = (machine.registers[instruction.rs as usize] + instruction.immediate) as u16;
            let value = machine.registers[instruction.rt as usize] as u16;
            machine.memory[address as usize] = value;
            machine.pc += 1;

            if let Some(trace) = trace.as_deref_mut() {
                trace.data_write_address = address;
                trace.data_write_value = value;
            }

            Ok(())
        }

        match instruction.ty {
            InstructionType::Nop => self.pc = branch_on(self, 0, instruction.immediate),
            InstructionType::Brp => self.pc = branch_on(self, P, instruction.immediate),
            InstructionType::Brz => self.pc = branch_on(self, Z, instruction.immediate),
            InstructionType::Brzp => self.pc = branch_on(self, Z | P, instruction.immediate),
            InstructionType::Brn => self.pc = branch_on(self, N, instruction.immediate),
            InstructionType::Brnp => self.pc = branch_on(self, N | P, instruction.immediate),
            InstructionType::Brnz => self.pc = branch_on(self, N | Z, instruction.immediate),
            InstructionType::Brnzp => self.pc = branch_on(self, N | Z | P, instruction.immediate),
            InstructionType::Add => binary(self, trace, instruction, i16::wrapping_add),
            InstructionType::Mul => binary(self, trace, instruction, i16::wrapping_mul),
            InstructionType::Sub => binary(self, trace, instruction, i16::wrapping_sub),
            InstructionType::Div => binary(self, trace, instruction, move |a, b| i16::checked_div(a, b).unwrap_or(0)),
            InstructionType::Addi => binary_immediate(self, trace, instruction, i16::wrapping_add),
            InstructionType::Mod => binary(self, trace, instruction, move |a, b| i16::checked_rem(a, b).unwrap_or(0)),
            InstructionType::And => binary(self, trace, instruction, i16::bitand),
            InstructionType::Not => binary(self, trace, instruction, move |a, _| i16::not(a)),
            InstructionType::Or => binary(self, trace, instruction, i16::bitor),
            InstructionType::Xor => binary(self, trace, instruction, i16::bitxor),
            InstructionType::Andi => binary_immediate(self, trace, instruction, i16::bitand),
            InstructionType::Ldr => load(self, trace, instruction)?,
            InstructionType::Str => store(self, trace, instruction)?,
            InstructionType::Const => {
                self.registers[instruction.rd as usize] = instruction.immediate;
                self.pc += 1;
            },
            InstructionType::Hiconst => {
                let rd = &mut self.registers[instruction.rd as usize];
                *rd = (instruction.immediate << 8) | *rd & 0xff;
                self.pc += 1;
            },
            InstructionType::Cmp => {
                let rs = self.registers[instruction.rs as usize];
                let rt = self.registers[instruction.rt as usize];
                cmp(self, rs, rt);
            },
            InstructionType::Cmpu => {
                let rs = self.registers[instruction.rs as usize] as u16;
                let rt = self.registers[instruction.rt as usize] as u16;
                cmp(self, rs, rt);
            },
            InstructionType::Cmpi => {
                let rs = self.registers[instruction.rs as usize];
                let immediate = instruction.immediate;
                cmp(self, rs, immediate);
            },
            InstructionType::Cmpiu => {
                let rs = self.registers[instruction.rs as usize] as u16;
                let immediate = instruction.immediate as u16;
                cmp(self, rs, immediate);
            },
            InstructionType::Sll => binary_immediate(self, trace, instruction, move |a, b| a << b),
            InstructionType::Sra => binary_immediate(self, trace, instruction, move |a, b| a >> b),
            InstructionType::Srl => binary_immediate(self, trace, instruction, move |a, b| ((a as u16) >> (b as u16)) as i16),
            InstructionType::Jsrr => {
                let rs = self.registers[instruction.rs as usize] as u16;
                self.pc += 1;
                self.registers[7] = self.pc as i16;
                self.pc = rs;
            },
            InstructionType::Jsr => {
                let immediate = instruction.immediate as u16;
                self.pc += 1;
                self.registers[7] = self.pc as i16;
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
                self.registers[7] = self.pc as i16;
                self.pc = 0x8000 | instruction.immediate as u16;
                self.psr |= OS_MODE;
            },
            InstructionType::Rti => {
                self.pc += self.registers[7] as u16;
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

        if let Some(trace) = trace.as_deref_mut() {
            trace.nzp_value = self.psr & 0x0007;
        }
        
        Ok(())
    }
}