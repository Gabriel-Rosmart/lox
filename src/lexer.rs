use crate::error::{ErrorBag, LoxError};

#[derive(Debug, Clone)]
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
    Percentage,   /* Character '%' */

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
    Integer(isize),
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
    Comment, /* Comments in the form // */
    Eof,     /* End of file */
    Invalid, /* Helper token to detect errors */
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Position,
}

pub struct Lexer<'a> {
    input: Vec<char>,
    cursor: usize,
    span: Position,
    pub error_bag: &'a mut ErrorBag,
}

impl<'a> Lexer<'a> {
    pub fn new(file_contents: String, error_bag: &'a mut ErrorBag) -> Self {
        Self {
            input: file_contents.chars().collect(),
            cursor: 0,
            span: Position { line: 1, column: 1 },
            error_bag,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.input.is_empty() {
            return None;
        }

        let (tokenkind, length) = match self.input[0] {
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
            '%' => (TokenKind::Percentage, 1),
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
            _ => {
                self.error_bag.errors.push(LoxError::LexerError(format!(
                    "Use of invalid token: \x1b[32m{}\x1b[0m at line {}, column {}",
                    self.input[0], self.span.line, self.span.column
                )));
                (TokenKind::Invalid, 1)
            }
        };

        self.chomp(length);
        self.cursor = 0;
        self.span.column += length;
        Some(Token {
            kind: tokenkind,
            span: Position {
                line: self.span.line,
                column: self.span.column - length,
            },
        })
    }

    fn advance_cursor(&mut self) {
        if self.cursor < self.input.len() {
            self.cursor += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        let mut index: usize = 0;
        while (index < self.input.len()) && self.input[index].is_whitespace() {
            self.span.column += 1;
            if self.input[index] == '\n' {
                self.span.line += 1;
                self.span.column = 1;
            }
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
                TokenKind::Integer(number.parse::<isize>().unwrap()),
                self.cursor + 1,
            )
        }
    }

    fn read_identifier(&mut self) -> (TokenKind, usize) {
        while self.cursor < self.input.len()
            && (self.input[self.cursor].is_alphanumeric() || self.input[self.cursor] == '_')
        {
            self.cursor += 1;
        }

        let token = self.input[0..self.cursor].iter().collect::<String>();
        let token_len = token.len();

        let kind = match token.as_str() {
            "fn" => TokenKind::Function,
            "struct" => TokenKind::Struct,
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
        while self.cursor < self.input.len() - 1 && self.peek() != '\"' {
            self.advance_cursor();
        }

        if self.cursor == self.input.len() - 1 {
            self.error_bag.errors.push(LoxError::LexerError(format!(
                "Unterminated string at line {}",
                self.span.line
            )));
            return (TokenKind::Invalid, self.cursor + 1);
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

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "\x1b[32m[Bang]\x1b[0m"),
            Self::OpenParen => write!(f, "\x1b[32m[Open Parentheses]\x1b[0m"),
            Self::CloseParen => write!(f, "\x1b[32m[Closing Parentheses]\x1b[0m"),
            Self::OpenBrace => write!(f, "\x1b[32m[Open Brace]\x1b[0m"),
            Self::CloseBrace => write!(f, "\x1b[32m[Closing Brace]\x1b[0m"),
            Self::None => write!(f, "\x1b[32m[None]\x1b[0m"),
            Self::If => write!(f, "\x1b[32m[If]\x1b[0m"),
            Self::Or => write!(f, "\x1b[32m[Or]\x1b[0m"),
            Self::And => write!(f, "\x1b[32m[And]\x1b[0m"),
            Self::Dot => write!(f, "\x1b[32m[Dot]\x1b[0m"),
            Self::For => write!(f, "\x1b[32m[For]\x1b[0m"),
            Self::While => write!(f, "\x1b[32m[While]\x1b[0m"),
            Self::Let => write!(f, "\x1b[32m[Let]\x1b[0m"),
            Self::True => write!(f, "\x1b[32m[True]\x1b[0m"),
            Self::False => write!(f, "\x1b[32m[False]\x1b[0m"),
            Self::Function => write!(f, "\x1b[32m[Function]\x1b[0m"),
            Self::ForwardSlash => write!(f, "\x1b[32m[ForwardSlash]\x1b[0m"),
            Self::Else => write!(f, "\x1b[32m[Else]\x1b[0m"),
            Self::Comma => write!(f, "\x1b[32m[Comma]\x1b[0m"),
            Self::Minus => write!(f, "\x1b[32m[Minus]\x1b[0m"),
            Self::Plus => write!(f, "\x1b[32m[Plus]\x1b[0m"),
            Self::Equal => write!(f, "\x1b[32m[Equal]\x1b[0m"),
            Self::LessThan => write!(f, "\x1b[32m[LessThan]\x1b[0m"),
            Self::LessEqual => write!(f, "\x1b[32m[LessEqual]\x1b[0m"),
            Self::GreaterThan => write!(f, "\x1b[32m[GreaterThan]\x1b[0m"),
            Self::GreaterEqual => write!(f, "\x1b[32m[GreaterEqual]\x1b[0m"),
            Self::Assign => write!(f, "\x1b[32m[Assign]\x1b[0m"),
            Self::Asterisk => write!(f, "\x1b[32m[Asterisk]\x1b[0m"),
            Self::NotEqual => write!(f, "\x1b[32m[NotEqual]\x1b[0m"),
            Self::Return => write!(f, "\x1b[32m[Return]\x1b[0m"),
            Self::Struct => write!(f, "\x1b[32m[Struct]\x1b[0m"),
            Self::Print => write!(f, "\x1b[32m[Print]\x1b[0m"),
            Self::Percentage => write!(f, "\x1b[32m[Percentage]\x1b[0m"),
            Self::Semicolon => write!(f, "\x1b[32m[Semicolon]\x1b[0m"),
            _ => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Lexer, TokenKind};
    use crate::error::ErrorBag;

    #[test]
    fn lexer_recognizes_invalid_tokens() {
        let invalid_program = "let x = &y".to_string(); // & not a valid token
        let mut error_bag = ErrorBag { errors: vec![] };

        let lexer = Lexer::new(invalid_program, &mut error_bag);

        let _tokens: Vec<_> = lexer
            .into_iter()
            .filter(|token| match token.kind {
                TokenKind::Comment | TokenKind::Invalid => false,
                _ => true,
            })
            .collect();

        assert!(!error_bag.errors.is_empty());
    }
}
