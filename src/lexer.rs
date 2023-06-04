struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0
        };

        lexer.read_char();
        lexer
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

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'0'..=b'9' => {
                return Token::Int(self.read_int());
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return match self.read_ident().as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    other => Token::Ident(other.to_string())
                }
            },
            0 => Token::Eof,
            ch => Token::Illegal((ch as char).to_string()),
        };

        self.read_char();

        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Token {
    Assign,
    Plus,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    Let,
    Function,
    Ident(String),

    Int(String),

    Eof,
    Illegal(String),
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
           Token::Eof,
        ];

        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        let mut lexer = Lexer::new(input.into());
        for token in tokens_in_order {
            let lexer_token = lexer.next_token();
            println!("Expected Token: {:?}, Lexer token: {:?}", token, lexer_token);
            assert_eq!(token, lexer_token);
        }
    }
}
