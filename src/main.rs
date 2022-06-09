// @Todo error handling, use span information
// @Todo code/data in the wrong section of memory
// @Todo when code/data is written on top of each other, issue an error (unless the actual program allows this)
//     - we were permitted to just override the old values when we had to write the loader for class, but i suspect a 
//       good assembler would warn you about this
// @Todo multiple files
//     - clean up main to be mostly UX stuff and have lib bundle functions
//     - function for front end / per file, function for back end / whole program

use std::path::PathBuf;

use cereal::lexer::Lexer;
use cereal::parser::Parser;

#[derive(clap::Parser)]
struct Args {
    path: PathBuf,
    #[clap(default_value = "output.obj")]
    output_path: PathBuf,
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
    
    // println!("SOURCE:");
    // println!("{}", string);

    for token in lexer {
        match token {
            Ok(token) => if errors.is_empty() { tokens.push(token) },
            Err(error) => errors.push(error),
        }
    }
    
    // println!();

    if !errors.is_empty() {
        for error in errors {
            println!("ERROR in file {:?}: {}", &args.path, error);
        }
        return;
    }

    /*
    println!("TOKENS:");
    for token in &tokens {
        println!("{:?}", token);
    }
    */
    
    let mut parser = Parser::new(tokens);
    
    let mut blocks = vec![];
    let mut errors = vec![];
    for block in &mut parser {
        match block {
            Ok(block) => if errors.is_empty() { blocks.push(block) },
            Err(error) => errors.push(error),
        }
    }
    
    // println!();

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
    
    // println!("PRINTED:");
    // cereal::parser::print_blocks(&blocks, &parser.constants).unwrap();
    
    if let Err(errors) = cereal::expand_psuedo_instructions(&mut blocks, &parser.constants) {
        for error in errors {
            println!("ERROR in file {:?}: {}", &args.path, error);
        }
        return;
    }
    
    // println!("EXPANDED:");
    // cereal::parser::print_blocks(&blocks, &parser.constants).unwrap();

    let labels = match cereal::patch(&mut blocks) {
        Ok(labels) => labels,
        Err(errors) => {
            for error in errors {
                println!("ERROR in file {:?}: {}", &args.path, error);
            }
            return;
        }
    };

    // println!("PATCHED:");
    // cereal::parser::print_blocks(&blocks, &parser.constants).unwrap();
    
    let bytes = cereal::write_object_code(&blocks, &labels);
    let mut file = std::fs::File::create("output.obj").expect("Could not create object file.");
    use std::io::Write;
    file.write_all(&bytes[..]).expect("Failed to write to object file.");
    
}
