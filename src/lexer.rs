use token;
use token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>, // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LowerThan,
            Some('>') => Token::GreaterThan,
            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,

            Some(ch @ _) => {
                if is_letter(ch) {
                    // TODO
                    let literal = self.read_identifier();
                    token::lookup_ident(&literal)
                } else if ch.is_numeric() {
                    // TODO
                    Token::Integer(self.read_number())
                } else {
                    Token::Illegal // TODO: Maybe we need ch here, to display a nice error message later?
                }
            }

            // Handle EOF
            None => Token::EndOfFile,
        };

        // TODO: Do not read all the time here.
        self.read_char();
        return tok;
    }

    fn skip_whitespace(&mut self) {
        // Loop read_char() until non-whitespace is found
        while match self.ch {
            Some(ch) => ch.is_whitespace(),
            _ => false,
        } {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input
                .chars()
                .nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    // TODO: Use peekable
    fn peek_char_eq(&mut self, ch: char) -> bool {
        // Return false on EOF
        if self.read_position >= self.input.len() {
            return false;
        }

        let peek_ch = self.input
            .chars()
            .nth(self.read_position)
            .unwrap(); // TODO: Unwrap sucks

        peek_ch == ch
    }

    // TODO: Not sure whether String is advisable here. Couldn't find anything that clones
    // self.input into a &str.
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        // TODO: While peek.is_numeric?
        while is_letter(self.ch.expect("Error reading character")) {
            self.read_char();
        }

        // Return new str containing the identifier
        self.input[position..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        // TODO: While peek.is_numeric?
        while self.ch.expect("Error reading character").is_numeric() {
            self.read_char();
        }

        // Return new str containing the number
        self.input[position..self.position].to_owned()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[test]
fn next_token_test() {

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let input = "let five = 5;
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
";

    let tests = vec![
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Integer("5".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Integer("10".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LeftParenthesis,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LeftParenthesis,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer("5".to_string()),
        Token::Semicolon,
        Token::Integer("5".to_string()),
        Token::LowerThan,
        Token::Integer("10".to_string()),
        Token::GreaterThan,
        Token::Integer("5".to_string()),
        Token::Semicolon,
        Token::If,
        Token::LeftParenthesis,
        Token::Integer("5".to_string()),
        Token::LowerThan,
        Token::Integer("10".to_string()),
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBrace,
        Token::Integer("10".to_string()),
        Token::Equal,
        Token::Integer("10".to_string()),
        Token::Semicolon,
        Token::Integer("10".to_string()),
        Token::NotEqual,
        Token::Integer("9".to_string()),
        Token::Semicolon,
        Token::EndOfFile,
    ];

    let mut l = Lexer::new(input);
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok, t);
    }
}
