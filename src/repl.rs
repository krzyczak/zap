use std::io::{self, Write};

use zap::lexer::{Lexer, Token};

const PROMPT: &str = ">> ";

pub fn start() {
    println!("Welcome to Zap!");

    loop {
        print!("{}", PROMPT);
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input!");

        let mut lexer = Lexer::new(input);

        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                break;
            }
            println!("{:?}", token);
        }
    }
}
