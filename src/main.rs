pub mod lexer;
pub mod token;

use std::io::{self, BufRead, Write};
use token::Token;
use lexer::Lexer;

// Start a custom repl
fn main() {
    let stdin = io::stdin();

    loop {
        // Stdout needs to be flushed, due to missing newline
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let mut lexer = Lexer::new(&mut line);

        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == Token::EndOfFile {
                break;
            }
        }
    }
}
