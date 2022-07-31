use crate::{S, Spannable};
use crate::c::lexer::{Token, TokenType};
use crate::c::Error;

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
pub enum Literal<'s> {
    Numeric(S<'s, i32>),
}

#[derive(Debug)]
pub enum BinaryType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl BinaryType {
    fn precedence(&self) -> u8 {
        use BinaryType::*;
        match *self {
            Add | Sub => 5,
            Mul | Div | Mod => 6,
        }
    }
}

#[derive(Debug)]
pub struct Binary<'s> {
    pub left: Box<Expression<'s>>,
    pub ty: BinaryType,
    pub right: Box<Expression<'s>>,
}


#[derive(Debug)]
pub enum ExpressionType<'s> {
    Literal(Literal<'s>),
    Binary(Binary<'s>)
}

#[derive(Debug)]
pub struct Expression<'s> {
    pub ty: ExpressionType<'s>,
    pub expr_ty: Option<Type>,
}

#[derive(Debug)]
pub struct Return<'s> {
    pub expr: Option<Expression<'s>>,
}

#[derive(Debug)]
pub enum StatementType<'s> {
    Return(S<'s, Return<'s>>),
}

#[derive(Debug)]
pub struct Statement<'s> {
    pub ty: StatementType<'s>,
}

#[derive(Debug)]
pub struct Procedure<'s> {
    pub args: Vec<S<'s, (Type, &'s str)>>,
    pub name: S<'s, &'s str>,
    pub return_type: Option<S<'s, Type>>,
    pub body: Vec<Statement<'s>>,
}

#[derive(Debug)]
pub enum TopLevelType<'s> {
    Procedure(Procedure<'s>),
}

#[derive(Debug)]
pub struct TopLevel<'s> {
    pub ty: TopLevelType<'s>,
}


pub struct Parser<'s> {
    tokens: Vec<S<'s, Token<'s>>>,
    next_token: usize,
    _peek_token: Option<Token<'s>>,
}

impl<'s> Parser<'s> {
    pub fn new(tokens: Vec<S<'s, Token<'s>>>) -> Self {
        Parser {
            tokens,
            next_token: 0,
            _peek_token: None,
        }
    }
    
    fn peek(&mut self) -> Option<&S<'s, Token<'s>>> {
        self.tokens.get(self.next_token)
    }
    
    fn consume(&mut self) -> Option<S<'s, Token<'s>>> {
        let token = self.tokens.get(self.next_token).copied();
        self.next_token += 1;
        token
    }
    
    fn next_token_expected(&mut self, expected: &str) -> Result<S<'s, Token<'s>>, Error> {
        self.consume().ok_or_else(|| format!("Expected {}, found end of file.", expected))
    }
    
    fn next_token_expected_of_type(&mut self, expected: &str, ty: TokenType) -> Result<S<'s, Token<'s>>, Error> {
        self.next_token_expected(expected).and_then(|token| {
            if ty == token.ty {
                Ok(token)
            } else {
                Err(format!("Expected {}, found '{}'", expected, token.chars))
            }
        })
    }
    
    fn expression(&mut self, precedence: u8) -> Result<Expression<'s>, Error> {
        use crate::c::lexer;
        let num = self.next_token_expected("numeric literal")?;
        let n = if let TokenType::Literal(lexer::Literal::Numeric(n)) = num.ty {
            n
        } else {
            return Err(format!("Expected numeric literal, found '{}'", num.chars));
        };
        let literal = Literal::Numeric(n.spanned(num.span));
        let ty = ExpressionType::Literal(literal);
        let mut expr = Expression {
            ty,
            expr_ty: None,
        };
        
        // The most based of parsing techniques
        // All I know about Pratt parsing is that there is a loop and recursion
        while let Some(next) = self.peek() {
            let ty = match next.ty {
                TokenType::Plus => Some(BinaryType::Add),
                TokenType::Minus => Some(BinaryType::Sub),
                TokenType::Star => Some(BinaryType::Mul),
                TokenType::Slash => Some(BinaryType::Div),
                TokenType::Percent => Some(BinaryType::Mod),
                _ => None,
            };
            if let Some(ty) = ty {
                if ty.precedence() >= precedence {
                    let _op = self.consume().unwrap();
                    let right = self.expression(ty.precedence())?;
                    let binary = Binary {
                        left: Box::new(expr),
                        ty,
                        right: Box::new(right),
                    };
                    expr = Expression {
                        ty: ExpressionType::Binary(binary),
                        expr_ty: None,
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(expr)
    }
    
    fn return_statement(&mut self, ret: S<'s, Token<'s>>) -> Result<S<'s, Return<'s>>, Error> {
        let next = match self.peek() {
            Some(n) => n,
            None => return Err(format!("Expected ';' or an expression, found end of file.")),
        };
        let return_value = match next.ty {
            TokenType::Semicolon => None,
            _ => Some(self.expression(0)?)
        };
        let _semicolon = self.next_token_expected("';'")?;
        
        let return_ = Return {
            expr: return_value,
        };
        Ok(return_.spanned(ret.span))
    }
    
    fn statement(&mut self) -> Option<Result<Statement<'s>, Error>> {
        let ret = self.peek()?;
        let stmt_ty: StatementType<'s> = match ret.ty {
            TokenType::Return => {
                let ret = self.consume().unwrap();
                let ret = match self.return_statement(ret) {
                    Ok(r) => r,
                    Err(e) => return Some(Err(e)),
                };
                StatementType::Return(ret)
            }
            _ => return None,
        };
        
        Some(Ok(Statement {
            ty: stmt_ty,
        }))
    }
    
    fn procedure(&mut self, ty: S<'s, Token<'s>>, name: S<'s, Token<'s>>, _left_paren: S<'s, Token<'s>>) -> Result<Procedure<'s>, Error> {
        self.next_token_expected_of_type("')'", TokenType::RightParen)?;
        self.next_token_expected_of_type("'{'", TokenType::LeftBrace)?;
        
        let mut stmts = Vec::new();
        while let Some(stmt) = self.statement() {
            let stmt = stmt?;
            stmts.push(stmt);
        }

        self.next_token_expected_of_type("'}'", TokenType::RightBrace)?;
        
        let return_type = match ty.ty {
            TokenType::Int => Type::Int,
            _ => return Err(format!("'int' is the only allowed return type.")),
        };
        
        Ok(Procedure {
            args: Vec::new(),
            name: name.chars.spanned(name.span),
            return_type: Some(return_type.spanned(ty.span)),
            body: stmts,
        })
    }
    
    fn top_level_decl(&mut self, ty: S<'s, Token<'s>>) -> Result<TopLevel<'s>, Error> {
        let identifier = self.next_token_expected_of_type("an identifier", TokenType::Identifier)?;
        let third = self.next_token_expected_of_type("'('", TokenType::LeftParen)?;
        
        let top_level_ty = TopLevelType::Procedure(self.procedure(ty, identifier, third)?);
        
        Ok(TopLevel { ty: top_level_ty })
    }
    
    fn top_level(&mut self) -> Option<Result<TopLevel<'s>, Error>> {
        let first = self.consume()?;
        let res = match first.ty {
            TokenType::Int => self.top_level_decl(first),
            _ => Err(format!("Expected 'int', found '{}'", first.chars)),
        };
        Some(res)
    }

    pub fn fill(&mut self, top_levels: &mut Vec<TopLevel<'s>>) -> Result<(), Error> {
        while let Some(top_level) = self.top_level().transpose()? {
            top_levels.push(top_level);
        }
        Ok(())
    }
}