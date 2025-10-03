use std::process;

pub struct Lexer {
    source: String,
    pub cur_char: char,
    cur_pos: i32,
    next_char: fn(),
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string() + "\n",
            cur_char: '\0',
            cur_pos: -1,
            next_char: || {},
        }
    }

    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos >= self.source.len() as i32 {
            self.cur_char = '\0';
        } else {
            self.cur_char = self
                .source
                .chars()
                .nth(self.cur_pos as usize)
                .expect("Failed to get char");
        }
    }

    pub fn peek(&mut self) -> char {
        if self.cur_pos + 1 > self.source.len() as i32 {
            return '\0';
        }
        let cur_pos = self.cur_pos + 1;
        self.source.chars().nth(cur_pos as usize).unwrap_or('\0')
    }

    pub fn get_token(&mut self) -> Token {
        let token: Token;

        match self.cur_char {
            '+' => token = Token::new(self.cur_char, TokenType::PLUS),
            '-' => token = Token::new(self.cur_char, TokenType::MINUS),
            '*' => token = Token::new(self.cur_char, TokenType::ASTERISK),
            '/' => token = Token::new(self.cur_char, TokenType::SLASH),
            '\n' => token = Token::new(self.cur_char, TokenType::NEWLINE),
            '\0' => token = Token::new('\0', TokenType::EOF),
            _ => {
                println!("Token not recognized");
                process::exit(0);
            }
        };

        self.next_char();
        token
    }
}

#[derive(Debug,PartialEq)]
pub enum TokenType {
    EOF = -1,
    NEWLINE = 0,
    NUMBER = 1,
    IDENT = 2,
    STRING = 3,
    // Keywords.
    LABEL = 101,
    GOTO = 102,
    PRINT = 103,
    INPUT = 104,
    LET = 105,
    IF = 106,
    THEN = 107,
    ENDIF = 108,
    WHILE = 109,
    REPEAT = 110,
    ENDWHILE = 111,
    // Operators.
    EQ = 201,
    PLUS = 202,
    MINUS = 203,
    ASTERISK = 204,
    SLASH = 205,
    EQEQ = 206,
    NOTEQ = 207,
    LT = 208,
    LTEQ = 209,
    GT = 210,
    GTEQ = 211,
}

pub struct Token {
    text: char,
    pub kind: TokenType,
}

impl Token {
    pub fn new(token_text: char, token_kind: TokenType) -> Self {
        Token {
            text: token_text,
            kind: token_kind,
        }
    }
}
