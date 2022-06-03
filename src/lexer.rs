#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Debug)]
enum DirectiveType {
    Os,
    Code,
    Data,
    Addr,
    Falign,
    Fill,
    Stringz,
    Blkw,
    Const,
    UConst,
}

#[derive(Debug)]
enum InstructionType {
    Nop,
    Brp,
    Brz,
    Brzp,
    Brn,
    Brnp,
    Brnz,
    Brnzp,
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    And,
    Not,
    Or,
    Xor,
    Ldr,
    Str,
    Const,
    Hiconst,
    Cmp,
    Cmpu,
    Cmpi,
    Cmpiu,
    Sll,
    Sra,
    Srl,
    Jsrr,
    Jsr,
    Jmpr,
    Jmp,
    Trap,
    Rti,
    Ret,
    Lea,
    Lc,
}

/// Hex Literals and Registers could be confused for labels and vice versa, so we parse identifiers
/// as though they could be either if they lead with an 'r' or an 'h'
#[derive(Debug)]
enum Identifier {
    Hex(u16),
    Register(u8),
    Identifier,
}

#[derive(Debug)]
enum LiteralType {
    Signed(i16),
    Unsigned(u16),
}

#[derive(Debug)]
enum TokenType {
    Literal(LiteralType),
    Identifier(Identifier),
    Directive(DirectiveType),
    Instruction(InstructionType),
    Comma,
    Colon,
}

#[derive(Debug)]
pub struct Token<'input> {
    span: Span,
    chars: &'input str,
    ty: TokenType,
}

struct CharIter<'input> {
    iter: std::str::CharIndices<'input>,
    peek_pair: Option<(usize, char)>,
    input: &'input str,
}

impl<'input> CharIter<'input> {
    fn new(input: &'input str) -> Self {
        CharIter {
            iter: input.char_indices(),
            peek_pair: None,
            input,
        }
    }
    
    fn fill_peek(&mut self) {
        if self.peek_pair.is_none() {
            if let Some(p) = self.iter.next() {
                self.peek_pair = Some(p);
            }
        }
    }
    
    fn peek(&mut self) -> Option<char> {
        self.fill_peek();
        Some(self.peek_pair?.1)
    }
    
    fn peek_position(&mut self) -> usize {
        self.fill_peek();
        match self.peek_pair {
            Some((p, _)) => p,
            None => self.input.len(),
        }
    }
    
