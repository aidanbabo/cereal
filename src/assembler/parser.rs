use std::collections::HashMap;

use super::lexer::{Token, TokenType, DirectiveType, Identifier, LiteralType};
use crate::{InstructionType, Block, BlockType, Data, InstructionWithLabel, Operand, Reg};
use crate::{instruction_operands, number_fits};

#[derive(PartialEq)]
enum Section {
    Code,
    Data,
}

pub struct Parser<'a, 'b> {
    tokens: Vec<Token<'a>>,
    next_token: usize,
    section: Section,
    in_os_mode: bool,
    pub constants: &'b mut HashMap<&'a str, i32>
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(tokens: Vec<Token<'a>>, constants: &'b mut HashMap<&'a str, i32>) -> Self {
        Parser {
            tokens,
            next_token: 0,
            section: Section::Code,
            in_os_mode: false,
            constants,
        }
    }
    
    fn peek(&mut self) -> Option<&Token<'a>> {
        self.tokens.get(self.next_token)
    }
    
    fn consume(&mut self) -> Option<Token<'a>> {
        let token = self.tokens.get(self.next_token).cloned();
        self.next_token += 1;
        token
    }
    
    fn new_block(&self) -> Block<'a> {
        Block { 
            addr: None, 
            labels: vec![],
            aligned: false,
            ty: if self.section == Section::Code { BlockType::Code(vec![]) } else { BlockType::Data(vec![]) },
        }
    }
    
    fn get_directive_arg(&mut self, directive_type: DirectiveType) -> Result<Token<'a>, String> {
        let next = self.consume();
        match next {
            Some(n) => Ok(n),
            None => return Err(directive_error(directive_type, None)),
        }
    }

    fn parse_directives(&mut self, block: &mut Block<'a>) -> Result<(), String> {
        while let Some(peek) = self.peek() {

            let dt = if let TokenType::Directive(dt) = peek.ty {
                dt
            } else {
                break
            };

            match dt {
                DirectiveType::Os => {
                    self.in_os_mode = true;
                    self.consume();
                }
                DirectiveType::Falign => {
                    block.aligned = true;
                    self.consume();
                }
                DirectiveType::Code => {
                    self.section = Section::Code;
                    block.ty = BlockType::Code(vec![]);
                    self.consume();
                }
                DirectiveType::Data => {
                    self.section = Section::Data;
                    block.ty = BlockType::Data(vec![]);
                    self.consume();
                }
                DirectiveType::Addr => {
                    let _directive = self.consume();
                    let addr = self.get_directive_arg(dt)?;
                    
                    let addr = match addr.ty {
                        TokenType::Literal(LiteralType::Unsigned(val)) => val,
                        TokenType::Identifier(Identifier::Hex(val)) => val,
                        _ => return Err(directive_error(dt, Some(&addr))),
                    };
                    
                    block.addr = Some(addr);
                }
                _ => break,
            }
        }
        
        Ok(())
    }
    
    fn parse_labels(&mut self, block: &mut Block<'a>) -> Result<(), String> {
        while let Some(peek) = self.peek() {

            let label = if let TokenType::Identifier(_) = peek.ty {
                self.consume().unwrap()
            } else {
                return Ok(());
            };
            
            block.labels.push(label.chars);

            let next = if let Some(p) = self.peek() {
                p
            } else {
                return Ok(());
            };
            
            match next.ty {
                TokenType::Colon => {
                    self.consume();
                    continue;
                }
                TokenType::Directive(ty @ (DirectiveType::Const | DirectiveType::Uconst)) => {

                    let _directive = self.consume().unwrap();
                    let num = self.get_directive_arg(ty)?;

                    let val = match num.ty {
                        TokenType::Literal(LiteralType::Signed(val))   if ty == DirectiveType::Const  => val as i32,
                        TokenType::Literal(LiteralType::Unsigned(val)) if ty == DirectiveType::Uconst => val as i32,
                        TokenType::Identifier(Identifier::Hex(val))    if ty == DirectiveType::Uconst => val as i32,
                        _ => return Err(directive_error(ty, Some(&num))),
                    };
                    
                    if let Some(old) = self.constants.insert(label.chars, val) {
                        return Err(format!("Label '{}' is already associated with value '{}'", label.chars, old));
                    }
                }
                _ => {},
            }
            
        }
        
        Ok(())
    }
    
    fn parse_data(&mut self, data: &mut Vec<Data<'a>>) -> Result<(), String> {
        while let Some(peek) = self.peek() {

            let dt = if let TokenType::Directive(dt) = peek.ty {
                dt
            } else {
                break
            };
        
            match dt {
                DirectiveType::Blkw => {
                    let _blkw = self.consume().unwrap();
                    let num = self.get_directive_arg(dt)?;

                    let size = match num.ty {
                        TokenType::Literal(LiteralType::Unsigned(val)) => val,
                        TokenType::Identifier(Identifier::Hex(val)) => val,
                        _ => return Err(directive_error(dt, Some(&num))),
                    };
                    
                    data.push(Data::Block(size));
                }
                
                DirectiveType::Fill => {
                    let _fill = self.consume().unwrap();
                    let num = self.get_directive_arg(dt)?;

                    let val = match num.ty {
                        TokenType::Identifier(Identifier::Hex(val)) => val as i16,
                        TokenType::Literal(LiteralType::Signed(val)) => val as i16,
                        TokenType::Literal(LiteralType::Unsigned(val)) => {
                            if val as i32 > i16::MAX as i32 {
                                return Err(format!("Literal '{}' after .fill directive is too big to fit in a signed 16-bit number.", num.chars));
                            }
                            val as i16
                        }
                        _ => return Err(directive_error(dt, Some(&num))),
                    };
                    
                    data.push(Data::Word(val));
                }
                
                DirectiveType::Stringz => {
                    let _stringz = self.consume().unwrap();
                    let string = self.get_directive_arg(dt)?;
                    
                    let contents = if let TokenType::String(s) = string.ty {
                        s
                    } else {
                        return Err(directive_error(dt, Some(&string)));
                    };

                    data.push(Data::Stringz(contents));
                }
                _ => break,
            }
        }

        return Ok(());
        
    }
    
    fn parse_instruction(&mut self, ty: InstructionType, specs: &[Operand]) -> Result<InstructionWithLabel<'a>, String> {

        let mut instruction = InstructionWithLabel { ty, rd: -1, rt: -1, rs: -1, immediate: i32::MAX, label: None };
        for (i, &spec) in specs.into_iter().enumerate() {
            
            // optional comma
            if let Some(c) = self.peek() {
                if let TokenType::Comma = c.ty {
                    self.consume();
                }
            }

            let token = if let Some(t) = self.consume() {
                t
            } else {
                return Err(format!("Instruction '{}' expects a {} as its {} operand, but found end of file.", ty, op_err_str(spec), number(i + 1)));
            };
                    
            match spec {
                Operand::Register { register } => {
                    let r = if let TokenType::Identifier(Identifier::Register(r)) = token.ty {
                        r
                    } else {
                        return Err(error(&instruction, i, spec, token.chars));
                    };
                    
                    use Reg::*;
                    match register {
                        Rd => instruction.rd = r as i8,
                        Rs => instruction.rs = r as i8,
                        Rt => instruction.rt = r as i8,
                    }
                },
                Operand::Label => {
                    if let TokenType::Identifier(_) = token.ty {
                    } else {
                        return Err(error(&instruction, i, spec, token.chars));
                    };
                    
                    instruction.label = Some(token.chars);
                },
                Operand::Immediate { signed, bits } => immediate(i, spec, token, signed, bits, &mut instruction)?,
                Operand::RegisterOrImmediate { register, signed, bits } => {

                    if let TokenType::Identifier(Identifier::Register(r)) = token.ty {
                        use Reg::*;
                        match register {
                            Rd => instruction.rd = r as i8,
                            Rs => instruction.rs = r as i8,
                            Rt => instruction.rt = r as i8,
                        }
                        continue;
                    }
                    immediate(i, spec, token, signed, bits, &mut instruction)?
                },
            }
        }

        return Ok(instruction);
        
        fn error(instruction: &InstructionWithLabel, i: usize, spec: Operand, chars: &str) -> String {
            format!("Instruction '{}' expects a {} as its {} operand, but found '{}'.", instruction.ty, op_err_str(spec), number(i + 1), chars)
        }

        fn number(i: usize) -> &'static str {
            match i {
                1 => "first",
                2 => "second",
                3 => "third",
                _ => panic!("internal error: passed {} to number", i),
            }
        }

        fn op_err_str(op: Operand) -> &'static str {
            use Operand::*;
            
            match op {
                Register { ..} => "register",
                Label => "label",
                Immediate { .. } => "immediate value",
                RegisterOrImmediate { .. } => "register or immediate value",
            }
        }

        fn immediate(i: usize, spec: Operand, token: Token<'_>, signed: bool, bits: u8, instruction: &mut InstructionWithLabel) -> Result<(), String> {
            let lit_value = match token.ty {
                TokenType::Literal(lt) => lt.to_i32(),
                TokenType::Identifier(Identifier::Hex(h)) => h as i32,
                _ => return Err(error(instruction, i, spec, token.chars)),
            };
            
            if !signed && lit_value < 0 {
                return Err(format!("Instruction '{}' expects an unsigned immediate value as its {} operand, but '{}' is signed.", instruction.ty, number(i + 1), token.chars));
            }
            
            if !number_fits(lit_value, signed, bits) {
                let signedness = if signed {
                    "a signed"
                } else {
                    "an unsigned"
                };
                return Err(format!("Instruction '{}' expects {} {}-bit immediate value as its {} operand, but '{}' cannot fit into {} bits.", instruction.ty, signedness, bits, number(i + 1), token.chars, bits));
            }
            
            instruction.immediate = lit_value;
            Ok(())
        }
    }
    
    fn parse_instructions(&mut self, instructions: &mut Vec<InstructionWithLabel<'a>>) -> Result<(), String> {
        while let Some(i) = self.peek() {

            let instruction_type = if let TokenType::Instruction(it) = i.ty {
                it
            } else {
                break
            };
            
            self.consume();
            
            let ops = &mut [Operand::Label; 3];
            let ops = instruction_operands(instruction_type, &mut ops[..]);
            
            let instruction = self.parse_instruction(instruction_type, ops);
            
            let instruction = match instruction {
                Ok(i) => i,
                Err(e) => return Err(e),
            };
            
            instructions.push(instruction);
        }
        Ok(())
    }

    fn next_block(&mut self) -> Option<Result<Block<'a>, String>> {
        if self.peek().is_none() {
            return None;
        }
        
        let mut block = self.new_block();
        if let Err(e) = self.parse_directives(&mut block) {
            return Some(Err(e));
        }

        if let Err(e) = self.parse_labels(&mut block) {
            return Some(Err(e));
        }
        
        let ret = match block.ty {
            BlockType::Code(ref mut instructions) => self.parse_instructions(instructions),
            BlockType::Data(ref mut data) => self.parse_data(data),
        };
        
        if let Err(e) = ret {
            return Some(Err(e));
        }
        
        if block.is_empty() {
            Some(Err(format!("Empty block!")))
        } else {
            Some(Ok(block))
        }
        
    }
}

impl<'a, 'b> Iterator for Parser<'a, 'b> {
    type Item = Result<Block<'a>, String>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next_block()
    }
}

fn directive_error(directive: DirectiveType, found: Option<&Token>) -> String {
    use DirectiveType::*;
    let (es, ds) = match directive {
        Addr => ("an unsigned integer", "addr"),
        Const => ("a signed integer", "const"),
        Uconst => ("an unsigned integer", "uconst"),
        Fill => ("a signed integer", "fill"),
        Blkw => ("an unsigned integer", "blkw"),
        Stringz => ("a string", "stringz"),
        _ => unreachable!("directive {:?} has no arguments", directive),
    };
    match found {
        Some(f) => format!("Expected {} after .{} directive, but found '{}'.", es, ds, f.chars),
        None =>    format!("Expected {} after .{} directive, but found end of file.", es, ds),
    }
}
