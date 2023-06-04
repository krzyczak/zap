use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    println!("Welcome to Zap!");

    loop {
        print!("{}", PROMPT);
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input!");

        println!("{}", input);
    }
}
