use std::collections::HashMap;
use std::path::Path;

use crate::Block;

mod lexer;
mod parser;
use lexer::Lexer;
use parser::Parser;

pub fn parse_string<'container, 'source>(
    filename: &Path,
    string: &'source str,
    blocks: &'container mut Vec<Block<'source>>,
    constants: &'container mut HashMap<&'source str, i32>,
) -> Result<(), ()> {
    let lexer = Lexer::new(string);

    let mut tokens = vec![];
    let mut errors = vec![];

    // println!("SOURCE:");
    // println!("{}", string);

    for token in lexer {
        match token {
            Ok(token) => {
                if errors.is_empty() {
                    tokens.push(token)
                }
            }
            Err(error) => errors.push(error),
        }
    }

    // println!();
    if !errors.is_empty() {
        for (line, error) in errors {
            println!("ERROR in file {:?} on line {}: {}", filename, line, error);
        }
        return Err(());
    }

    /*
    println!("TOKENS:");
    for token in &tokens {
        println!("{:?}", token);
    }
    */

    let parser = Parser::new(tokens, constants);

    let mut errors = vec![];
    for block in parser {
        match block {
            Ok(block) => {
                if errors.is_empty() {
                    blocks.push(block)
                }
            }
            Err(error) => {
                errors.push(error);
                break;
            }
        }
    }

    // println!();

    if !errors.is_empty() {
        for error in errors {
            println!("ERROR in file {:?}: {}", filename, error);
        }
        return Err(());
    }

    /*
    println!("BLOCKS:");
    for block in &blocks {
        println!("{:?}", block);
    }
    */

    Ok(())
}
