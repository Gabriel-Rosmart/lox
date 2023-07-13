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
    Number,

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

#[derive(Debug)]
#[allow(unused)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
}

pub struct Lexer {
    input: Vec<char>,
    read_position: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            input: content.chars().collect(),
            read_position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.is_at_end() {
            return None;
        }

        let tok = match self.input[self.read_position] {
            '{' => TokenType::LeftCurBrace,
            '}' => TokenType::RightCurBrace,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '!' => {
                if self.peek() == '=' {
                    return self.read_double_operator(TokenType::NotEqual);
                } else {
                    TokenType::Bang
                }
            }
            '>' => {
                if self.peek() == '=' {
                    return self.read_double_operator(TokenType::GreaterEqual);
                } else {
                    TokenType::GreaterThan
                }
            }
            '<' => {
                if self.peek() == '=' {
                    return self.read_double_operator(TokenType::LessEqual);
                } else {
                    TokenType::LessThan
                }
            }
            '*' => TokenType::Asterisk,
            '/' => {
                if self.peek() == '/' {
                    let comment = self.read_comment();
                    let token_type = TokenType::Comment;
                    return Some(Token {
                        lexeme: comment,
                        token_type,
                    });
                } else {
                    TokenType::ForwardSlash
                }
            }
            '=' => {
                if self.peek() == '=' {
                    return self.read_double_operator(TokenType::Equal);
                } else {
                    TokenType::Assign
                }
            }
            '\"' => {
                return Some(Token {
                    lexeme: self.read_string(),
                    token_type: TokenType::String,
                });
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_ident();
                let token_type = match ident.as_str() {
                    "fn" => TokenType::Function,
                    "let" => TokenType::Let,
                    "if" => TokenType::If,
                    "false" => TokenType::False,
                    "true" => TokenType::True,
                    "return" => TokenType::Return,
                    "else" => TokenType::Else,
                    "while" => TokenType::While,
                    "None" => TokenType::None,
                    "print" => TokenType::Print,
                    _ => TokenType::Identifier,
                };

                return Some(Token {
                    lexeme: ident,
                    token_type,
                });
            }
            '0'..='9' => {
                let ident = self.read_number();
                let token_type = TokenType::Number;
                return Some(Token {
                    lexeme: ident,
                    token_type,
                });
            }
            _ => unreachable!("Monke should learn to type"),
        };

        let token = self.input[self.read_position].to_string();
        self.advance_cursor();
        return Some(Token {
            lexeme: token,
            token_type: tok,
        });
    }

    /// Check if we reached the end of file
    fn is_at_end(&self) -> bool {
        self.read_position >= self.input.len()
    }

    /// Get next character if possible
    fn peek(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input[self.read_position + 1];
        }
    }

    // Get the next offset 2 character if possible
    fn peek_next(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input[self.read_position + 2];
        }
    }

    // Advance cursor by one char
    fn advance_cursor(&mut self) {
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.input[self.read_position].is_whitespace() {
            self.advance_cursor();
        }
    }

    // Reads operators like '<='
    fn read_double_operator(&mut self, token_type: TokenType) -> Option<Token> {
        self.advance_cursor();
        let token = Some(Token {
            lexeme: self.input[self.read_position - 1..self.read_position + 1]
                .iter()
                .collect(),
            token_type,
        });
        self.advance_cursor();
        return token;
    }

    fn read_comment(&mut self) -> String {
        let pos = self.read_position;
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance_cursor();
        }

        self.advance_cursor();
        self.advance_cursor();

        return self.input[pos..self.read_position - 1].iter().collect();
    }

    fn read_ident(&mut self) -> String {
        let pos = self.read_position;
        while !self.is_at_end()
            && (self.input[self.read_position].is_alphanumeric()
                || self.input[self.read_position] == '_')
        {
            self.advance_cursor();
        }

        return self.input[pos..self.read_position].iter().collect();
    }

    fn read_string(&mut self) -> String {
        let pos = self.read_position;
        while !self.is_at_end() && self.peek() != '\"' {
            self.advance_cursor();
        }

        let content = self.input[(pos + 1)..self.read_position + 1]
            .iter()
            .collect::<String>();

        self.advance_cursor();
        self.advance_cursor();

        content
    }

    fn read_number(&mut self) -> String {
        let pos = self.read_position;
        while !self.is_at_end() && self.peek().is_numeric() {
            self.advance_cursor();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance_cursor();

            while !self.is_at_end() && self.peek().is_numeric() {
                self.advance_cursor();
            }
        }

        self.advance_cursor();

        return self.input[pos..self.read_position].iter().collect();
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
