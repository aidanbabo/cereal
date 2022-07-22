use std::path::PathBuf;
use clap::Parser;

use cereal::simulator::{loader::load, Machine};

#[derive(Parser)]
struct Args {
    input_paths: Vec<PathBuf>,
}

fn main() {
    println!("This is the simulator!");
    
    let args = Args::parse();

    let mut machine = Machine::new();
    for path in &args.input_paths {
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("There was an error opening file {:?}: {}", path, e);
                continue;
            }
        };
        let _ = load(&bytes, &mut machine, Some(&mut std::io::stdout()));
    }
}
