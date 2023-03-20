use std::collections::{BTreeMap, HashMap};

mod command;
mod decode;
mod loader;

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

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.ty {
            InstructionType::Nop => write!(f, "{}", self.ty.to_mnemonic()),
            InstructionType::Brp => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Brz => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Brzp => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Brn => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Brnp => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Brnz => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Brnzp => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Add => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Mul => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Sub => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Div => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Addi => write!(
                f,
                "{} r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate
            ),
            InstructionType::Mod => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::And => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Not => {
                write!(f, "{} r{}, r{}", self.ty.to_mnemonic(), self.rd, self.rs)
            }
            InstructionType::Or => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Xor => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Andi => write!(
                f,
                "{} r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate
            ),
            InstructionType::Ldr => write!(
                f,
                "{} r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate
            ),
            InstructionType::Str => write!(
                f,
                "{} r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rt,
                self.rs,
                self.immediate
            ),
            InstructionType::Const => write!(
                f,
                "{} r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.immediate
            ),
            InstructionType::Hiconst => write!(
                f,
                "{} r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.immediate
            ),
            InstructionType::Cmp => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Cmpu => write!(
                f,
                "{} r{}, r{}, r{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.rt
            ),
            InstructionType::Cmpi => write!(
                f,
                "{} r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate
            ),
            InstructionType::Cmpiu => write!(
                f,
                "{} r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate as i16 as u16
            ),
            InstructionType::Sll => write!(
                f,
                "{}, r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate
            ),
            InstructionType::Sra => write!(
                f,
                "{}, r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate
            ),
            InstructionType::Srl => write!(
                f,
                "{}, r{}, r{}, #{}",
                self.ty.to_mnemonic(),
                self.rd,
                self.rs,
                self.immediate
            ),
            InstructionType::Jsrr => write!(f, "{} r{}", self.ty.to_mnemonic(), self.rs),
            InstructionType::Jsr => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Jmpr => write!(f, "{} r{}", self.ty.to_mnemonic(), self.rs),
            InstructionType::Jmp => write!(f, "{} #{}", self.ty.to_mnemonic(), self.immediate),
            InstructionType::Trap => {
                write!(f, "{} 0x{:02x}", self.ty.to_mnemonic(), self.immediate)
            }
            InstructionType::Rti => write!(f, "{}", self.ty.to_mnemonic()),
        }
    }
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
        let nzp_write_enable = self.write_enable_flags & NZP_WRITE_ENABLE;
        let register_file_write_enable =
            (self.write_enable_flags & REGISTER_FILE_WRITE_ENABLE) >> 1;
        let data_write_enable = (self.write_enable_flags & DATA_WRITE_ENABLE) >> 2;

        writeln!(
            writer,
            "{:04X} {:016b} {} {} {:04X} {} {} {} {:04X} {:04X}",
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

#[allow(dead_code)]
#[derive(Debug)]
struct ExecutionError {
    kind: ExecutionErrorKind,
    pc: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
enum ExecutionErrorKind {
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

const P: u16 = 1;
const Z: u16 = 2;
const N: u16 = 4;
const OS_MODE: u16 = 0x8000;
const MEMORY_SIZE: usize = 1 << 16;

struct Machine {
    pc: u16,
    psr: u16,
    registers: [i16; 8],
    memory: Box<[u16; MEMORY_SIZE]>,
    symbols: HashMap<String, u16>,
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

use std::path::PathBuf;
pub struct Options {
    pub trace_path: Option<PathBuf>,
    pub input_paths: Vec<PathBuf>,
    pub step_cap: Option<u64>,
    pub loader_trace: bool,
    pub headless: bool,
}

use eframe::egui;

pub(crate) struct CerealApp {
    machine: Machine,
    command: String,
    command_index: Option<usize>,
    command_history: Vec<String>,
    command_output: String,
    breakpoints: BTreeMap<u16, String>,
}

impl CerealApp {
    fn new(machine: Machine) -> Self {
        CerealApp {
            machine,
            command: String::new(),
            command_index: None,
            command_history: Default::default(),
            command_output: String::new(),
            breakpoints: Default::default(),
        }
    }
}

impl CerealApp {
    fn command(&mut self, ui: &mut egui::Ui) {
        ui.label("Command");
        

        let modified: bool;
        if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            if let Some(ci) = &mut self.command_index {
                *ci += 1;
                if *ci >= self.command_history.len() {
                    self.command_index = None;
                }
            }
            modified = true;
        } else if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            if let Some(ci) = &mut self.command_index {
                if *ci != 0 {
                    *ci -= 1;
                }
            } else {
                if self.command_history.is_empty() {
                    self.command_index = None;
                } else {
                    self.command_index = Some(self.command_history.len() - 1);
                }
            }
            modified = true;
        } else {
            modified = false;
        }

        if modified {
            self.command = self.command_index
                .map(|ci| self.command_history[ci].clone())
                .unwrap_or(String::new())
        }

        let mut output = egui::TextEdit::singleline(&mut self.command).desired_width(f32::INFINITY).show(ui);
        if output.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            use egui::widgets::text_edit::CCursorRange;
            use egui::text::CCursor;

            self.command_history.push(self.command.to_string());
            command::command(self);
            output.response.request_focus();
            output.state.set_ccursor_range(Some(CCursorRange::two(CCursor::new(0), CCursor::new(self.command.chars().count()))));
            output.state.store(ui.ctx(), output.response.id);
        }
        
        let scroll_area = egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .max_height(100.0)
            .stick_to_bottom(true);
        scroll_area.show(ui, |ui| {
            ui.set_height(100.0);
            egui::TextEdit::multiline(&mut &*self.command_output).desired_width(f32::INFINITY).show(ui);
        });
    }

    fn registers(&mut self, ui: &mut egui::Ui) {

        let register = |ui: &mut egui::Ui, i| ui.horizontal(|ui| {
            ui.label(format!("R{i}"));
            ui.label(&format!("x{:04X}", self.machine.registers[i])); 
        });

        ui.label("Registers");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                register(ui, 0);
                register(ui, 1);
                register(ui, 2);
                register(ui, 3);
                register(ui, 4);
                register(ui, 5);
            });
            ui.vertical(|ui| {
                register(ui, 6);
                register(ui, 7);
                ui.horizontal(|ui| {
                    ui.label("PC");
                    ui.label(&format!("x{:04X}", self.machine.pc)); 
                });
                ui.horizontal(|ui| {
                    ui.label("");
                    ui.label("");
                });
                ui.horizontal(|ui| {
                    ui.label("PSR");
                    ui.label(&format!("x{:04X}", self.machine.psr)); 
                });
                ui.horizontal(|ui| {
                    ui.label("CC");
                    if self.machine.psr & P > 0 {
                        ui.label("P");
                    } else if self.machine.psr & N > 0 {
                        ui.label("N");
                    } else {
                        ui.label("Z");
                    }
                });
            });
        });
    }

    fn devices(&mut self, ui: &mut egui::Ui) {
        fn unpack(b: u16, s: u8) -> u8 {
            ((b >> s) as u8 & (((1 << 5) - 1))) << 3
        }

        let memory_start = 0xC000;
        let memory_end = 0xFDFF;
        let mut pixel_data = Vec::with_capacity(128 * 124);
        for addr in memory_start..memory_end+1 {
            let data = self.machine.memory[addr];
            let color = egui::Color32::from_rgb(unpack(data, 10), unpack(data, 5), unpack(data, 0));
            pixel_data.push(color);
        }

        let image_data = egui::ImageData::Color(egui::ColorImage {
            size: [128, 124], 
            pixels: pixel_data,
        });

        let texture = ui.ctx().load_texture("Display", image_data, egui::TextureOptions::NEAREST);
        ui.image(&texture, [128.0 * 2.0, 124.0 * 2.0]);
    }

    fn memory(&mut self, ui: &mut egui::Ui) {
        ui.label("Memory");

        let scroll_area = egui::ScrollArea::vertical()
            .max_height(400.0)
            .max_width(300.0)
            .auto_shrink([false; 2])
            .always_show_scroll(true);

        let row_height = ui.text_style_height(&egui::TextStyle::Body);
        scroll_area.show_rows(ui, row_height, u16::MAX as usize + 1, |ui, row_range| {
            ui.set_height(400.0);
            for row in row_range {
                let text = if row > 0xfdff {
                    format!("Address: x{:04X} Value ???", row)
                } else {
                    format!("Address: x{:04X} Value {}", row, self.machine.memory[row])
                };
                ui.label(text);
            }
        });
    }

    fn show_breakpoints(&mut self, ui: &mut egui::Ui) {
        ui.label("Breakpoints");

        let scroll_area = egui::ScrollArea::vertical()
            .max_height(200.0)
            .max_width(300.0)
            .auto_shrink([false; 2])
            .always_show_scroll(true);

        let row_height = ui.text_style_height(&egui::TextStyle::Body);
        scroll_area.show_rows(ui, row_height, self.breakpoints.len(), |ui, row_range| {
            ui.set_height(400.0);
            let iter = self.breakpoints.iter().skip(row_range.start).take(row_range.end - row_range.start);
            for (&addr, label) in iter {
                let text = if addr > 0xfdff {
                    format!("x{:04X} ({}) Value ???", addr, label)
                } else {
                    format!("x{:04X} ({}) Value {}", addr, label, self.machine.memory[addr as usize])
                };
                ui.label(text);
            }
        });
    }
}

