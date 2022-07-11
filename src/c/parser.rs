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
pub enum ExpressionType<'s> {
    Literal(Literal<'s>),
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
    
    /*
    fn next_token_of_type(&mut self, ty: TokenType) -> Result<Token<'s>, Error> {
        let token = self.next_token_expected
    }
    */
    
    fn expression(&mut self) -> Result<Expression<'s>, Error> {
        use crate::c::lexer;
        let num = self.next_token_expected("numeric literal")?;
        let n = if let TokenType::Literal(lexer::Literal::Numeric(n)) = num.ty {
            n
        } else {
            return Err(format!("Expected numeric literal, found '{}'", num.chars));
        };
        let literal = Literal::Numeric(n.spanned(num.span));
        let ty = ExpressionType::Literal(literal);
        let expr = Expression {
            ty,
            expr_ty: None,
        };
        Ok(expr)
    }
    
    fn return_statement(&mut self, ret: S<'s, Token<'s>>) -> Result<S<'s, Return<'s>>, Error> {
        let next = match self.peek() {
            Some(n) => n,
            None => return Err(format!("Expected ';' or an expression, found end of file.")),
        };
        let return_value = match next.ty {
            TokenType::Semicolon => None,
            _ => Some(self.expression()?)
        };
        let _semicolon = self.next_token_expected("';'")?;
        
        let _return = Return {
            expr: return_value,
        };
        Ok(_return.spanned(ret.span))
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
        let right_paren = self.next_token_expected("')'")?;
        if let TokenType::RightParen = right_paren.ty {
        } else {
            return Err(format!("Expected ')', found '{}'", right_paren.chars));
        }
        
        // @Todo duplication
        let left_brace = self.next_token_expected("'{'")?;
        if let TokenType::LeftBrace = left_brace.ty {
        } else {
            return Err(format!("Expected '{{', found '{}'", left_brace.chars));
        }
        
        let mut stmts = Vec::new();
        while let Some(stmt) = self.statement() {
            let stmt = stmt?;
            stmts.push(stmt);
        }

        // @Todo duplication
        let right_brace = self.next_token_expected("'}'")?;
        if let TokenType::RightBrace = right_brace.ty {
        } else {
            return Err(format!("Expected '}}', found '{}'", right_brace.chars));
        }
        
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
        let second = self.next_token_expected("an identifier")?;

        let identifier = if let TokenType::Identifier = second.ty {
            second
        } else {
            return Err(format!("Expected identifier, found '{}'", second.chars));
        };
        
        let third = self.next_token_expected("'('")?;
        
        let top_level_ty = match third.ty {
            TokenType::LeftParen => TopLevelType::Procedure(self.procedure(ty, identifier, third)?),
            _ => return Err(format!("Expected '(', found '{}'", third.chars)),
        };
        
        Ok(TopLevel { ty: top_level_ty })
    }
    
    fn top_level(&mut self) -> Option<Result<TopLevel<'s>, Error>> {
        let first = self.consume()?;
        match first.ty {
            TokenType::Int => return Some(self.top_level_decl(first)),
            _ => return Some(Err(format!("Expected 'int', found '{}'", first.chars))),
        }
    }

    pub fn fill(&mut self, top_levels: &mut Vec<TopLevel<'s>>) -> Result<(), Error> {
        while let Some(top_level) = self.top_level() {
            let top_level = top_level?;
            top_levels.push(top_level);
        }
        Ok(())
    }
}