// use crate::lexer::Token;
//
// pub trait Node {
//     fn token_literal(&self) -> String; // TODO: if the nodes are similar enough move the
//                                        // function here as default
// }
//
// struct Program {
//     statemets: Vec<LetStatement>
// }
//
// impl Program {
//     // Seems this will only be used for debugging.
//     fn token_literal(&self) -> String {
//         if self.statemets.len() > 0 {
//             self.statemets[0].token_literal()
//         } else {
//             "".to_string()
//         }
//     }
// }
//
// struct LetStatement {
//     token: Token,
//     name: Identifier, // make it as reference and add lifetime specfiers?
//     // value: Box<dyn Expression>,
// }
//
// impl LetStatement {
//     pub fn token_literal(&self) -> String {
//         self.token.to_string()
//     }
// }
//
// // I think we could just use Token::Ident here.
// struct Identifier {
//     token: Token,
//     value: String,
// }
//
// impl Identifier {
//     pub fn token_literal(&self) -> String {
//         self.token.to_string()
//     }
// }

use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Token> = vec![];

        // while let token = self.lexer.next_token() {
        //     match token {
        //         Token::Eof => return Program { statements },
        //         token => statements.append(token),
        //     }
        //     println!("----------- {:?}", token);
        // }

        loop {
            let token = self.lexer.next_token();
            match token {
                Token::Eof => break,
                token => statements.push(token)
            }
        }

        Program { statements }
        
        // loop {
        //     if token = self.lexer.next_token() {}
        //     if token == Token::Eof {
        //         break;
        //     }
        //     println!("{:?}", token);
        // }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    statements: Vec<Token>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_statemet() {
        let lexer = Lexer::new("let five = 5;".into());
        let mut parser = Parser::new(lexer);
        let result = parser.parse();
        let statement = Program {
            statements: vec![
                Token::Let,
                Token::Ident("five".into()),
                Token::Assign,
                Token::Int("5".into()),
                Token::Semicolon,
            ],
        };
        assert_eq!(result, statement);
    }
}
