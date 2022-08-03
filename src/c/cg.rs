use std::collections::HashMap;

use crate::{Block, BlockType, InstructionWithLabel, Data};
use crate::c::ast::*;
use crate::insn;

pub fn generate<'c, 's>(ast: Vec<TopLevel<'s>>, blocks: &'c mut Vec<Block<'s>>, constants: &'c mut HashMap<&'s str, i32>) {
    let mut ctx = CgContext::new(blocks, constants);
    for top_level in ast {
        ctx.generate_top_level(top_level);
    }
}

#[derive(Clone, Copy, Debug)]
enum Location<'s> {
    Nowhere,
    Register(i8),
    Stack(i32),
    Label(&'s str),
}

struct CgContext<'c, 's> {
    blocks: &'c mut Vec<Block<'s>>, 
    _constants: &'c mut HashMap<&'s str, i32>,
    available_registers: Vec<i8>,
    stack_slots_spilled: usize,
    locals: HashMap<&'s str, Location<'s>>,
    globals: HashMap<&'s str, Location<'s>>,
    stack_space: Option<i32>,
}

impl<'c, 's> CgContext<'c, 's> {
    fn new(blocks: &'c mut Vec<Block<'s>>, _constants: &'c mut HashMap<&'s str, i32>) -> Self {
        CgContext {
            blocks,
            _constants,
            available_registers: vec![0, 1, 2, 3, 4], // fp, sp, and ret reserved
            stack_slots_spilled: 0,
            locals: HashMap::new(),
            globals: HashMap::new(),
            stack_space: None,
        }
    }
    
    fn instructions(&mut self) -> &mut Vec<InstructionWithLabel<'s>> {
        // @Cleanup This is insane person code to satisfy the borrow checker
        loop {
            if let Some(BlockType::Code(_)) = self.blocks.last_mut().map(|b| &b.ty) {
                break;
            } else {
                let block = Block {
                    addr: None,
                    aligned: false,
                    labels: Vec::new(),
                    ty: BlockType::Code(Vec::new()),
                };
            
                self.blocks.push(block);
            }
        }

