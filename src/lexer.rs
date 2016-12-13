use token;

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

    pub fn next_token(&mut self) -> token::Token {
        // Unlike Go, Rust doesn't initialize the variables by default. 
        let mut tok = token::Token::default();

        self.skip_whitespace();

        match self.ch {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    tok = token::Token::Equal;                    
                } else {
                    tok = token::Token::Assign;;
                }
            }
            Some('+') => tok = token::Token::Plus,
            Some('-') => tok = token::Token::Minus,
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    tok = token::Token::NotEqual;
                } else {
                    tok = token::Token::Bang;
                }
            }
            Some('/') => tok = token::Token::Slash,
            Some('*') => tok = token::Token::Asterisk,
            Some('<') => tok = token::Token::LowerThan,
            Some('>') => tok = token::Token::GreaterThan,
            Some(';') => tok = token::Token::Semicolon,
            Some(',') => tok = token::Token::Comma,
            Some('{') => tok = token::Token::LeftBrace,
            Some('}') => tok = token::Token::RightBrace,
            Some('(') => tok = token::Token::LeftParenthesis,
            Some(')') => tok = token::Token::RightParenthesis,

            Some(ch @ _) => {
                if is_letter(ch) {
                    tok = token::lookup_ident(&self.read_identifier());
                    return tok;
                } else if is_digit(ch) {
                    return token::Token::Integer(self.read_number());
                } else {
                    tok = token::Token::Illegal(ch);
                }
            }

            // Handle EOF
            None => {
                tok = token::Token::EndOfFile;
            }
        }

        self.read_char();
        return tok;
    }

    fn skip_whitespace(&mut self) {
        // Loop read_char() until non-whitespace is found
        loop {
            match self.ch {
                Some(' ') => self.read_char(),
                Some('\t') => self.read_char(),
                Some('\n') => self.read_char(),
                Some('\r') => self.read_char(),
                Some(_) => return,
                None => return, // EOF is handled by caller
            }
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

        while is_letter(self.ch.expect("Error reading character")) {
            self.read_char();
        }

        // Return new str containing the identifier
        self.input[position..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while is_digit(self.ch.expect("Error reading character")) {
            self.read_char();
        }

        // Return new str containing the identifier
        self.input[position..self.position].to_owned()
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
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
    let mut tests = Vec::new();
    tests.push(token::Token::Let);
    tests.push(token::Token::Ident("five".to_string()));
    tests.push(token::Token::Assign);
    tests.push(token::Token::Integer("5".to_string())); 
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::Let);
    tests.push(token::Token::Ident("ten".to_string()));
    tests.push(token::Token::Assign);
    tests.push(token::Token::Integer("10".to_string()));
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::Let);
    tests.push(token::Token::Ident("add".to_string()));
    tests.push(token::Token::Assign);
    tests.push(token::Token::Function);
    tests.push(token::Token::LeftParenthesis);
    tests.push(token::Token::Ident("x".to_string()));
    tests.push(token::Token::Comma);
    tests.push(token::Token::Ident("y".to_string()));
    tests.push(token::Token::RightParenthesis);
    tests.push(token::Token::LeftBrace);
    tests.push(token::Token::Ident("x".to_string()));
    tests.push(token::Token::Plus);
    tests.push(token::Token::Ident("y".to_string()));
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::RightBrace);
    tests.push(token::Token::Semicolon); 
    tests.push(token::Token::Let);
    tests.push(token::Token::Ident("result".to_string())); 
    tests.push(token::Token::Assign);
    tests.push(token::Token::Ident("add".to_string()));
    tests.push(token::Token::LeftParenthesis);
    tests.push(token::Token::Ident("five".to_string()));
    tests.push(token::Token::Comma);
    tests.push(token::Token::Ident("ten".to_string()));
    tests.push(token::Token::RightParenthesis);
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::Bang); 
    tests.push(token::Token::Minus);
    tests.push(token::Token::Slash);
    tests.push(token::Token::Asterisk);
    tests.push(token::Token::Integer("5".to_string()));
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::Integer("5".to_string()));
    tests.push(token::Token::LowerThan);
    tests.push(token::Token::Integer("10".to_string())); 
    tests.push(token::Token::GreaterThan);
    tests.push(token::Token::Integer("5".to_string()));
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::If); 
    tests.push(token::Token::LeftParenthesis); 
    tests.push(token::Token::Integer("5".to_string()));
    tests.push(token::Token::LowerThan);
    tests.push(token::Token::Integer("10".to_string()));
    tests.push(token::Token::RightParenthesis);
    tests.push(token::Token::LeftBrace);
    tests.push(token::Token::Return);
    tests.push(token::Token::True);
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::RightBrace);
    tests.push(token::Token::Else);
    tests.push(token::Token::LeftBrace);
    tests.push(token::Token::Return);
    tests.push(token::Token::False);
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::RightBrace);
    tests.push(token::Token::Integer("10".to_string()));
    tests.push(token::Token::Equal);
    tests.push(token::Token::Integer("10".to_string()));
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::Integer("10".to_string()));
    tests.push(token::Token::NotEqual);
    tests.push(token::Token::Integer("9".to_string()));
    tests.push(token::Token::Semicolon);
    tests.push(token::Token::EndOfFile);

    let mut l = Lexer::new(input);
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok, t);
    }
}

