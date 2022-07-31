use std::collections::HashMap;

use crate::{Block, BlockType, InstructionWithLabel};
use crate::c::parser::{TopLevel, TopLevelType, Procedure, Statement, StatementType, Return, Expression, ExpressionType, Literal, Binary, BinaryType};
use crate::insn;

pub fn generate<'c, 's>(ast: Vec<TopLevel<'s>>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    let mut ctx = CgContext::new(blocks, constants);
    for top_level in ast {
        ctx.generate_top_level(top_level);
    }
}

#[derive(Clone, Copy, Debug)]
enum Location {
    Register(i8),
    _Unused,
}

struct CgContext<'c, 's> {
    blocks: &'c mut Vec<Block<'s>>, 
    _constants: &'c mut HashMap<&'s str, i32>,
    available_registers: Vec<i8>,
    stack_slots_spilled: usize,
}

impl<'c, 's> CgContext<'c, 's> {
    fn new(blocks: &'c mut Vec<Block<'s>>, _constants: &'c mut HashMap<&'s str, i32>) -> Self {
        CgContext {
            blocks,
            _constants,
            available_registers: vec![0, 1, 2, 3, 4], // fp, sp, and ret reserved
            stack_slots_spilled: 0,
        }
    }
    
    fn instructions(&mut self) -> &mut Vec<InstructionWithLabel<'s>> {
        let len = self.blocks.len() - 1;
        let block = &mut self.blocks[len];
        if let BlockType::Code(i) = &mut block.ty {
            i
        } else {
            panic!("Tried to return from data block!");
        }
    }
    
    fn take_available_register(&mut self, not: Option<i8>) -> i8 {
        self.available_registers.pop().unwrap_or_else(|| {
            let reg = not.map_or(0, |not| (not + 1) % 4);
            self.instructions().push(insn::addi(6, 6, -1));
            self.instructions().push(insn::str(reg, 6, 0));
            self.stack_slots_spilled += 1;
            reg
        })
    }

    fn return_available_register(&mut self, reg: i8) {
        if self.stack_slots_spilled > 0 {
            self.instructions().push(insn::ldr(reg, 6, 0));
            self.instructions().push(insn::addi(6, 6, 1));
            self.stack_slots_spilled -= 1;
        } else {
            self.available_registers.push(reg)
        }
    }

    fn generate_top_level(&mut self, top_level: TopLevel<'s>) {
        match top_level.ty {
            TopLevelType::Procedure(procedure) => self.generate_procedure(procedure),
        }
    }

    fn generate_procedure(&mut self, procedure: Procedure<'s>) {
        assert!(procedure.args.is_empty());
    
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
    
        self.blocks.push(block);
    
        for statement in procedure.body {
            self.generate_statement(statement);
        }
    }

    fn generate_statement(&mut self, statement: Statement<'s>) {
        match statement.ty {
            StatementType::Return(ret) => self.generate_return(ret.t),
        }
    }

    fn generate_return(&mut self, ret: Return<'s>) {
        if let Some(expr) = ret.expr {
            self.generate_expression(expr, Location::Register(7));
        }
    
        let instructions = self.instructions();
    
        instructions.push(insn::addi(6, 6, 3));    // free space for return address, base pointer, and return value
        instructions.push(insn::str(7, 6, -1));    // store return value
        instructions.push(insn::ldr(7, 6, -2));    // restore return address
        instructions.push(insn::ldr(5, 6, -3));    // restore base pointer
        instructions.push(insn::jmpr(7));          // return
    }

    fn generate_expression(&mut self, expression: Expression<'s>, location: Location) {
        match expression.ty {
            ExpressionType::Literal(literal) => self.generate_literal(literal, location),
            ExpressionType::Binary(binary) => self.generate_binary(binary, location),
        }
    }

    fn generate_binary(&mut self, binary: Binary<'s>, location: Location) {
        let (dest, needs_move) = if let Location::Register(dest) = location {
            (dest, false)
        } else {
            (self.take_available_register(None), true)
        };
        
        self.generate_expression(*binary.left, Location::Register(dest));
        let reg = self.take_available_register(Some(dest));
        self.generate_expression(*binary.right, Location::Register(reg));
        match binary.ty {
            BinaryType::Add => self.instructions().push(insn::add(dest, dest, reg)),
            BinaryType::Sub => self.instructions().push(insn::sub(dest, dest, reg)),
            BinaryType::Mul => self.instructions().push(insn::mul(dest, dest, reg)),
            BinaryType::Div => self.instructions().push(insn::div(dest, dest, reg)),
            BinaryType::Mod => self.instructions().push(insn::mod_(dest, dest, reg)),
        }
        
        if needs_move {
            self.mov(location, Location::Register(dest));
            self.return_available_register(dest);
        }
        
        self.return_available_register(reg);
    }

    fn generate_literal(&mut self, literal: Literal<'s>, location: Location) {
        match literal {
            Literal::Numeric(n) => {
                let (reg, needs_move) = if let Location::Register(reg) = location {
                    (reg, false)
                } else {
                    (self.take_available_register(None), true)
                };
            
                let instructions = self.instructions();
                let num = *n;
                instructions.push(insn::konst(reg, num));
                if !crate::number_fits(num, true, 9) {
                    instructions.push(insn::hiconst(reg, num >> 8));
                }
            
                if needs_move {
                    self.mov(location, Location::Register(reg));
                    self.return_available_register(reg);
                }
            }
        }
    }

    fn mov(&mut self, dst: Location, src: Location) {
        use Location::*;
    
        match (dst, src) {
            (Register(dst), Register(src)) => {
                let instructions = self.instructions();
                instructions.push(insn::addi(dst, src, 0));
            }
            _ => unreachable!("Unsupported locations dst: {:?} and src: {:?}", dst, src),
        }
    }
}
