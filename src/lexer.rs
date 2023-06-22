#[derive(Debug, PartialEq)]
pub enum Token {
    Assign,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Bang,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Comma,
    Semicolon,

    Let,
    Function,
    If,
    Else,
    True,
    False,
    Return,
    Ident(String),

    Int(String),

    Eof,
    Illegal(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            },
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::ForwardSlash,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            b'0'..=b'9' => {
                return Token::Int(self.read_int());
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return match self.read_ident().as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    other => Token::Ident(other.to_string())
                }
            },
            0 => Token::Eof,
            ch => Token::Illegal((ch as char).to_string()),
        };

        self.read_char();

        token
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

    fn peek(&self) -> u8 {
        if self.read_position < self.input.len() {
            self.input[self.read_position]
        } else {
            0
        }
    }

    fn read_int(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[start_pos..self.position]).to_string()
    }

    fn read_ident(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[start_pos..self.position]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn illegal_token() {
        let mut lexer = Lexer::new("`".into());
        let result = lexer.next_token();
        assert_eq!(result, Token::Illegal("`".into()));
    }

    #[test]
    fn equals() {
        let mut lexer = Lexer::new("=".into());
        let result = lexer.next_token();
        assert_eq!(result, Token::Assign);
    }

    #[test]
    fn a_few_simple_tokens() {
        let tokens_in_order = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,

        ];

        let mut lexer = Lexer::new("=+(){},;".into());
        for token in tokens_in_order {
            assert_eq!(token, lexer.next_token());
        }
    }

    #[test]
    fn large_int() {
        let expected = Token::Int("623923".into());
        let input = "623923";

        let mut lexer = Lexer::new(input.into());
        assert_eq!(expected, lexer.next_token());
    }

    #[test]
    fn small_int() {
        let expected = Token::Int("5".into());
        let input = "5";

        let mut lexer = Lexer::new(input.into());
        assert_eq!(expected, lexer.next_token());
    }

    #[test]
    fn long_script() {
        let tokens_in_order = [
           Token::Let,
           Token::Ident("five".into()),
           Token::Assign,
           Token::Int(String::from("5")),
           Token::Semicolon,
           Token::Let,
           Token::Ident("ten".to_string()),
           Token::Assign,
           Token::Int("10".to_string()),
           Token::Semicolon,
           Token::Let,
           Token::Ident("add".to_string()),
           Token::Assign,
           Token::Function,
           Token::LParen,
           Token::Ident("x".to_string()),
           Token::Comma,
           Token::Ident("y".to_string()),
           Token::RParen,
           Token::LBrace,
           Token::Ident("x".to_string()),
           Token::Plus,
           Token::Ident("y".to_string()),
           Token::Semicolon,
           Token::RBrace,
           Token::Semicolon,
           Token::Let,
           Token::Ident("result".to_string()),
           Token::Assign,
           Token::Ident("add".to_string()),
           Token::LParen,
           Token::Ident("five".to_string()),
           Token::Comma,
           Token::Ident("ten".to_string()),
           Token::RParen,
           Token::Semicolon,
           Token::Bang,
           Token::Minus,
           Token::ForwardSlash,
           Token::Asterisk,
           Token::Int("5".into()),
           Token::Semicolon,
           Token::Int("5".into()),
           Token::LessThan,
           Token::Int("10".into()),
           Token::GreaterThan,
           Token::Int("5".into()),
           Token::Semicolon,
           Token::If,
           Token::LParen,
           Token::Int("5".into()),
           Token::LessThan,
           Token::Int("10".into()),
           Token::RParen,
           Token::LBrace,
           Token::Return,
           Token::True,
           Token::Semicolon,
           Token::RBrace,
           Token::Else,
           Token::LBrace,
           Token::Return,
           Token::False,
           Token::Semicolon,
           Token::RBrace,
           Token::Int(10.to_string()),
           Token::Equal,
           Token::Int(10.to_string()),
           Token::Semicolon,
           Token::Int(10.to_string()),
           Token::NotEqual,
           Token::Int(9.to_string()),
           Token::Semicolon,
           Token::Eof,
        ];

        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;

        let mut lexer = Lexer::new(input.into());
        for token in tokens_in_order {
            let lexer_token = lexer.next_token();
            debug_assert_eq!(token, lexer_token);
        }
    }
}
