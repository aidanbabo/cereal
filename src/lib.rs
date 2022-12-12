use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub mod simulator;

mod asm_instruction;
mod assembler;
mod block;
mod c;
mod char_utils;
mod ir;
mod link;
mod printer;
mod span;

pub use asm_instruction::{InstructionType, InstructionWithLabel};
pub use span::{Span, Spannable, S};

const CODE_HEADER: u16 = 0xCADE;
const DATA_HEADER: u16 = 0xDADA;
const SYMBOL_HEADER: u16 = 0xC3B7;
const FILE_HEADER: u16 = 0xF17E;
const LINE_HEADER: u16 = 0x715E;

pub fn number_fits(i: i32, signed: bool, bits: u8) -> bool {
    let mut min = 0;
    let mut max = 1 << bits;
    if signed {
        let change = 1 << (bits - 1);
        min -= change;
        max -= change;
    }
    i >= min && i < max
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

    for (i, path) in options.input_paths.iter().enumerate() {
        let extension = if let Some(e) = path.extension() {
            e
        } else {
            println!("Cannot read file path extension for file '{:?}'", path);
            return Err(());
        };

        if extension == "asm" {
            match assembler::parse_string(path, &file_strings[i], &mut blocks, &mut constants) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
        } else if extension == "c" {
            match c::compile(path, &file_strings[i], &mut blocks, &mut constants) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
        } else {
            println!(
                "ERROR: Only accepting .asm and .c files as inputs. Cannot compile '{:?}'",
                path
            );
            return Err(());
        }
    }

    let bytes = match link::link(&mut blocks, &constants, options.debug_info) {
        Ok(bytes) => bytes,
        Err(()) => return Err(()),
    };

    let mut file = match File::create(&options.output_path) {
        Ok(file) => file,
        Err(error) => {
            println!(
                "Could not create object file '{:?}': {}.",
                &options.output_path, error
            );
            return Err(());
        }
    };

    if let Err(error) = file.write_all(&bytes[..]) {
        println!(
            "Failed to write to object file '{:?}': {}.",
            &options.output_path, error
        );
        return Err(());
    }

    Ok(())
}