impl eframe::App for CerealApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();

            self.command(ui);

            ui.horizontal(|ui| {
                ui.push_id("Registers and Device", |ui| {
                    ui.vertical(|ui| {
                        self.registers(ui);
                        self.devices(ui);
                    });
                });
                ui.push_id("Memory", |ui| {
                    ui.vertical(|ui| {
                        self.memory(ui);
                    });
                });
                ui.push_id("Breakpoints and Dumps", |ui| {
                    ui.vertical(|ui| {
                        self.show_breakpoints(ui);
                    });
                });
            });
        });
    }
}

// @Todo keep the machine around after an error
pub fn run(options: Options) -> i16 {
    let mut machine = Machine::new();

    let mut stdout = std::io::stdout();

    for path in &options.input_paths {
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("There was an error opening file {:?}: {}", path, e);
                continue;
            }
        };
        let loader_trace = options.loader_trace.then_some(&mut stdout as _); // unsizing coercion
        loader::load(&bytes, &mut machine, loader_trace).expect("Load failure");
    }

    let mut trace_file = options.trace_path.as_ref().map(|path| {
        let file = std::fs::File::create(path).expect("Invalid file");
        std::io::BufWriter::new(file)
    });

    if !options.headless {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1040.0, 860.0)),
            ..Default::default()
        };
        eframe::run_native(
            "Cereal Sim",
            options,
            Box::new(|_cc| Box::new(CerealApp::new(machine))),
        ).unwrap();

        0
    } else {
        let mut steps = 0;
        while machine.pc() != 0x80ff {
            steps += 1;
            match options.step_cap {
                Some(cap) if steps > cap => panic!("exceeded step limit"),
                _ => {}
            }

            let mut trace = options.trace_path.as_ref().map(|_| Trace::new());
            match machine.step(&mut trace) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    break;
                }
            }
            if let Some(trace) = trace {
                trace
                    .write_to_file(trace_file.as_mut().unwrap())
                    .expect("Failed to write to a file");
            }
        }

        machine.registers[0]
    }

}
