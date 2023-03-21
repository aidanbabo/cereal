use std::collections::HashMap;
use super::{decode, Trace, Instruction, InstructionType};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExecutionError {
    pub kind: ExecutionErrorKind,
    pub pc: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ExecutionErrorKind {
    PcRollover,
    InvalidJump {
        address: u16,
    },
    InvalidMemoryAccess {
        address: u16,
        lacks_privilege: bool,
        is_read: bool,
    },
    InvalidInstruction,
}

pub const P: u16 = 1;
pub const Z: u16 = 2;
pub const N: u16 = 4;
const OS_MODE: u16 = 0x8000;
const MEMORY_SIZE: usize = 1 << 16;

pub struct Machine {
    pub pc: u16,
    pub psr: u16,
    pub registers: [i16; 8],
    pub memory: Box<[u16; MEMORY_SIZE]>,
    pub symbols: HashMap<String, u16>,
}

impl Machine {
    pub fn new() -> Self {
        let memory = {
            let vec = vec![0; MEMORY_SIZE];
            let boxed_slice: Box<[u16]> = vec.into_boxed_slice();
            boxed_slice.try_into().unwrap()
        };
        Machine {
            pc: 0x8200,
            psr: OS_MODE | N,
            registers: [0; 8],
            memory,
            symbols: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.registers = [0; 8];
        for cell in self.memory.iter_mut() {
            *cell = 0;
        }
        self.pc = 0x8200;
        self.psr = OS_MODE | N;
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    fn os_mode(&self) -> bool {
        self.psr & OS_MODE > 0
    }

    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        trace: &mut Option<Trace>,
    ) -> Result<(), ExecutionError> {
        use core::ops::{BitAnd, BitOr, BitXor, Not};

        fn nzp(machine: &mut Machine, trace: &mut Option<Trace>, value: i16) {
            use core::cmp::Ordering::*;
            let bits = match value.cmp(&0) {
                Less => N,
                Equal => Z,
                Greater => P,
            };
            machine.psr = (machine.psr & OS_MODE) | bits;
            if let Some(trace) = trace {
                trace.nzp_value = bits;
            }
        }

        fn write_to_register(
            machine: &mut Machine,
            trace: &mut Option<Trace>,
            register: u8,
            value: i16,
        ) {
            machine.registers[register as usize] = value;
            nzp(machine, trace, value);
            if let Some(trace) = trace {
                trace.register_write_value = value as u16;
            }
        }

        fn branch_on(
            machine: &Machine,
            instruction: Instruction,
            bits: u16,
        ) -> Result<u16, ExecutionError> {
            let pc = if machine.psr & bits > 0 {
                (machine.pc as i32 + 1 + instruction.immediate as i32) as u16
            } else {
                machine.pc + 1
            };

            if (0x2000..0x8000).contains(&pc) || pc >= 0xA000 {
                Err(ExecutionError {
                    kind: ExecutionErrorKind::InvalidJump { address: pc },
                    pc: machine.pc,
                })
            } else {
                Ok(pc)
            }
        }

        fn jump_to(
            machine: &mut Machine,
            address: u16,
            from_pc_plus_one: bool,
        ) -> Result<(), ExecutionError> {
            if (0x2000..0x8000).contains(&address) || address >= 0xA000 {
                Err(ExecutionError {
                    kind: ExecutionErrorKind::InvalidJump { address },
                    pc: if from_pc_plus_one {
                        machine.pc + 1
                    } else {
                        machine.pc
                    },
                })
            } else {
                machine.pc = address;
                Ok(())
            }
        }

        // @Speed do these get inlined properly?
        fn binary(
            machine: &mut Machine,
            trace: &mut Option<Trace>,
            instruction: Instruction,
            f: fn(i16, i16) -> i16,
        ) {
            let rs = machine.registers[instruction.rs as usize];
            let rt = machine.registers[instruction.rt as usize];
            let rd = f(rs, rt);
            write_to_register(machine, trace, instruction.rd, rd);
            machine.pc += 1;
        }

        // @Speed do these get inlined properly?
        fn binary_immediate(
            machine: &mut Machine,
            trace: &mut Option<Trace>,
            instruction: Instruction,
            f: fn(i16, i16) -> i16,
        ) {
            let rs = machine.registers[instruction.rs as usize];
            let immediate = instruction.immediate;
            let rd = f(rs, immediate);
            write_to_register(machine, trace, instruction.rd, rd);
            machine.pc += 1;
        }

        fn cmp<T: Ord>(machine: &mut Machine, trace: &mut Option<Trace>, a: T, b: T) {
            use core::cmp::Ordering::*;
            let bits = match a.cmp(&b) {
                Less => N,
                Equal => Z,
                Greater => P,
            };
            machine.psr = (machine.psr & OS_MODE) | bits;
            if let Some(trace) = trace {
                trace.nzp_value = bits;
            }
            machine.pc += 1;
        }

        fn check_address(
            machine: &Machine,
            address: u16,
            is_read: bool,
        ) -> Result<(), ExecutionError> {
            let lacks_privilege = if address >= 0x8000 && !machine.os_mode() {
                true
            } else if address < 0x2000 || (0x8000..0xA000).contains(&address) {
                false
            } else {
                return Ok(());
            };

            Err(ExecutionError {
                kind: ExecutionErrorKind::InvalidMemoryAccess {
                    address,
                    lacks_privilege,
                    is_read,
                },
                pc: machine.pc,
            })
        }

        fn load(
            machine: &mut Machine,
            trace: &mut Option<Trace>,
            instruction: Instruction,
        ) -> Result<(), ExecutionError> {
            let address =
                (machine.registers[instruction.rs as usize] + instruction.immediate) as u16;
            check_address(machine, address, true)?;
            let value = machine.memory[address as usize] as i16;
            write_to_register(machine, trace, instruction.rd, value);
            machine.pc += 1;

            if let Some(trace) = trace {
                trace.data_access_address = address;
                trace.data_access_value = value as u16;
            }

            Ok(())
        }

        fn store(
            machine: &mut Machine,
            trace: &mut Option<Trace>,
            instruction: Instruction,
        ) -> Result<(), ExecutionError> {
            let address =
                (machine.registers[instruction.rs as usize] + instruction.immediate) as u16;
            check_address(machine, address, false)?;
            let value = machine.registers[instruction.rt as usize] as u16;
            machine.memory[address as usize] = value;
            machine.pc += 1;

            if let Some(trace) = trace {
                trace.data_access_address = address;
                trace.data_access_value = value;
            }

            Ok(())
        }

        match instruction.ty {
            InstructionType::Nop => self.pc = branch_on(self, instruction, 0)?,
            InstructionType::Brp => self.pc = branch_on(self, instruction, P)?,
            InstructionType::Brz => self.pc = branch_on(self, instruction, Z)?,
            InstructionType::Brzp => self.pc = branch_on(self, instruction, Z | P)?,
            InstructionType::Brn => self.pc = branch_on(self, instruction, N)?,
            InstructionType::Brnp => self.pc = branch_on(self, instruction, N | P)?,
            InstructionType::Brnz => self.pc = branch_on(self, instruction, N | Z)?,
            InstructionType::Brnzp => self.pc = branch_on(self, instruction, N | Z | P)?,
            InstructionType::Add => binary(self, trace, instruction, i16::wrapping_add),
            InstructionType::Mul => binary(self, trace, instruction, i16::wrapping_mul),
            InstructionType::Sub => binary(self, trace, instruction, i16::wrapping_sub),
            InstructionType::Div => binary(self, trace, instruction, move |a, b| {
                i16::checked_div(a, b).unwrap_or(0)
            }),
            InstructionType::Addi => binary_immediate(self, trace, instruction, i16::wrapping_add),
            InstructionType::Mod => binary(self, trace, instruction, move |a, b| {
                i16::checked_rem(a, b).unwrap_or(0)
            }),
            InstructionType::And => binary(self, trace, instruction, i16::bitand),
            InstructionType::Not => binary(self, trace, instruction, move |a, _| i16::not(a)),
            InstructionType::Or => binary(self, trace, instruction, i16::bitor),
            InstructionType::Xor => binary(self, trace, instruction, i16::bitxor),
            InstructionType::Andi => binary_immediate(self, trace, instruction, i16::bitand),
            InstructionType::Ldr => load(self, trace, instruction)?,
            InstructionType::Str => store(self, trace, instruction)?,
            InstructionType::Const => {
                let value = instruction.immediate;
                write_to_register(self, trace, instruction.rd, value);
                self.pc += 1;
            }
            InstructionType::Hiconst => {
                let rd = self.registers[instruction.rd as usize];
                let value = (instruction.immediate << 8) | rd & 0xff;
                write_to_register(self, trace, instruction.rd, value);
                self.pc += 1;
            }
            InstructionType::Cmp => {
                let rs = self.registers[instruction.rs as usize];
                let rt = self.registers[instruction.rt as usize];
                cmp(self, trace, rs, rt);
            }
            InstructionType::Cmpu => {
                let rs = self.registers[instruction.rs as usize] as u16;
                let rt = self.registers[instruction.rt as usize] as u16;
                cmp(self, trace, rs, rt);
            }
            InstructionType::Cmpi => {
                let rs = self.registers[instruction.rs as usize];
                let immediate = instruction.immediate;
                cmp(self, trace, rs, immediate);
            }
            InstructionType::Cmpiu => {
                let rs = self.registers[instruction.rs as usize] as u16;
                let immediate = instruction.immediate as u16;
                cmp(self, trace, rs, immediate);
            }
            InstructionType::Sll => binary_immediate(self, trace, instruction, move |a, b| a << b),
            InstructionType::Sra => binary_immediate(self, trace, instruction, move |a, b| a >> b),
            InstructionType::Srl => binary_immediate(self, trace, instruction, move |a, b| {
                ((a as u16) >> (b as u16)) as i16
            }),
            InstructionType::Jsrr => {
                let rs = self.registers[instruction.rs as usize] as u16;
                self.pc += 1;
                write_to_register(self, trace, 7, self.pc as i16);
                jump_to(self, rs, true)?;
            }
            InstructionType::Jsr => {
                let immediate = instruction.immediate as u16;
                self.pc += 1;
                write_to_register(self, trace, 7, self.pc as i16);
                jump_to(self, (self.pc & OS_MODE) | (immediate << 4), true)?;
            }
            InstructionType::Jmpr => {
                let rs = self.registers[instruction.rs as usize] as u16;
                jump_to(self, rs, false)?;
            }
            InstructionType::Jmp => {
                let immediate = instruction.immediate;
                jump_to(self, (self.pc as i16 + 1 + immediate) as u16, false)?;
            }
            InstructionType::Trap => {
                self.pc += 1;
                let value = self.pc as i16;
                write_to_register(self, trace, 7, value);
                self.pc = 0x8000 | instruction.immediate as u16;
                self.psr |= OS_MODE;
            }
            InstructionType::Rti => {
                self.psr &= !OS_MODE;
                jump_to(self, self.registers[7] as u16, false)?;
            }
        }

        // All jump style instructions check their jump location ahead of time
        // So here we just check if we ran over from the previous instruction
        if self.pc >= 0x2000 && self.pc < 0x8000 || self.pc >= 0xA000 {
            Err(ExecutionError {
                kind: ExecutionErrorKind::PcRollover,
                pc: self.pc - 1,
            })
        } else {
            Ok(())
        }
    }

    pub fn step(&mut self, trace: &mut Option<Trace>) -> Result<(), ExecutionError> {
        let pc = self.pc;
        let instruction_word = self.memory[pc as usize];
        let instruction = decode::decode(instruction_word, trace).map_err(|_| ExecutionError {
            kind: ExecutionErrorKind::InvalidInstruction,
            pc: self.pc,
        })?;

        if let Some(trace) = trace {
            trace.current_pc = pc;
            trace.current_instruction = instruction_word;
        }

        self.execute_instruction(instruction, trace)?;

        Ok(())
    }
}