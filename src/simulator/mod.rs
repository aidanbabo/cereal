use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::PathBuf;

mod command;
mod decode;
mod loader;
mod machine;

use machine::{Machine, ExecutionError};

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
                self.immediate as u16
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

pub struct Trace {
    pub current_pc: u16,
    pub current_instruction: u16,
    pub write_enable_flags: u16,
    pub register_write_value: u16,
    pub nzp_value: u16,
    pub data_access_address: u16,
    pub data_access_value: u16,
    pub register_write_register: u8,
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
#[derive(Default)]
pub struct Options {
    pub trace_path: Option<PathBuf>,
    pub input_paths: Vec<PathBuf>,
    pub step_cap: Option<u64>,
    pub loader_trace: bool,
    pub headless: bool,
    pub from_directory: Option<PathBuf>,
    pub startup_script: Option<PathBuf>,
}

use eframe::egui;

#[derive(Default, PartialEq, Eq)]
enum ExecutionState {
    Running,
    #[default]
    Suspended,
}

#[derive(Default)]
pub(crate) struct CerealApp {
    machine: Machine,
    command: String,
    command_index: Option<usize>,
    command_history: Vec<String>,
    command_output: String,
    script_commands: Vec<String>,
    breakpoints: BTreeMap<u16, String>,
    trace: Option<Box<dyn Write>>,
    execution_state: ExecutionState,
}

impl CerealApp {
    fn new(machine: Machine, startup_script: Option<PathBuf>) -> Self {
        let mut app = CerealApp {
            machine,
            ..Default::default()
        };
        if let Some(path) = startup_script {
            command::command(&mut app, None, &format!("script {}", path.to_string_lossy()));
        }
        app
    }
}

impl CerealApp {
    fn command(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
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
            } else if self.command_history.is_empty() {
                self.command_index = None;
            } else {
                self.command_index = Some(self.command_history.len() - 1);
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
            command::command(self, Some(frame), &self.command.to_string());
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
                    if self.machine.psr & machine::P > 0 {
                        ui.label("P");
                    } else if self.machine.psr & machine::N > 0 {
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
            ((b >> s) as u8 & ((1 << 5) - 1)) << 3
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

    fn run_frame(&mut self) -> Result<(), ExecutionError> {
        for _ in 0..500 {
            let mut trace = self.trace.as_ref().map(|_| Trace::new());

            self.machine.step(&mut trace)?;

            if let Some(trace) = trace {
                if let Err(e) = trace.write_to_file(self.trace.as_mut().unwrap()) {
                    self.command_output.push_str(&format!("Failed to write to trace file: {:?}\n", e));
                    return Ok(());
                }
            }

            // Postcondition so we can move past breakpoints
            if self.breakpoints.contains_key(&self.machine.pc) {
                self.execution_state = ExecutionState::Suspended;
                self.command_output.push_str(&format!("Hit breakpoint at x{:04X}\n", self.machine.pc));
                break;
            }
        }
        Ok(())
    }
}

impl eframe::App for CerealApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();

            self.command(ui, frame);

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

        if self.execution_state == ExecutionState::Running {
            self.run_frame().expect("No Execution Errors");
        }
        if self.execution_state == ExecutionState::Suspended {
            let cmds = self.script_commands.clone();
            self.script_commands.clear();
            for cmd in cmds {
                command::command(self, Some(frame), &cmd);
            }
        }

    }
}

// @Todo keep the machine around after an error
pub fn run(cli_options: Options) -> i16 {
    let mut machine = Machine::new();

    if let Some(dir) = cli_options.from_directory {
        std::env::set_current_dir(dir).expect("Cannot local directory\n");
    }

    let mut stdout = std::io::stdout();
    for path in &cli_options.input_paths {
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("There was an error opening file {:?}: {}", path, e);
                continue;
            }
        };
        let loader_trace = cli_options.loader_trace.then_some(&mut stdout as _); // unsizing coercion
        loader::load(&bytes, &mut machine, loader_trace).expect("Load failure");
    }

    let mut trace_file = cli_options.trace_path.as_ref().map(|path| {
        let file = std::fs::File::create(path).expect("Invalid file");
        std::io::BufWriter::new(file)
    });

    if !cli_options.headless {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1040.0, 860.0)),
            ..Default::default()
        };
        eframe::run_native(
            "Cereal Sim",
            options,
            Box::new(|_cc| Box::new(CerealApp::new(machine, cli_options.startup_script))),
        ).unwrap();

        0
    } else {
        let mut steps = 0;
        while machine.pc() != 0x80ff {
            steps += 1;
            match cli_options.step_cap {
                Some(cap) if steps > cap => panic!("exceeded step limit"),
                _ => {}
            }

            let mut trace = cli_options.trace_path.as_ref().map(|_| Trace::new());
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
