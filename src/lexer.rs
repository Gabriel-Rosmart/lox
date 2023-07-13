use anyhow::Result;

#[derive(Debug)]
#[allow(unused)]
pub enum TokenType {
    /* Single character tokens */
    LeftParen,     /* Character '(' */
    RightParen,    /* Character ')' */
    LeftCurBrace,  /* Character '{' */
    RightCurBrace, /* Character '}' */
    Comma,         /* Character ',' */
    Dot,           /* Character '.' */
    Minus,         /* Character '-' */
    Plus,          /* Character '+' */
    Semicolon,     /* Character ';' */
    ForwardSlash,  /* Character '/' */
    Asterisk,      /* Character '*' */

    /* One or two character tokens */
    Bang,         /* Character '!' */
    NotEqual,     /* Chracter '!=' */
    Assign,       /* Character '=' */
    Equal,        /* Character '==' */
    GreaterThan,  /* Character '>' */
    GreaterEqual, /* Character '>=' */
    LessThan,     /* Character '<' */
    LessEqual,    /* Character '<=' */

    /* Literals */
    Identifier,
    String,
    Number(String),

    /* Keywords */
    And,      /* Logical AND '&&' */
    Struct,   /* Class type */
    Else,     /* Conditional */
    False,    /* Boolean */
    Function, /* Function */
    For,      /* Loop */
    If,       /* Conditional */
    None,     /* NULL value */
    Or,       /* Logical OR '||' */
    Print,    /* Built-in function */
    Return,   /* Statement */
    True,     /* Boolean */
    Let,      /* Variable declaration */
    While,    /* Loop */
    Eof,      /* End of file */

    Ident(String),
}

#[derive(Debug)]
#[allow(unused)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        let mut lexer = Self {
            input: content.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Result<TokenType> {
        self.skip_whitespace();

        let tok = match self.ch {
            b'{' => TokenType::LeftCurBrace,
            b'}' => TokenType::RightCurBrace,
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b',' => TokenType::Comma,
            b';' => TokenType::Semicolon,
            b'+' => TokenType::Plus,
            b'-' => TokenType::Minus,
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    TokenType::NotEqual
                } else {
                    TokenType::Bang
                }
            },
            b'>' => TokenType::GreaterThan,
            b'<' => TokenType::LessThan,
            b'*' => TokenType::Asterisk,
            b'/' => TokenType::ForwardSlash,
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            },
            b'\"' => {
                while self.peek() != b'\"' {
                    self.read_char();
                }

                self.read_char();
                TokenType::String
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "fn" => TokenType::Function,
                    "let" => TokenType::Let,
                    "if" => TokenType::If,
                    "false" => TokenType::False,
                    "true" => TokenType::True,
                    "return" => TokenType::Return,
                    "else" => TokenType::Else,
                    "print" => TokenType::Print,
                    _ => TokenType::Ident(ident),
                });
            },
            b'0'..=b'9' => return Ok(TokenType::Number(self.read_int())),
            0 => TokenType::Eof,
            _ => unreachable!("no monkey program should contain these characters and you should feel bad about yourself")
        };

        self.read_char();
        return Ok(tok);
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}
