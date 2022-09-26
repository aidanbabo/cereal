use crate::c::ast::*;
use crate::c::lexer::{Token, TokenType};
use crate::c::Error;
use crate::{Spannable, S};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Comma,
    Assignment,
    BitOr,
    BitXor,
    BitAnd,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    // Used for left associative operators (+, -, etc.)
    fn higher(self) -> Self {
        use Precedence::*;
        match self {
            None => Comma,
            Comma => Assignment,
            Assignment => BitOr,
            BitOr => BitXor,
            BitXor => BitAnd,
            BitAnd => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => Primary,
            Primary => unreachable!("Tried to get a super high precedence"),
        }
    }
}

type PrefixFunction<'s> = fn(&mut Parser<'s>) -> Result<Expression<'s>, Error>;
type InfixFunction<'s> = fn(&mut Parser<'s>, Expression<'s>) -> Result<Expression<'s>, Error>;

struct Rule<'s> {
    precedence: Precedence,
    prefix: Option<PrefixFunction<'s>>,
    infix: Option<InfixFunction<'s>>,
}

impl<'s> Rule<'s> {
    fn for_type(ty: TokenType) -> Rule<'s> {
        use TokenType::*;
        match ty {
            Literal(_) => Rule {
                precedence: Precedence::None,
                prefix: Some(Parser::numeric_literal),
                infix: None,
            },
            Minus => Rule {
                precedence: Precedence::Term,
                prefix: Some(Parser::unary),
                infix: Some(Parser::binary),
            },
            Plus => Rule {
                precedence: Precedence::Term,
                prefix: Some(Parser::unary),
                infix: Some(Parser::binary),
            },
            Star | Slash | Percent => Rule {
                precedence: Precedence::Factor,
                prefix: None,
                infix: Some(Parser::binary),
            },
            Tilde => Rule {
                precedence: Precedence::None,
                prefix: Some(Parser::unary),
                infix: None,
            },
            Pipe => Rule {
                precedence: Precedence::BitOr,
                prefix: None,
                infix: Some(Parser::binary),
            },
            Carrot => Rule {
                precedence: Precedence::BitXor,
                prefix: None,
                infix: Some(Parser::binary),
            },
            Ampersand => Rule {
                precedence: Precedence::BitAnd,
                prefix: None,
                infix: Some(Parser::binary),
            },
            Equals => Rule {
                precedence: Precedence::Assignment,
                prefix: None,
                infix: Some(Parser::assignment),
            },
            LeftParen => Rule {
                precedence: Precedence::Call,
                prefix: Some(Parser::grouping),
                infix: Some(Parser::call),
            },
            Identifier => Rule {
                precedence: Precedence::None,
                prefix: Some(Parser::variable),
                infix: None,
            },
            Comma => Rule {
                precedence: Precedence::Comma,
                prefix: None,
                infix: Some(Parser::comma),
            },
            Return | Int | RightParen | LeftBrace | RightBrace | Semicolon => Rule {
                precedence: Precedence::None,
                prefix: None,
                infix: None,
            },
        }
    }
}

pub struct Parser<'s> {
    tokens: Vec<S<'s, Token<'s>>>,
    next_token: usize,
}

impl<'s> Parser<'s> {
    pub fn new(tokens: Vec<S<'s, Token<'s>>>) -> Self {
        Parser {
            tokens,
            next_token: 0,
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
        self.consume()
            .ok_or_else(|| format!("Expected {}, found end of file.", expected))
    }

    fn next_token_expected_of_type(
        &mut self,
        expected: &str,
        ty: TokenType,
    ) -> Result<S<'s, Token<'s>>, Error> {
        self.next_token_expected(expected).and_then(|token| {
            if ty == token.ty {
                Ok(token)
            } else {
                Err(format!(
                    "(line: {}) Expected {}, found '{}'.",
                    token.span.line, expected, token.chars
                ))
            }
        })
    }

    fn grouping(&mut self) -> Result<Expression<'s>, Error> {
        self.consume();
        let expr = self.expression()?;
        self.next_token_expected_of_type("')'", TokenType::RightParen)?;
        Ok(expr)
    }

    fn numeric_literal(&mut self) -> Result<Expression<'s>, Error> {
        use crate::c::lexer;

        let num = self.next_token_expected("numeric literal")?;
        let n = if let TokenType::Literal(lexer::Literal::Numeric(n)) = num.ty {
            n
        } else {
            return Err(format!("Expected numeric literal, found '{}'", num.chars));
        };
        let literal = Literal::Numeric(n.spanned(num.span));
        let expr = Expression {
            ty: ExpressionType::Literal(literal),
            expr_ty: None,
        };