    fn consume(&mut self) -> Option<char> {
        match self.peek_pair {
            Some((_, c)) => {
                self.peek_pair = None;
                Some(c)
            }
            None => match self.iter.next() {
                Some((_, c)) => Some(c),
                None => None,
            }
        }
    }
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
        "uconst" => Some(UConst),
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


pub struct Lexer<'input> {
    input: &'input str,
    char_iter: CharIter<'input>,
    token_start: usize,
    line: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let char_iter = CharIter::new(input);
        Lexer { 
            input, 
            char_iter,
            token_start: 0,
            line: 1,
        }
    }
    
    fn span(&mut self) -> Span {
        Span {
            start: self.token_start,
            end: self.char_iter.peek_position(),
            line: self.line,
        }
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
    
    fn directive(&mut self) -> Option<Result<Token<'input>, String>> {
        if !self.check(is_identifier) {
            return Some(Err(format!("Expected directive name after '.'.")));
        } else {
            self.consume_while(is_alpha);
            if !self.check(is_whitespace) {
                return Some(Err(format!("Expected space after directive name.")))
            }
        }

        let span = self.span();
        let chars = &self.input[span.start..span.end];
        let ty = if let Some(ty) = directive_type(&chars[1..]) {
            TokenType::Directive(ty)
        } else {
            return Some(Err(format!("{} is not a directive name.", &chars[1..])));
        };
        let token = Token { span, chars, ty };
    
        return Some(Ok(token));
    }
    
    fn decimal(&mut self, leading_pound: bool) -> Option<Result<Token<'input>, String>> {
        let mut is_negative = false;

        if leading_pound {
            if self.check(|c| c == '-') {
                self.char_iter.consume();
                is_negative = true;
            }
            if !self.check(is_decimal) {
                return Some(Err(format!("Expected decimal number after '#'.")));
            }
        }
        
        self.consume_while(is_decimal);
        if !self.check(is_whitespace) {
            return Some(Err(String::from("Expected space after decimal literal.")));
        }

        let span = self.span();
        let chars = &self.input[span.start..span.end];
        let mut value_text = chars;
        if leading_pound { 
            value_text = &value_text[1..];
        }

        if !is_negative {
            match value_text.parse::<u16>() {
                Ok(v) =>  {
                    let token = Token { span, chars, ty: TokenType::Literal(LiteralType::Unsigned(v)) };
                    return Some(Ok(token));
                },
                Err(e) => {
                    match e.kind() {
                        std::num::IntErrorKind::PosOverflow => return Some(Err(format!("{} is too large to fit in a 16 bit unsigned.", value_text))),
                        _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                    }
                }
            };
        } else {
            match value_text.parse::<i16>() {
                Ok(v) =>  {
                    let token = Token { span, chars, ty: TokenType::Literal(LiteralType::Signed(v)) };
                    return Some(Ok(token));
                },
                Err(e) => {
                    match e.kind() {
                        std::num::IntErrorKind::PosOverflow => return Some(Err(format!("{} is too large to fit in a 16 bit signed.", value_text))),
                        std::num::IntErrorKind::NegOverflow => return Some(Err(format!("{} is too small to fit in a 16 bit signed.", value_text))),
                        _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                    }
                }
            };
        }
        
    }
    
    fn single_char(&mut self, ty: TokenType) -> Option<Result<Token<'input>, String>> {
        let span = self.span();
        let chars = &self.input[span.start..span.end];
        let token = Token { span, chars, ty };
        Some(Ok(token))
    }
    
    fn identifier(&mut self, could_be_hex: bool, could_be_register: bool) -> Option<Result<Token<'input>, String>> {
        self.consume_while(is_identifier);
        let span = self.span();
        let chars = &self.input[span.start..span.end];

        if could_be_hex && chars[1..].chars().all(is_hex) {
            let value_text = &chars[1..];
            let value = match u16::from_str_radix(value_text, 16) {
                Ok(v) => v,
                Err(e) => {
                    match e.kind() {
                        std::num::IntErrorKind::PosOverflow => return Some(Err(format!("{} is too large to fit in 16 bits.", value_text))),
                        _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                    }
                }
            };

            let token = Token { span, chars, ty: TokenType::Identifier(Identifier::Hex(value)) };
            return Some(Ok(token));
        }
        
        if could_be_register && chars[1..].chars().all(is_decimal) {
            
            let value_text = &chars[1..];
            let value = match u8::from_str_radix(value_text, 10) {
                Ok(v) => v,
                Err(e) => {
                    match e.kind() {
                        std::num::IntErrorKind::PosOverflow => return Some(Err(format!("No such register {}.", value_text))),
                        _ => panic!("Internal integer parse error {:?} on text '{}'.", e, value_text),
                    }
                }
            };
            if value >= 8 {
                return Some(Err(format!("No such register {}.", value_text)));
            }

            let token = Token { span, chars, ty: TokenType::Identifier(Identifier::Register(value)) };
            return Some(Ok(token));
        }
        
        if let Some(ty) = instruction_type(chars) {
            let token = Token { span, chars, ty: TokenType::Instruction(ty) };
            return Some(Ok(token))
        }

        let token = Token { span, chars, ty: TokenType::Identifier(Identifier::Identifier) };
        Some(Ok(token))
    }
    
    fn next_token(&mut self) -> Option<Result<Token<'input>, String>> {
        loop {
            self.consume_whitespace();
            self.token_start = self.char_iter.peek_position();
    
            match self.char_iter.consume()? {
                ';' => self.consume_while(|c| c != '\n'),
                ',' => return self.single_char(TokenType::Comma),
                ':' => return self.single_char(TokenType::Colon),
                '.' => return self.directive(),
                '#' => return self.decimal(true),
                c if is_decimal(c) => return self.decimal(false),
                'r' | 'R' => return self.identifier(false, true),
                'x' | 'X' => return self.identifier(true, false),
                c if is_identifier(c) => return self.identifier(false, false),
                _ => {
                    self.consume_while(|c| !is_token_delimeter(c));
                    let span = self.span();
                    return Some(Err(format!("Unexpected '{}'.", &self.input[span.start..span.end])));
                }
            }
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Token<'input>, String>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
