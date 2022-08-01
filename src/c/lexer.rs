use crate::{Span, S, Spannable};
use crate::char_utils::CharIter;
use crate::c::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    Numeric(i32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    // Fat types
    Literal(Literal),
    Identifier,

    // Keywords
    Return,
    Int,
    
    // Characters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Tilde,
    Pipe,
    Carrot,
    Ampersand,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub chars: &'a str,
    pub ty: TokenType,
}

pub struct Lexer<'a> {
    input: &'a str,
    iter: CharIter<'a>,
    token_start: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            iter: CharIter::new(input),
            token_start: 0,
            line: 1,
        }
    }

    fn span(&mut self) -> (Span<'a>, &'a str) {
        let span = Span::new(
            self.input,
            self.token_start,
            self.iter.peek_position(),
            self.line,
        );
        (span, &self.input[span.start..span.end])
    }

    fn consume_while(&mut self, f: impl Fn(char) -> bool) {
        while let Some(c) = self.iter.peek() {
            if !f(c) {
                return;
            }
            self.iter.consume();
            if c == '\n' { 
                self.line += 1
            }
        }
    }
    
    fn consume_whitespace(&mut self) {
        self.consume_while(is_whitespace);
    }
    
    fn check(&mut self, f: impl Fn(char) -> bool) -> bool {
        match self.iter.peek() {
            Some(c) if f(c) => true,
            _ => false,
        }
    }
    
    fn single_char(&mut self, ty: TokenType) -> Result<S<'a, Token<'a>>, Error> {
        let (span, chars) = self.span();
        let token = Token { chars, ty };
        Ok(token.spanned(span))
    }
    
    fn numeric_literal(&mut self) -> Result<S<'a, Token<'a>>, Error> {
        self.consume_while(is_decimal);
        let (span, chars) = self.span();

        if !self.check(is_token_delimeter) {
            return Err(format!("Unexpected '{}' in numeric literal.", self.iter.peek().unwrap()));
        }
        
        let num = i32::from_str_radix(chars, 10).expect("Internal error: failed to parse number from string");
        if !crate::number_fits(num, true, 16) {
            return Err(format!("'{}' cannot fit into a 16-bit signed number.", num));
        }

        let ty = TokenType::Literal(Literal::Numeric(num));
        let token = Token { chars, ty };
        Ok(token.spanned(span))
    }
    
    fn identifier(&mut self) -> Result<S<'a, Token<'a>>, Error> {
        self.consume_while(is_identifier_rest);
        let (span, chars) = self.span();
        
        if !self.check(is_token_delimeter) {
            return Err(format!("Unexpected '{}' in identifier.", self.iter.peek().unwrap()));
        }
        
        let ty = match keyword(chars) {
            Some(keyword) => keyword,
            None => TokenType::Identifier,
        };
        let token = Token { chars, ty };
        Ok(token.spanned(span))
    }

    fn next_token(&mut self) -> Option<Result<S<'a, Token<'a>>, Error>> {
        loop {
            self.consume_whitespace();
            self.token_start = self.iter.peek_position();
            let token = match self.iter.consume()? {
                '(' => self.single_char(TokenType::LeftParen),
                ')' => self.single_char(TokenType::RightParen),
                '{' => self.single_char(TokenType::LeftBrace),
                '}' => self.single_char(TokenType::RightBrace),
                ';' => self.single_char(TokenType::Semicolon),
                '+' => self.single_char(TokenType::Plus),
                '-' => self.single_char(TokenType::Minus),
                '*' => self.single_char(TokenType::Star),
                '/' => self.single_char(TokenType::Slash),
                '%' => self.single_char(TokenType::Percent),
                '~' => self.single_char(TokenType::Tilde),
                '&' => self.single_char(TokenType::Ampersand),
                '^' => self.single_char(TokenType::Carrot),
                '|' => self.single_char(TokenType::Pipe),
                c if is_decimal(c) => self.numeric_literal(),
                c if is_identifier_start(c) => self.identifier(),
                _ => {
                    self.consume_while(is_not_token_delimeter);
                    let (_, chars) = self.span();
                    return Some(Err(format!("Unexpected '{}'.", chars)));
                }
            };
            return Some(token);
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<S<'a, Token<'a>>, Error>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn is_whitespace(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\t' || c == ' '
}

fn is_decimal(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

fn is_identifier_start(c: char) -> bool {
    is_alpha(c) || c == '_'
}

fn is_identifier_rest(c: char) -> bool {
    is_identifier_start(c) || is_decimal(c)
}

fn is_not_token_delimeter(c: char) -> bool {
    !is_token_delimeter(c)
}

fn is_token_delimeter(c: char) -> bool {
    is_whitespace(c) || c == '(' || c == ')' || c == '}' || c == '{' || c == ';' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '~' || c == '|' || c == '^' || c == '&'
}

fn keyword(s: &str) -> Option<TokenType> {
    use TokenType::*;
    let keyword = match s {
        "int" => Int,
        "return" => Return,
        _ => return None,
    };
    Some(keyword)
}
