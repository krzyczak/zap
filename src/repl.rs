use std::io::{self, Write};

use zap::{lexer::{Lexer, Token}, parser::Parser};

const PROMPT: &str = ">> ";

pub fn start() {
    println!("Welcome to Zap!");

    loop {
        print!("{}", PROMPT);
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input!");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        parser.parse();

        // loop {
        //     let token = lexer.next_token();
        //     if token == Token::Eof {
        //         break;
        //     }
        //     println!("{:?}", token);
        // }
    }
}
