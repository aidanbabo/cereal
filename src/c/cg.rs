use std::collections::HashMap;

use crate::{Block, BlockType};
use crate::c::parser::{TopLevel, TopLevelType, Procedure, Statement, StatementType, Return, Expression, ExpressionType, Literal};
use crate::insn;

pub fn generate<'c, 's>(ast: Vec<TopLevel<'s>>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    for top_level in ast {
        generate_top_level(top_level, blocks, constants);
    }
}

pub fn generate_top_level<'c, 's>(top_level: TopLevel<'s>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    match top_level.ty {
        TopLevelType::Procedure(procedure) => generate_procedure(procedure, blocks, constants),
    }
}

pub fn generate_procedure<'c, 's>(procedure: Procedure<'s>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    assert!(procedure._args.is_empty());
    
    let mut instructions = Vec::new();
    
    // prologue
    instructions.push(insn::str(7, 6, -2));    // save return address
    instructions.push(insn::str(5, 6, -3));    // save frame pointer
    instructions.push(insn::addi(6, 6, -3));   // update stack pointer
    instructions.push(insn::addi(5, 6, 0));    // set frame pointer to new stack pointer
    
    let block = Block {
        addr: None,
        aligned: true,
        labels: vec![*procedure.name],
        ty: BlockType::Code(instructions),
    };
    
    blocks.push(block);
    
    for statement in procedure.body {
        generate_statement(statement, blocks, constants);
    }
}
pub fn generate_statement<'c, 's>(statement: Statement<'s>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    match statement.ty {
        StatementType::Return(ret) => generate_return(ret.t, blocks, constants),
    }
}

pub fn generate_return<'c, 's>(ret: Return<'s>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    if let Some(expr) = ret.expr {
        generate_expression(expr, blocks, constants);
    }
    
    // @Duplication
    let len = blocks.len() - 1;
    let block = &mut blocks[len];
    let instructions = if let BlockType::Code(i) = &mut block.ty {
        i
    } else {
        panic!("Tried to return from data block!");
    };
    
    instructions.push(insn::addi(6, 6, 3));    // free space for return address, base pointer, and return value
    instructions.push(insn::str(7, 6, -1));    // store return value
    instructions.push(insn::ldr(7, 6, -2));    // restore return address
    instructions.push(insn::ldr(5, 6, -3));    // restore base pointer
    instructions.push(insn::jmpr(7));          // return
}

// @Todo currently all expressions put the result in r7
pub fn generate_expression<'c, 's>(expression: Expression<'s>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    match expression.ty {
        ExpressionType::Literal(literal) => generate_literal(literal, blocks, constants),
    }
}
pub fn generate_literal<'c, 's>(literal: Literal<'s>, blocks: &'c mut Vec<Block<'s>>, _constants: &'c mut HashMap<&'s str, i32>) {
    match literal {
        Literal::Numeric(n) => {
            let len = blocks.len() - 1;
            let block = &mut blocks[len];
            let instructions = if let BlockType::Code(i) = &mut block.ty {
                i
            } else {
                panic!("Tried to add numeric literal to data block!");
            };
            
            let num = *n;
            instructions.push(insn::konst(7, num));
            if !crate::number_fits(num, true, 9) {
                instructions.push(insn::hiconst(7, num >> 8));
            }
        }
    }
}
