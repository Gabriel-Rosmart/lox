#[derive(Debug)]
#[allow(unused)]
pub enum TokenKind {
    /* Single character tokens */
    OpenParen,    /* Character '(' */
    CloseParen,   /* Character ')' */
    OpenBrace,    /* Character '{' */
    CloseBrace,   /* Character '}' */
    Comma,        /* Character ',' */
    Dot,          /* Character '.' */
    Minus,        /* Character '-' */
    Plus,         /* Character '+' */
    Semicolon,    /* Character ';' */
    ForwardSlash, /* Character '/' */
    Asterisk,     /* Character '*' */

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
    Identifier(String),
    QuotedString(String),
    Integer(usize),
    Decimal(f64),

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

    /* Special */
    Comment,
    Eof, /* End of file */
}

pub struct Lexer {
    input: Vec<char>,
    cursor: usize,
}

impl Lexer {
    pub fn new(file_contents: String) -> Self {
        Self {
            input: file_contents.chars().collect(),
            cursor: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<TokenKind> {
        self.skip_whitespace();

        if self.input.is_empty() {
            return None;
        }

        let (token, length) = match self.input[0] {
            '{' => (TokenKind::OpenBrace, 1),
            '}' => (TokenKind::CloseBrace, 1),
            '(' => (TokenKind::OpenParen, 1),
            ')' => (TokenKind::CloseParen, 1),
            ',' => (TokenKind::Comma, 1),
            ';' => (TokenKind::Semicolon, 1),
            '+' => (TokenKind::Plus, 1),
            '-' => (TokenKind::Minus, 1),
            '*' => (TokenKind::Asterisk, 1),
            '.' => (TokenKind::Dot, 1),
            '=' => {
                if self.peek() == '=' {
                    (TokenKind::Equal, 2)
                } else {
                    (TokenKind::Assign, 1)
                }
            }
            '!' => {
                if self.peek() == '=' {
                    (TokenKind::NotEqual, 2)
                } else {
                    (TokenKind::Bang, 1)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    (TokenKind::GreaterEqual, 2)
                } else {
                    (TokenKind::GreaterThan, 1)
                }
            }
            '<' => {
                if self.peek() == '=' {
                    (TokenKind::LessEqual, 2)
                } else {
                    (TokenKind::LessThan, 1)
                }
            }
            '/' => {
                if self.peek() == '/' {
                    self.read_comment()
                } else {
                    (TokenKind::ForwardSlash, 1)
                }
            }
            '\"' => self.read_quoted_string(),
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
            _ => panic!("Token invalid"),
        };

        self.chomp(length);
        self.cursor = 0;
        Some(token)
    }

    fn advance_cursor(&mut self) {
        if self.cursor < self.input.len() {
            self.cursor += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        let mut index: usize = 0;
        while (index < self.input.len()) && self.input[index].is_whitespace() {
            index += 1;
        }

        self.input = self.input[index..].to_vec();
    }

    fn peek(&self) -> char {
        if self.cursor >= self.input.len() {
            '\0'
        } else {
            self.input[self.cursor + 1]
        }
    }

    fn chomp(&mut self, length: usize) {
        self.input = self.input[length..].to_vec();
    }

    fn read_number(&mut self) -> (TokenKind, usize) {
        while self.peek().is_numeric() {
            self.advance_cursor();
        }

        if self.peek() == '.' {
            self.advance_cursor();
            while self.peek().is_numeric() {
                self.advance_cursor();
            }

            let number = self.input[0..self.cursor + 1].iter().collect::<String>();
            (
                TokenKind::Decimal(number.parse::<f64>().unwrap()),
                self.cursor + 1,
            )
        } else {
            let number = self.input[0..self.cursor + 1].iter().collect::<String>();
            (
                TokenKind::Integer(number.parse::<usize>().unwrap()),
                self.cursor + 1,
            )
        }
    }

    fn read_identifier(&mut self) -> (TokenKind, usize) {
        while self.cursor < self.input.len() && self.input[self.cursor].is_alphanumeric() {
            self.cursor += 1;
        }

        let token = self.input[0..self.cursor].iter().collect::<String>();
        let token_len = token.len();

        let kind = match token.as_str() {
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "false" => TokenKind::False,
            "true" => TokenKind::True,
            "return" => TokenKind::Return,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "None" => TokenKind::None,
            "or" => TokenKind::Or,
            "and" => TokenKind::And,
            "print" => TokenKind::Print,
            _ => TokenKind::Identifier(token),
        };

        (kind, token_len)
    }

    fn read_quoted_string(&mut self) -> (TokenKind, usize) {
        while self.cursor < self.input.len() && self.peek() != '\"' {
            self.advance_cursor();
        }

        let qstring = self.input[1..self.cursor + 1].iter().collect::<String>();
        (TokenKind::QuotedString(qstring), self.cursor + 2)
    }

    fn read_comment(&mut self) -> (TokenKind, usize) {
        while self.cursor < self.input.len() && self.input[self.cursor] != '\n' {
            self.advance_cursor();
        }

        (TokenKind::Comment, self.cursor + 1)
    }
}

impl Iterator for Lexer {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
