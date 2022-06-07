// @Todo error handling, use span information

use std::path::PathBuf;

use cereal::lexer::Lexer;
use cereal::parser::Parser;

#[derive(clap::Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    let string = match std::fs::read_to_string(&args.path) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to open file {}", e);
            return;
        }
    };
    
    let lexer = Lexer::new(&string);
    
    let mut tokens = vec![];
    let mut errors = vec![];
    
    println!("SOURCE:");
    println!("{}", string);

    for token in lexer {
        match token {
            Ok(token) => if errors.is_empty() { tokens.push(token) },
            Err(error) => errors.push(error),
        }
    }
    
    println!();

    if !errors.is_empty() {
        for error in errors {
            println!("ERROR in file {:?}: {}", &args.path, error);
        }
        return;
    }

    println!("TOKENS:");
    for token in &tokens {
        println!("{:?}", token);
    }
    
    let parser = Parser::new(tokens);
    
    let mut blocks = vec![];
    let mut errors = vec![];
    for block in parser {
        match block {
            Ok(block) => if errors.is_empty() { blocks.push(block) },
            Err(error) => errors.push(error),
        }
    }
    
    println!();

    if !errors.is_empty() {
        for error in errors {
            println!("ERROR in file {:?}: {}", &args.path, error);
        }
        return;
    }
    
    /*
    println!("BLOCKS:");
    for block in &blocks {
        println!("{:?}", block);
    }
    */
    
}
