use clap::Parser;
use std::path::PathBuf;

mod lexer;
use lexer::Lexer;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let string = match std::fs::read_to_string(args.path) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to open file {}", e);
            return;
        }
    };
    
    let lexer = Lexer::new(&string);
    
    println!("TOKENS:");
    for token in lexer {
        match token {
            Ok(token) => {
                println!("{:?}", token);
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }
    
    println!();
    println!("SOURCE:");
    println!("{}", string);
}
