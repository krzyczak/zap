#[derive(Default)]
struct Lexer {
    input: Vec<u8>,
    position: usize,
    // next_position: i32,
    // ch: char
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {input: input.into_bytes(), ..Default::default()}
    }

    fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token::Eof;
        } else {
            let ch = self.input[self.position];
            self.position += 1;
            match ch {
                b'=' => Token::Assign,
                b'+' => Token::Plus,
                b'(' => Token::LParen,
                b')' => Token::RParen,
                b'{' => Token::LBrace,
                b'}' => Token::RBrace,
                b',' => Token::Comma,
                b';' => Token::Semicolon,
                0 => Token::Eof,
                _ => panic!("Not yet implemented"),
            }
        }
    }

    fn read_char() {
        todo!()
    }
}

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
    Eof
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asdf() {
        let mut lexer = Lexer::new("=".into());
        let result = lexer.next_token();
        assert_eq!(result, Token::Assign);
    }

    #[test]
    fn qwerty() {
        let tests = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof
        ];

        let mut lexer = Lexer::new("=+(){},;".into());
        for test in tests {
            assert_eq!(test, lexer.next_token());
        }

    }
}