        if let Some(BlockType::Code(i)) = self.blocks.last_mut().map(|b| &mut b.ty) {
            i
        } else { panic!() }
    }

    fn new_data(&mut self, label: &'s str) -> &mut Vec<Data<'s>> {
        let block = Block {
            addr: None,
            aligned: false,
            labels: vec![label],
            ty: BlockType::Data(Vec::new()),
        };
    
        self.blocks.push(block);
        if let Some(BlockType::Data(d)) = self.blocks.last_mut().map(|b| &mut b.ty) {
            d
        } else { panic!() }
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
    
    fn resolve(&self, name: &str) -> Location<'s> {
        let local = self.locals.get(name);
        if let Some(&l) = local {
            return l;
        }
        let global = self.globals.get(name);
        if let Some(&g) = global {
            return g;
        }
            
        panic!("No identifier named {}", name)
    }

    fn mov(&mut self, dst: Location<'s>, src: Location<'s>) {
        use Location::*;
        let (reg, needs_return) = match (dst, src) {
            (Register(dst), Register(src)) => {
                let instructions = self.instructions();
                instructions.push(insn::addi(dst, src, 0));
                return;
            }
            (Register(dst), _) => (dst, false),
            (_, Register(src)) => (src, false),
            _ => (self.take_available_register(None), true),
        };

        // src -> reg
        match src {
            Nowhere => panic!(),
            Register(_) => {},
            Stack(offset) => self.instructions().push(insn::ldr(reg, 5, offset)),
            Label(label) => {
                let addr = self.take_available_register(Some(reg));
                self.instructions().push(insn::lea(addr, label));
                self.instructions().push(insn::ldr(reg, addr, 0));
                self.return_available_register(addr);
            }
        }

        // reg -> dst
        match dst {
            Nowhere => panic!(),
            Register(_) => {},
            Stack(offset) => self.instructions().push(insn::str(reg, 5, offset)),
            Label(label) => {
                let addr = self.take_available_register(Some(reg));
                self.instructions().push(insn::lea(addr, label));
                self.instructions().push(insn::str(reg, addr, 0));
                self.return_available_register(addr);
            }
        }
        
        if needs_return {
            self.return_available_register(reg);
        }
    }

    fn generate_literal(&mut self, literal: Literal<'s>, location: Location<'s>) {
        let reg = match location {
            Location::Nowhere => return,
            Location::Register(reg) => reg,
            _ => unreachable!(),
        };

        match literal {
            Literal::Numeric(n) => {
                let instructions = self.instructions();
                let num = *n;
                instructions.push(insn::konst(reg, num));
                if !crate::number_fits(num, true, 9) {
                    instructions.push(insn::hiconst(reg, num >> 8));
                }
            }
        }
    }

    fn generate_unary(&mut self, unary: Unary<'s>, location: Location<'s>) {
        self.generate_expression(*unary.expr, location);

        let dest = match location {
            Location::Nowhere => return,
            Location::Register(reg) => reg,
            _ => unreachable!(),
        };

        match *unary.ty {
            // @Speed is there really not a one instruction way to get this sone
            UnaryType::Negate => {
                self.instructions().push(insn::not(dest, dest));
                self.instructions().push(insn::addi(dest, dest, 1));
            }
            UnaryType::BitNot => self.instructions().push(insn::not(dest, dest)),
            UnaryType::Plus => { /* no -op */ },
        }
    }

    fn generate_binary(&mut self, binary: Binary<'s>, location: Location<'s>) {
        self.generate_expression(*binary.left, location);
        let (dest, reg) = match location {
            Location::Nowhere => {
                self.generate_expression(*binary.right, location);
                return;
            }
            Location::Register(dest) => {
                (dest, self.take_available_register(Some(dest)))
            }
            _ => unreachable!(),
        };
        self.generate_expression(*binary.right, Location::Register(reg));

        match *binary.ty {
            BinaryType::Add => self.instructions().push(insn::add(dest, dest, reg)),
            BinaryType::Sub => self.instructions().push(insn::sub(dest, dest, reg)),
            BinaryType::Mul => self.instructions().push(insn::mul(dest, dest, reg)),
            BinaryType::Div => self.instructions().push(insn::div(dest, dest, reg)),
            BinaryType::Mod => self.instructions().push(insn::mod_(dest, dest, reg)),
            BinaryType::BitAnd => self.instructions().push(insn::and(dest, dest, reg)),
            BinaryType::BitXor => self.instructions().push(insn::xor(dest, dest, reg)),
            BinaryType::BitOr => self.instructions().push(insn::or(dest, dest, reg)),
        }
        
        self.return_available_register(reg);
    }
    
    fn generate_getter(&mut self, expr: Expression<'s>, location: Location<'s>) {
        match expr.ty {
            ExpressionType::Variable(name) => {
                let variable_location = self.resolve(name);
                self.mov(location, variable_location);
            },
            _ => panic!("Expression is not assignable!"),
        }
    }
    
    fn generate_setter(&mut self, expr: Expression<'s>) -> Location<'s> {
        match expr.ty {
            ExpressionType::Variable(name) => self.resolve(name),
            _ => panic!("Expression is not assignable!"),
        }
    }
    
    fn generate_assignment(&mut self, assignment: Assignment<'s>, location: Location<'s>) {
        let assign_to = self.generate_setter(*assignment.left);
        let (location, needs_return) = if let Location::Nowhere = location {
            (Location::Register(self.take_available_register(None)), true)
        } else {
            (location, false)
        };
        self.generate_expression(*assignment.right, location);
        match *assignment.ty {
            AssignmentType::Regular => (),
        }
        self.mov(assign_to, location);
        match location {
            Location::Register(reg) if needs_return => self.return_available_register(reg),
            _ => {},
        }
    }
    
    fn generate_comma(&mut self, comma: Comma<'s>, location: Location<'s>) {
        self.generate_expression(*comma.left, Location::Nowhere);
        self.generate_expression(*comma.right, location);
    }

    fn generate_expression(&mut self, expression: Expression<'s>, location: Location<'s>) {
        let (dest, needs_move) = match location {
            Location::Nowhere => (-1, false),
            Location::Register(dest) => (dest, false),
            Location::Stack(_) | Location::Label(_) => (self.take_available_register(None), true),
        };
        
        match expression.ty {
            ExpressionType::Literal(literal) => self.generate_literal(literal, location),
            ExpressionType::Unary(unary) => self.generate_unary(unary, location),
            ExpressionType::Binary(binary) => self.generate_binary(binary, location),
            ExpressionType::Assignment(assignment) => self.generate_assignment(assignment, location),
            ExpressionType::Variable(_) => self.generate_getter(expression, location),
            ExpressionType::Comma(comma) => self.generate_comma(comma, location)
        }

        if needs_move {
            self.mov(location, Location::Register(dest));
            self.return_available_register(dest);
        }
        
    }

    fn generate_return(&mut self, ret: Return<'s>) {
        if let Some(expr) = ret.expr {
            self.generate_expression(expr, Location::Register(7));
        }
    
        let stack_space = self.stack_space.expect("Didn't calculate stack space for procedure.");
        let instructions = self.instructions();
        
        if stack_space != 0 {
            instructions.push(insn::addi(6, 6, -stack_space));
        }
    
        instructions.push(insn::addi(6, 6, 3));    // free space for return address, base pointer, and return value
        instructions.push(insn::str(7, 6, -1));    // store return value
        instructions.push(insn::ldr(7, 6, -2));    // restore return address
        instructions.push(insn::ldr(5, 6, -3));    // restore base pointer
        instructions.push(insn::jmpr(7));          // return
    }

    fn generate_statement(&mut self, statement: Statement<'s>) {
        match statement.ty {
            StatementType::Return(ret) => self.generate_return(ret.t),
            StatementType::Expression(expr) => self.generate_expression(expr, Location::Nowhere),
        }
    }

    fn generate_procedure(&mut self, procedure: Procedure<'s>) {
        assert!(procedure.args.is_empty(), "only working with nullary procedures");
        
        self.locals.clear();
        let mut stack_index = -1;
        for decl in procedure.declarations {
            for (_, name) in decl.names {
                self.locals.insert(*name, Location::Stack(stack_index));
                stack_index -= 1;
            }
        }
        let stack_space = stack_index + 1;
        self.stack_space = Some(stack_space);
    
        let mut instructions = Vec::new();
    
        // prologue
        instructions.push(insn::str(7, 6, -2));    // save return address
        instructions.push(insn::str(5, 6, -3));    // save frame pointer
        instructions.push(insn::addi(6, 6, -3));   // update stack pointer
        instructions.push(insn::addi(5, 6, 0));    // set frame pointer to new stack pointer
        if stack_space != 0 {
            instructions.push(insn::addi(6, 6, stack_space));
        }
    
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
    
    fn generate_global(&mut self, global: GlobalVariable<'s>) {
        for (_, name) in global.names {
            let data = self.new_data(*name);
            data.push(Data::Word(0));
            self.globals.insert(*name, Location::Label(*name));
        }
    }

    fn generate_top_level(&mut self, top_level: TopLevel<'s>) {
        match top_level.ty {
            TopLevelType::Procedure(procedure) => self.generate_procedure(procedure),
            TopLevelType::Variable(global) => self.generate_global(global),
        }
    }
}
