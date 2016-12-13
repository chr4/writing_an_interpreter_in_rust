use std::io::{self, BufRead, Write};

pub mod lexer;
pub mod token;

// Start a custom repl
fn main() {
    let stdin = io::stdin();

    loop {
        // Stdout needs to be flushed, due to missing newline
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let mut lexer = lexer::Lexer::new(&mut line);

        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == token::Token::EndOfFile {
                break;
            }
        }
    }
}
