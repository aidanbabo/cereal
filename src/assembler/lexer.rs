use std::borrow::Cow;

use crate::{InstructionType, Span};
use crate::char_utils::CharIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectiveType {
    Os,
    Code,
    Data,
    Addr,
    Falign,
    Fill,
    Stringz,
    Blkw,
    Const,
    Uconst,
}

/// Hex Literals and Registers could be confused for labels and vice versa, so we parse identifiers
/// as though they could be either if they lead with an 'r' or an 'x'
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Identifier {
    Hex(u16),
    Register(u8),
    Identifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralType {
    Signed(i16),
    Unsigned(u16),
}

impl LiteralType {
    pub fn to_i32(self) -> i32 {
        match self {
            LiteralType::Signed(s) => s as i32,
            LiteralType::Unsigned(u) => u as i32,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    Literal(LiteralType),
    Identifier(Identifier),
    Directive(DirectiveType),
    Instruction(InstructionType),
    String(Cow<'a, str>),
    Comma,
    Colon,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub span: Span<'a>,
    pub chars: &'a str,
    pub ty: TokenType<'a>,
}

fn is_decimal(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_hex(c: char) -> bool {
    is_decimal(c) || c >= 'a' && c <= 'f' || c >= 'A' && c <= 'F'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

fn is_identifier(c: char) -> bool {
     is_alpha(c) || is_decimal(c) || c == '_'
}

fn is_whitespace(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\t' || c == ' '
}

fn is_token_delimeter(c: char) -> bool {
    is_whitespace(c) || c == ',' || c == ':'
}

fn is_not_token(c: char) -> bool {
    is_whitespace(c) || c == ';'
}

fn directive_type(s: &str) -> Option<DirectiveType> {
    use DirectiveType::*;
    match &*s.to_lowercase() {
        "os" => Some(Os),
        "code" => Some(Code),
        "data" => Some(Data),
        "addr" => Some(Addr),
        "falign" => Some(Falign),
        "fill" => Some(Fill),
        "stringz" => Some(Stringz),
        "blkw" => Some(Blkw),
        "const" => Some(Const),
        "uconst" => Some(Uconst),
        _ => None,
    }
}

fn instruction_type(s: &str) -> Option<InstructionType> {
    use InstructionType::*;
    match &*s.to_lowercase() {
        "nop" => Some(Nop),
        "brp" => Some(Brp),
        "brz" => Some(Brz),
        "brzp" => Some(Brzp),
        "brn" => Some(Brn),
        "brnp" => Some(Brnp),
        "brnz" => Some(Brnz),
        "brnzp" => Some(Brnzp),
        "add" => Some(Add),
        "mul" => Some(Mul),
        "sub" => Some(Sub),
        "div" => Some(Div),
        "mod" => Some(Mod),
        "and" => Some(And),
        "not" => Some(Not),
        "or" => Some(Or),
        "xor" => Some(Xor),
        "ldr" => Some(Ldr),
        "str" => Some(Str),
        "const" => Some(Const),
        "hiconst" => Some(Hiconst),
        "cmp" => Some(Cmp),
        "cmpu" => Some(Cmpu),
        "cmpi" => Some(Cmpi),
        "cmpiu" => Some(Cmpiu),
        "sll" => Some(Sll),
        "sra" => Some(Sra),
        "srl" => Some(Srl),
        "jsrr" => Some(Jsrr),
        "jsr" => Some(Jsr),
        "jmpr" => Some(Jmpr),
        "jmp" => Some(Jmp),
        "trap" => Some(Trap),
        "rti" => Some(Rti),
        "ret" => Some(Ret),
        "lea" => Some(Lea),
        "lc" => Some(Lc),
        _ => None,
    }
}


pub struct Lexer<'a> {
    input: &'a str,
    char_iter: CharIter<'a>,
    token_start: usize,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let char_iter = CharIter::new(input);
        Lexer { 
            input, 
            char_iter,
            token_start: 0,
            line: 1,
        }
    }
    
    fn span(&mut self) -> (Span<'a>, &'a str) {
        let span = Span::new(
            self.input,
            self.token_start,
            self.char_iter.peek_position(),
            self.line,
        );
        (span, &self.input[span.start..span.end])
    }
    
    fn consume_while(&mut self, f: impl Fn(char) -> bool) {
        while let Some(c) = self.char_iter.peek() {
            if !f(c) {
                return;
            }
            self.char_iter.consume();
            if c == '\n' { 
                self.line += 1
            }
        }
    }
    
    fn consume_whitespace(&mut self) {
        self.consume_while(is_whitespace);
    }
    
    fn check(&mut self, f: impl Fn(char) -> bool) -> bool {
        match self.char_iter.peek() {
            Some(c) if f(c) => true,
            _ => false,
        }
    }
    
    fn directive(&mut self) -> Result<Token<'a>, String> {
        if !self.check(is_identifier) {
            return Err(format!("Expected directive name after '.'."));
        } else {
            self.consume_while(is_alpha);
            if !self.check(is_not_token) {
                return Err(format!("Expected space after directive name."));
            }
        }

        let (span, chars) = self.span();
        let ty = if let Some(ty) = directive_type(&chars[1..]) {
            TokenType::Directive(ty)
        } else {
            return Err(format!("{} is not a directive name.", &chars[1..]));
        };
        let token = Token { span, chars, ty };
    
        return Ok(token);
    }
    
    fn decimal(&mut self, leading_pound: bool) -> Result<Token<'a>, String> {
        let mut is_negative = false;

        if leading_pound {
            if self.check(|c| c == '-') {
                self.char_iter.consume();
                is_negative = true;
            }
            if !self.check(is_decimal) {
                return Err(format!("Expected decimal number after '#'."));
            }
        }
        
        self.consume_while(is_decimal);
        if !self.check(is_not_token) {
            return Err(String::from("Expected space after decimal literal."));
        }

        let (span, chars) = self.span();
        let mut value_text = chars;
        if leading_pound { 
            value_text = &value_text[1..];
        }

        if !is_negative {
            match value_text.parse::<u16>() {
                Ok(v) =>  {
                    let token = Token { span, chars, ty: TokenType::Literal(LiteralType::Unsigned(v)) };
                    return Ok(token);
                },
                Err(e) => match e.kind() {
                    std::num::IntErrorKind::PosOverflow => return Err(format!("{} is too large to fit in a 16 bit unsigned integer.", value_text)),
                    _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                }
            };
        } else {
            match value_text.parse::<i16>() {
                Ok(v) =>  {
                    let token = Token { span, chars, ty: TokenType::Literal(LiteralType::Signed(v)) };
                    return Ok(token);
                },
                Err(e) => match e.kind() {
                    std::num::IntErrorKind::PosOverflow => return Err(format!("{} is too large to fit in a 16 bit signed integer.", value_text)),
                    std::num::IntErrorKind::NegOverflow => return Err(format!("{} is too small to fit in a 16 bit signed integer.", value_text)),
                    _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                }
            };
        }
        
    }
    
    fn single_char(&mut self, ty: TokenType<'a>) -> Result<Token<'a>, String> {
        let (span, chars) = self.span();
        let token = Token { span, chars, ty };
        Ok(token)
    }
    
    fn string(&mut self) -> Result<Token<'a>, String> {

        // custom consume_while to accomodate \" sequence
        let mut escaped = false;
        while let Some(c) = self.char_iter.peek() {
            if c == '"' && !escaped {
                break;
            }

            if c == '\\' {
                escaped = true;
            } else {
                escaped = false;
            }
            self.char_iter.consume();
            if c == '\n' { 
                self.line += 1
            }
        }

        self.consume_while(|c| c != '"');
        if self.char_iter.consume() != Some('"') {
            return Err("Expected closing '\"' in string literal.".to_string());
        }

        let (span, chars) = self.span();
        let contents = &chars[1..chars.len() - 1];
        let contents = if contents.contains('\\') {
            let mut string = String::new();
            let mut escaped = false;
            for c in contents.chars() {
                if escaped {
                    escaped = false;
                    match c {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        '"' => string.push('"'),
                        '\\' => string.push('\\'),
                        _ => return Err(format!("Unsupported escape sequence '\\{}'", c)),
                    }
                } else if c != '\\' {
                    string.push(c);
                } else {
                    escaped = true;
                }
            }
            Cow::Owned(string)
        } else {
            Cow::Borrowed(contents)
        };

        let token = Token { span, chars, ty: TokenType::String(contents) };
        Ok(token)
    }
    
    fn hex(&mut self) -> Result<Token<'a>, String> {
        self.consume_while(is_hex);

        if !self.check(is_not_token) {
            return Err(String::from("Expected space after hex literal."));
        }

        let (span, chars) = self.span();
        
        let value_text = &chars[2..];
        let value = match u16::from_str_radix(value_text, 16) {
            Ok(v) => v,
            Err(e) => match e.kind() {
                std::num::IntErrorKind::PosOverflow => return Err(format!("{} is too large to fit in 16 bits.", value_text)),
                _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
            }
        };

        let token = Token { span, chars, ty: TokenType::Literal(LiteralType::Unsigned(value)) };
        Ok(token)
    }
    
    fn identifier(&mut self, could_be_hex: bool, could_be_register: bool) -> Result<Token<'a>, String> {
        self.consume_while(is_identifier);
        let (span, chars) = self.span();

        if could_be_hex && chars[1..].chars().all(is_hex) {
            let value_text = &chars[1..];
            let value = match u16::from_str_radix(value_text, 16) {
                Ok(v) => v,
                Err(e) => match e.kind() {
                    std::num::IntErrorKind::PosOverflow => return Err(format!("{} is too large to fit in 16 bits.", value_text)),
                    _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                }
            };

            let token = Token { span, chars, ty: TokenType::Identifier(Identifier::Hex(value)) };
            return Ok(token);
        }
        
        if could_be_register && chars[1..].chars().all(is_decimal) {
            
            let value_text = &chars[1..];
            let value = match u8::from_str_radix(value_text, 10) {
                Ok(v) => v,
                Err(e) => match e.kind() {
                    std::num::IntErrorKind::PosOverflow => return Err(format!("No such register '{}'.", value_text)),
                    _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                }
            };
            if value >= 8 {
                return Err(format!("No such register '{}'.", value_text));
            }

            let token = Token { span, chars, ty: TokenType::Identifier(Identifier::Register(value)) };
            return Ok(token);
        }
        
        if let Some(ty) = instruction_type(chars) {
            let token = Token { span, chars, ty: TokenType::Instruction(ty) };
            return Ok(token)
        }

        let token = Token { span, chars, ty: TokenType::Identifier(Identifier::Identifier) };
        Ok(token)
    }
    
    fn next_token(&mut self) -> Option<Result<Token<'a>, String>> {
        loop {
            self.consume_whitespace();
            self.token_start = self.char_iter.peek_position();
    
            match self.char_iter.consume()? {
                ';' => self.consume_while(|c| c != '\n'),
                ',' => return Some(self.single_char(TokenType::Comma)),
                ':' => return Some(self.single_char(TokenType::Colon)),
                '.' => return Some(self.directive()),
                '#' => return Some(self.decimal(true)),
                c if is_decimal(c) => {
                    if let Some(c) = self.char_iter.peek() {
                        if c == 'x' || c == 'X' {
                            self.char_iter.consume();
                            return Some(self.hex())
                        }
                    }
                    return Some(self.decimal(false))
                }
                'r' | 'R' => return Some(self.identifier(false, true)),
                'x' | 'X' => return Some(self.identifier(true, false)),
                c if is_identifier(c) => return Some(self.identifier(false, false)),
                '"' => return Some(self.string()),
                _ => {
                    self.consume_while(|c| !is_token_delimeter(c));
                    let (_, chars) = self.span();
                    return Some(Err(format!("Unexpected '{}'.", chars)));
                }
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, (usize, String)>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().map(|t| t.map_err(|e| (self.line, e)))
    }
}
