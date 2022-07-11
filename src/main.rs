// @Todo error handling, use span information
// @Todo indicate error in process return value
// @Todo debug info: .loc and filename indices

use std::io::Write;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(clap::Parser)]
struct Args {
    #[clap(default_value = "output.obj", long, short)]
    output_path: PathBuf,
    #[clap(long, short = 'g')]
    debug_info: bool,
    input_paths: Vec<PathBuf>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    
    if args.input_paths.is_empty() {
        return;
    }

    let mut blocks = Vec::new();
    let mut constants = HashMap::new();
    let mut file_strings = Vec::new();
    
    for path in &args.input_paths {
        let string = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to open file '{:?}': {}", path, e);
                return;
            }
        };
        file_strings.push(string);
    }

    for i in 0..args.input_paths.len() {
        let path = &args.input_paths[i];
        let extension = if let Some(e) = path.extension() {
            e
        } else {
            println!("Cannot read file path extension for file '{:?}'", path);
            return;
        };

        if extension == "asm" {
            match cereal::assemble(path, &file_strings[i], &mut blocks, &mut constants) {
                Ok(()) => (),
                Err(()) => return,
            }
        } else if extension == "c" {
            match cereal::compile_c(path, &file_strings[i], &mut blocks, &mut constants) {
                Ok(()) => (),
                Err(()) => return,
            }
        } else {
            println!("ERROR: Only accepting .asm and .c files as inputs. Cannot compile '{:?}'", path);
            return;
        }
    }
    
    let bytes = match cereal::compile_and_link(&mut blocks, &constants, args.debug_info) {
        Ok(bytes) => bytes,
        Err(())=> return,
    };

    let mut file = match fs::File::create(&args.output_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Could not create object file '{:?}': {}.", &args.output_path, error);
            return;
        }
    };

    if let Err(error) = file.write_all(&bytes[..]) {
        println!("Failed to write to object file '{:?}': {}.", &args.output_path, error);
        return;
    }
}
