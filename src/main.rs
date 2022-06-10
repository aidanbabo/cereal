// @Todo error handling, use span information
// @Todo code/data in the wrong section of memory
// @Todo when code/data is written on top of each other, issue an error (unless the actual program allows this)
//     - we were permitted to just override the old values when we had to write the loader for class, but i suspect a 
//       good assembler would warn you about this
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
    input_paths: Vec<PathBuf>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

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
        match cereal::parse_string(&args.input_paths[i], &file_strings[i], &mut blocks, &mut constants) {
            Ok(()) => (),
            Err(()) => return,
        }
    }
    
    let bytes = match cereal::compile_and_link(&mut blocks, &constants) {
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