        Ok(expr)
    }

    fn variable(&mut self) -> Result<Expression<'s>, Error> {
        let expr = Expression {
            ty: ExpressionType::Variable(self.consume().unwrap().chars),
            expr_ty: None,
        };
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression<'s>, Error> {
        let op = self.consume().unwrap();
        let ty = match op.ty {
            TokenType::Minus => UnaryType::Negate,
            TokenType::Tilde => UnaryType::BitNot,
            TokenType::Plus => UnaryType::Plus,
            _ => unreachable!("Input to this function must be a unary op"),
        };
        let expr = self.expression()?;
        let unary = Unary {
            ty: ty.spanned(op.span),
            expr: Box::new(expr),
        };
        let expr = Expression {
            ty: ExpressionType::Unary(unary),
            expr_ty: None,
        };

        Ok(expr)
    }

    fn binary(&mut self, left: Expression<'s>) -> Result<Expression<'s>, Error> {
        let op = self.consume().unwrap();
        let ty = match op.ty {
            TokenType::Plus => BinaryType::Add,
            TokenType::Minus => BinaryType::Sub,
            TokenType::Star => BinaryType::Mul,
            TokenType::Slash => BinaryType::Div,
            TokenType::Percent => BinaryType::Mod,
            TokenType::Ampersand => BinaryType::BitAnd,
            TokenType::Carrot => BinaryType::BitXor,
            TokenType::Pipe => BinaryType::BitOr,
            _ => unreachable!("Input to this function must be binary op"),
        };
        let precedence = Rule::for_type(op.ty).precedence;
        let right = self.parse_precedence(precedence.higher())?;
        let binary = Binary {
            left: Box::new(left),
            ty: ty.spanned(op.span),
            right: Box::new(right),
        };
        let expr = Expression {
            ty: ExpressionType::Binary(binary),
            expr_ty: None,
        };

        Ok(expr)
    }

    fn assignment(&mut self, left: Expression<'s>) -> Result<Expression<'s>, Error> {
        let equals = self.consume().unwrap();
        let right = self.parse_precedence(Precedence::Assignment)?;
        let assignment = Assignment {
            left: Box::new(left),
            ty: AssignmentType::Regular.spanned(equals.span),
            right: Box::new(right),
        };
        let expr = Expression {
            ty: ExpressionType::Assignment(assignment),
            expr_ty: None,
        };
        Ok(expr)
    }

    fn comma(&mut self, left: Expression<'s>) -> Result<Expression<'s>, Error> {
        let _comma = self.consume().unwrap();
        let right = self.parse_precedence(Precedence::Assignment)?;
        let comma = Comma {
            left: Box::new(left),
            right: Box::new(right),
        };
        let expr = Expression {
            ty: ExpressionType::Comma(comma),
            expr_ty: None,
        };
        Ok(expr)
    }

    fn call(&mut self, procedure: Expression<'s>) -> Result<Expression<'s>, Error> {
        assert_eq!(self.consume().unwrap().ty, TokenType::LeftParen);
        let args = {
            let mut args = Vec::new();
            while self
                .peek()
                .filter(|p| p.ty != TokenType::RightParen)
                .is_some()
            {
                let expr = self.parse_precedence(Precedence::Assignment)?;
                args.push(expr);
                // @Todo allows trailing comma
                if self.peek().filter(|p| p.ty == TokenType::Comma).is_some() {
                    self.consume().unwrap();
                }
            }
            args
        };

        self.next_token_expected_of_type("')'", TokenType::RightParen)?;

        let call = Call {
            procedure: Box::new(procedure),
            args,
        };
        let expr = Expression {
            ty: ExpressionType::Call(call),
            expr_ty: None,
        };
        Ok(expr)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expression<'s>, Error> {
        let prefix = match self.peek().and_then(|t| Rule::for_type(t.ty).prefix) {
            Some(p) => p,
            None => return Err("Expected expression".to_string()),
        };

        let mut expr = prefix(self)?;

        while let Some(infix) = self.peek().map(|t| Rule::for_type(t.ty)) {
            if precedence <= infix.precedence {
                expr = infix.infix.unwrap()(self, expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expression<'s>, Error> {
        self.parse_precedence(Precedence::Comma)
    }

    fn return_statement(&mut self, ret: S<'s, Token<'s>>) -> Result<S<'s, Return<'s>>, Error> {
        let next = match self.peek() {
            Some(n) => n,
            None => return Err("Expected ';' or an expression, found end of file.".to_string()),
        };
        let return_value = match next.ty {
            TokenType::Semicolon => None,
            _ => Some(self.expression()?),
        };
        let _semicolon = self.next_token_expected("';'")?;

        let return_ = Return { expr: return_value };
        Ok(return_.spanned(ret.span))
    }

    fn statement(&mut self) -> Result<Statement<'s>, Error> {
        let ret = self
            .peek()
            .expect("Not the last token, expected to parse a statement");
        let stmt_ty = match ret.ty {
            TokenType::Return => {
                let ret = self.consume().unwrap();
                let ret = self.return_statement(ret)?;
                StatementType::Return(ret)
            }
            _ => {
                let expr = self.expression()?;
                self.next_token_expected_of_type("';'", TokenType::Semicolon)?;
                StatementType::Expression(expr)
            }
        };

        Ok(Statement { ty: stmt_ty })
    }

    fn get_names(&mut self) -> Result<Vec<(usize, S<'s, &'s str>)>, Error> {
        let mut names = Vec::new();
        loop {
            if let Some(name) = self.peek() {
                if name.ty == TokenType::Identifier {
                    let name = self.consume().unwrap();
                    names.push((0, name.chars.spanned(name.span)));

                    if let Some(comma) = self.peek() {
                        if comma.ty == TokenType::Comma {
                            self.consume().unwrap();
                            continue;
                        }
                    }
                }
            }
            break;
        }

        if names.is_empty() {
            return Err("Expected identifiers in variable declaration.".to_string());
        }

        self.next_token_expected_of_type("';'", TokenType::Semicolon)?;

        Ok(names)
    }

    fn declaration(&mut self) -> Option<Result<Declaration<'s>, Error>> {
        let next = self.peek()?;
        let ty = match next.ty {
            TokenType::Int => {
                let ty = self.consume().unwrap();
                Type::Int.spanned(ty.span)
            }
            _ => return None,
        };

        let names = match self.get_names() {
            Ok(n) => n,
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(Declaration { ty, names }))
    }

    fn procedure(
        &mut self,
        ty: S<'s, Token<'s>>,
        name: S<'s, Token<'s>>,
    ) -> Result<Procedure<'s>, Error> {
        self.next_token_expected_of_type("'('", TokenType::LeftParen)?;

        let params = {
            let mut params = Vec::new();
            while self.peek().filter(|p| p.ty == TokenType::Int).is_some() {
                let int = self.consume().unwrap();
                let parameter =
                    self.next_token_expected_of_type("identifier", TokenType::Identifier)?;
                params.push((
                    Type::Int.spanned(int.span),
                    parameter.chars.spanned(parameter.span),
                ));

                // @Todo allows trailing commas
                if self.peek().filter(|t| t.ty == TokenType::Comma).is_some() {
                    self.consume().unwrap();
                } else {
                    break;
                }
            }
            params
        };

        self.next_token_expected_of_type("')'", TokenType::RightParen)?;
        self.next_token_expected_of_type("'{'", TokenType::LeftBrace)?;

        let mut decls = Vec::new();
        while let Some(decl) = self.declaration() {
            let decl = decl?;
            decls.push(decl);
        }

        let mut stmts = Vec::new();
        loop {
            if let Some(t) = self.peek() {
                if t.ty == TokenType::RightBrace {
                    break;
                }
            }
            let stmt = self.statement()?;
            stmts.push(stmt);
        }

        self.next_token_expected_of_type("'}'", TokenType::RightBrace)?;

        let return_type = match ty.ty {
            TokenType::Int => Type::Int,
            _ => return Err("'int' is the only allowed return type.".to_string()),
        };

        Ok(Procedure {
            params,
            name: name.chars.spanned(name.span),
            return_type: Some(return_type.spanned(ty.span)),
            declarations: decls,
            body: stmts,
        })
    }

    fn global_variable(
        &mut self,
        ty: S<'s, Token<'s>>,
        first: S<'s, Token<'s>>,
    ) -> Result<GlobalVariable<'s>, Error> {
        let mut names = if self.peek().unwrap().ty == TokenType::Comma {
            self.consume();
            self.get_names()?
        } else {
            self.next_token_expected_of_type("';'", TokenType::Semicolon)?;
            Vec::new()
        };
        names.insert(0, (0, first.chars.spanned(first.span)));

        let ast_ty = match ty.ty {
            TokenType::Int => Type::Int,
            _ => return Err("'int' is the only allowed type.".to_string()),
        };

        Ok(GlobalVariable {
            ty: ast_ty.spanned(ty.span),
            names,
        })
    }

    fn top_level_decl(&mut self, ty: S<'s, Token<'s>>) -> Result<TopLevel<'s>, Error> {
        let identifier =
            self.next_token_expected_of_type("an identifier", TokenType::Identifier)?;

        let top_level_ty = match self.peek() {
            Some(t) if t.ty == TokenType::Semicolon || t.ty == TokenType::Comma => {
                TopLevelType::Variable(self.global_variable(ty, identifier)?)
            }
            Some(t) if t.ty == TokenType::LeftParen => {
                TopLevelType::Procedure(self.procedure(ty, identifier)?)
            }
            Some(t) => return Err(format!("Expected either '(' or ';', found '{}'", t.chars)),
            None => return Err("Expected either '(' or ';', found nothing.".to_string()),
        };

        Ok(TopLevel { ty: top_level_ty })
    }

    fn top_level(&mut self) -> Result<TopLevel<'s>, Error> {
        let int = self.next_token_expected_of_type("'int'", TokenType::Int)?;
        self.top_level_decl(int)
    }

    pub fn fill(&mut self, top_levels: &mut Vec<TopLevel<'s>>) -> Result<(), Error> {
        while self.peek().is_some() {
            let top_level = self.top_level()?;
            top_levels.push(top_level);
        }
        Ok(())
    }
}
