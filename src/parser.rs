use std::borrow::Cow;
use std::collections::HashMap;

use crate::InstructionType;
use crate::lexer::{Token, TokenType, DirectiveType, Identifier, LiteralType};

#[derive(Debug)]
struct InstructionWithLabel<'a> {
    ty: InstructionType,
    rd: u8,
    rs: u8,
    rt: u8,
    immediate: i32,
    label: Option<&'a str>,
}

#[derive(Debug)]
enum Data<'a> {
    Block(u16),
    Stringz(Cow<'a, str>),
    Word(i16),
}

#[derive(Debug)]
enum BlockType<'a> {
    Code(Vec<InstructionWithLabel<'a>>),
    Data(Vec<Data<'a>>),
}

#[derive(Debug)]
pub struct Block<'a> {
    addr: Option<u16>,
    aligned: bool,
    labels: Vec<&'a str>,
    ty: BlockType<'a>,
}

#[derive(PartialEq)]
enum Section {
    Code,
    Data,
}

enum Reg {
    Rd,
    Rs,
    Rt,
}

enum Operand {
    Register {
        register: Reg
    },
    Immediate {
        signed: bool,
        bits: u8,
    },
    Label,
    RegisterOrImmediate {
        register: Reg,
        signed: bool,
        bits: u8,
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    next_token: usize,
    section: Section,
    in_os_mode: bool,
    constants: HashMap<&'a str, LiteralType>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            next_token: 0,
            section: Section::Code,
            in_os_mode: false,
            constants: Default::default(),
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

    fn parse_directives(&mut self, block: &mut Block<'a>) -> Result<(), String> {
        while let Some(peek) = self.peek() {
            // println!("DIRECTIVE LOOP {:?}", peek);

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
                    self.consume();
                    let addr = match self.consume() {
                        Some(a) => a,
                        None => return Err("Expected an address after a .addr directive.".to_string()),
                    };
                    
                    let addr = match addr.ty {
                        TokenType::Literal(LiteralType::Unsigned(val)) => val,
                        TokenType::Identifier(Identifier::Hex(val)) => val,
                        _ => return Err(format!("Expected an unsigned literal after .addr directive, but found '{}'.", addr.chars)),
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
            // println!("LABEL LOOP {:?}", peek);

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
                TokenType::Directive(DirectiveType::Const) => {
                    let _directive = self.consume().unwrap();
                    let num = if let Some(n) = self.consume() {
                        n
                    } else {
                        return Err("Expected a signed constant literal after .const directive, but found end of file.".to_string());
                    };

                    let val = match num.ty {
                        TokenType::Literal(LiteralType::Signed(val)) => val,
                        _ => return Err(format!("Expected a signed constant literal after .const directive, but found '{}'.", num.chars)),
                    };
                    
                    if let Some(old) = self.constants.insert(label.chars, LiteralType::Signed(val)) {
                        let val = match old {
                            LiteralType::Signed(val) => val as i32,
                            LiteralType::Unsigned(val) => val as i32,
                        };
                        return Err(format!("Label '{}' is already associated with value '{}'", label.chars, val));
                    }
                }
                TokenType::Directive(DirectiveType::Uconst) => {
                    let _directive = self.consume().unwrap();
                    let num = if let Some(n) = self.consume() {
                        n
                    } else {
                        return Err("Expected an unsigned constant literal after .uconst directive, but found end of file.".to_string());
                    };

                    let val = match num.ty {
                        TokenType::Literal(LiteralType::Unsigned(val)) => val,
                        TokenType::Identifier(Identifier::Hex(val)) => val,
                        _ => return Err(format!("Expected an unsigned constant literal after .uconst directive, but found '{}'.", num.chars)),
                    };

                    if let Some(old) = self.constants.insert(label.chars, LiteralType::Unsigned(val)) {
                        let val = match old {
                            LiteralType::Signed(val) => val as i32,
                            LiteralType::Unsigned(val) => val as i32,
                        };
                        return Err(format!("Label '{}' is already associated with value '{}'", label.chars, val));
                    }
                }
                _ => {},
            }
            
        }
        
        Ok(())
    }
    
    fn parse_data(&mut self, data: &mut Vec<Data<'a>>) -> Result<(), String> {
        while let Some(peek) = self.peek() {
            // println!("DATA LOOP {:?}", peek);

            let dt = if let TokenType::Directive(dt) = peek.ty {
                dt
            } else {
                break
            };
        
            match dt {
                DirectiveType::Blkw => {
                    let _blkw = self.consume().unwrap();
                    let num = if let Some(n) = self.consume() {
                        n
                    } else {
                        return Err("Expected an unsigned constant literal after .blkw directive, but found end of file.".to_string());
                    };

                    let size = match num.ty {
                        TokenType::Literal(LiteralType::Unsigned(val)) => val,
                        TokenType::Identifier(Identifier::Hex(val)) => val,
                        _ => return Err(format!("Expected an unsigned constant literal after .blkw directive, but found '{}'.", num.chars)),
                    };
                    
                    data.push(Data::Block(size));
                }
                
                DirectiveType::Fill => {
                    let _fill = self.consume().unwrap();
                    let num = if let Some(n) = self.consume() {
                        n
                    } else {
                        return Err("Expected a signed constant literal after .fill directive, but found end of file.".to_string());
                    };

                    let val = match num.ty {
                        TokenType::Identifier(Identifier::Hex(val)) => val as i16,
                        TokenType::Literal(LiteralType::Signed(val)) => val as i16,
                        TokenType::Literal(LiteralType::Unsigned(val)) => {
                            if val as i32 > i16::MAX as i32 {
                                return Err("Literal '{}' after .fill directive is too big to fit in a signed 16-bit number.".to_string());
                            }
                            val as i16
                        }
                        _ => return Err(format!("Expected a signed constant literal after .fill directive, but found '{}'.", num.chars)),
                    };
                    
                    data.push(Data::Word(val));
                }
                
                DirectiveType::Stringz => {
                    let _stringz = self.consume().unwrap();
                    let string = if let Some(s) = self.consume() {
                        s
                    } else {
                        return Err("Expected a string after .stringz directive, but found end of file.".to_string());
                    };
                    
                    let contents = if let TokenType::String(s) = string.ty {
                        s
                    } else {
                        return Err(format!("Expected a string after .stringz directive, but found '{}'.", string.chars));
                    };

                    data.push(Data::Stringz(contents));
                }
                _ => break,
            }
        }

        Ok(())
    }
    
    fn parse_instruction(&mut self, ty: InstructionType, specs: &[Operand]) -> Result<InstructionWithLabel<'a>, String> {

        let mut instruction = InstructionWithLabel { ty, rd: 0, rt: 0, rs: 0, immediate: 0, label: None };
        for (i, spec) in specs.into_iter().enumerate() {
            
            if i != 0 {
                let c = self.consume();
                if c.is_none() || c.unwrap().ty != TokenType::Comma {
                    return Err(format!("Missing comma after {} operand", number(i)));
                }
            }

            let token = if let Some(t) = self.consume() {
                t
            } else {
                return Err(format!("Instruction '{}' expects a {} as its {} operand, but found nothing.", ty, op_err_str(spec), number(i + 1)));
            };
                    
            match spec {
                Operand::Register { register } => {
                    let r = if let TokenType::Identifier(Identifier::Register(r)) = token.ty {
                        r
                    } else {
                        return Err(format!("Instruction '{}' expects a {} as its {} operand, but found '{}'.", ty, op_err_str(spec), number(i + 1), token.chars));
                    };
                    
                    use Reg::*;
                    match register {
                        Rd => instruction.rd = r,
                        Rs => instruction.rs = r,
                        Rt => instruction.rt = r,
                    }
                },
                Operand::Label => {
                    if let TokenType::Identifier(_) = token.ty {
                    } else {
                        return Err(format!("Instruction '{}' expects a {} as its {} operand, but found '{}'.", ty, op_err_str(spec), number(i + 1), token.chars));
                    };
                    
                    instruction.label = Some(token.chars);
                },
                Operand::Immediate { signed, bits } => {
                    let lit_value = match token.ty {
                        TokenType::Literal(lt) => lt.to_i32(),
                        TokenType::Identifier(Identifier::Hex(h)) => h as i32,
                        _ => return Err(format!("Instruction '{}' expects a {} as its {} operand, but found '{}'.", ty, op_err_str(spec), number(i + 1), token.chars)),
                    };
                    
                    if !signed && lit_value < 0 {
                        return Err(format!("Instruction '{}' expects an unsigned immediate value as its {} operand, but '{}' is signed.", ty, number(i + 1), token.chars));
                    }
                    
                    if !number_fits(lit_value, *signed, *bits) {
                        let signedness = if *signed {
                            "a signed"
                        } else {
                            "an unsigned"
                        };
                        return Err(format!("Instruction '{}' expects {} {}-bit immediate value as its {} operand, but '{}' cannot fit into {} bits.", ty, signedness, bits, number(i + 1), token.chars, bits));
                    }
                    
                    instruction.immediate = lit_value;
                    
                },
                Operand::RegisterOrImmediate { register, signed, bits } => {

                    // @Todo cutnpaste -> functions
                    
                    if let TokenType::Identifier(Identifier::Register(r)) = token.ty {
                        use Reg::*;
                        match register {
                            Rd => instruction.rd = r,
                            Rs => instruction.rs = r,
                            Rt => instruction.rt = r,
                        }
                        continue;
                    }
                    let lit_value = match token.ty {
                        TokenType::Literal(lit_type) => lit_type.to_i32(),
                        TokenType::Identifier(Identifier::Hex(h)) => h as i32,
                        _ => return Err(format!("Instruction '{}' expects a {} as its {} operand, but found '{}'.", ty, op_err_str(spec), number(i + 1), token.chars)),
                    };

                    if !signed && lit_value < 0 {
                        return Err(format!("Instruction '{}' expects an unsigned immediate value as its {} operand, but '{}' is signed.", ty, number(i + 1), token.chars));
                    }
                    
                    if !number_fits(lit_value, *signed, *bits) {
                        let signedness = if *signed {
                            "a signed"
                        } else {
                            "an unsigned"
                        };
                        return Err(format!("Instruction '{}' expects {} {}-bit immediate value as its {} operand, but '{}' cannot fit into {} bits.", ty, signedness, bits, number(i + 1), token.chars, bits));
                    }

                    instruction.immediate = lit_value;
                },
            }
        }
        Ok(instruction)
    }
    
