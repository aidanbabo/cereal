use std::path::Path;
use std::collections::HashMap;
use crate::Block;

mod lexer;
mod parser;
mod cg;
use lexer::Lexer;
use parser::Parser;

type Error = String;

pub fn compile<'container, 'source>(
    filename: &Path, 
    string: &'source str, 
    blocks: &'container mut Vec<Block<'source>>, 
    constants: &'container mut HashMap<&'source str, i32>
) -> Result<(), ()> {

    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    for token in Lexer::new(string) {
        match token {
            Ok(token) => if errors.is_empty() { tokens.push(token); },
            Err(error) => errors.push(error),
        }
    }
    
    if errors.is_empty() {
        /*
        for token in &tokens {
            println!("{:?}", token);
        }
        */
    } else {
        println!("Errors in file {:?}", filename);
        for error in errors {
            println!("{}", error);
        }
        return Err(());
    }
    
    let mut parser = Parser::new(tokens);
    let mut ast = Vec::new();
    match parser.fill(&mut ast) {
        Ok(()) => {},// println!("{:#?}", ast),
        Err(error) => {
            println!("Errors in file {:?}", filename);
            println!("{}", error);
            return Err(());
        }
    }
    
    cg::generate(ast, blocks, constants);
    
    Ok(())
}
