struct Lexer {}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {}
    }

    fn next_token(&self) -> Token {
        Token::Assign(String::from("="))
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Assign(String)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asdf() {
        let lexer = Lexer::new("=".into());
        let result = lexer.next_token();
        assert_eq!(result, Token::Assign("=".into()));
    }
}