    fn parse_instructions(&mut self, instructions: &mut Vec<InstructionWithLabel<'a>>) -> Result<(), String> {
        while let Some(i) = self.peek() {
            // println!("INSTRUCTION LOOP {:?}", i);

            let instruction_type = if let TokenType::Instruction(it) = i.ty {
                it
            } else {
                break
            };
            
            self.consume();
            
            use InstructionType::*;
            use Operand::*;
            let specs: &[Operand] = match instruction_type {
                Nop | Ret | Rti => &[],
                Brp | Brz | Brzp | Brn | Brnp | Brnz | Brnzp | Jsr | Jmp => &[Label],
                Lea | Lc => &[
                    Register { register: Reg::Rd }, 
                    Label,
                ],
                And | Add => &[
                    Register { register: Reg::Rd },
                    Register { register: Reg::Rs },
                    RegisterOrImmediate { register: Reg::Rt, signed: true, bits: 5 },
                ],
                Mul | Sub | Div | Mod | Or | Xor => &[
                    Register { register: Reg::Rd },
                    Register { register: Reg::Rs },
                    Register { register: Reg::Rt },
                ],
                Sll | Sra | Srl => &[
                    Register { register: Reg::Rd },
                    Register { register: Reg::Rs },
                    Immediate { signed: false, bits: 4 },
                ],
                Not => &[
                    Register { register: Reg::Rd },
                    Register { register: Reg::Rs },
                ],
                Ldr => &[
                    Register { register: Reg::Rd },
                    Register { register: Reg::Rs },
                    Immediate { signed: true, bits: 6 },
                ],
                Str => &[
                    Register { register: Reg::Rt },
                    Register { register: Reg::Rs },
                    Immediate { signed: true, bits: 6 },
                ],
                Const => &[
                    Register { register: Reg::Rd },
                    Immediate { signed: true, bits: 9 },
                ],
                Hiconst => &[
                    Register { register: Reg::Rd },
                    Immediate { signed: false, bits: 8 },
                ],
                Cmp | Cmpu => &[
                    Register { register: Reg::Rs },
                    Register { register: Reg::Rt },
                ],
                Cmpi => &[
                    Register { register: Reg::Rs },
                    Immediate { signed: true, bits: 7 },
                ],
                Cmpiu => &[
                    Register { register: Reg::Rs },
                    Immediate { signed: false, bits: 7 },
                ],
                Jsrr | Jmpr => &[
                    Register { register: Reg::Rs },
                ],
                Trap => &[
                    Immediate { signed: false, bits: 8 },
                ],
            };
            
            let instruction = self.parse_instruction(instruction_type, specs);
            
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
        
        Some(Ok(block))
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Block<'a>, String>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next_block()
    }
}

fn op_err_str(op: &Operand) -> &'static str {
    use Operand::*;
    
    match op {
        Register { ..} => "register",
        Label => "label",
        Immediate { .. } => "immediate value",
        RegisterOrImmediate { .. } => "register or immediate value",
    }
}

fn number(i: usize) -> &'static str {
    match i {
        1 => "first",
        2 => "second",
        3 => "third",
        _ => panic!("internal error: passed {} to number", i),
    }
}

fn number_fits(i: i32, signed: bool, bits: u8) -> bool {
    let mut min = 0;
    let mut max = 1 << bits;
    if signed {
        let change = 1 << (bits - 1);
        min -= change;
        max -= change;
    }
    i >= min && i < max
}
