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
        // Choosing an empty string and the ILLEGAL token as default values.
        // They are overwritten later.
        let mut tok = token::Token {
            token_type: token::TokenType::Illegal,
            literal: String::new(),
        };

        self.skip_whitespace();

        match self.ch {
            Some(ch @ '=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    tok = token::Token {
                        token_type: token::TokenType::Equal,
                        literal: String::from("=="),
                    };
                } else {
                    tok = new_token(token::TokenType::Assign, ch);
                }
            }
            Some(ch @ '+') => tok = new_token(token::TokenType::Plus, ch),
            Some(ch @ '-') => tok = new_token(token::TokenType::Minus, ch),
            Some(ch @ '!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    tok = token::Token {
                        token_type: token::TokenType::NotEqual,
                        literal: String::from("!="),
                    };
                } else {
                    tok = new_token(token::TokenType::Bang, ch);
                }
            }
            Some(ch @ '/') => tok = new_token(token::TokenType::Slash, ch),
            Some(ch @ '*') => tok = new_token(token::TokenType::Asterisk, ch),
            Some(ch @ '<') => tok = new_token(token::TokenType::LowerThan, ch),
            Some(ch @ '>') => tok = new_token(token::TokenType::GreaterThan, ch),
            Some(ch @ ';') => tok = new_token(token::TokenType::Semicolon, ch),
            Some(ch @ ',') => tok = new_token(token::TokenType::Comma, ch),
            Some(ch @ '{') => tok = new_token(token::TokenType::LeftBrace, ch),
            Some(ch @ '}') => tok = new_token(token::TokenType::RightBrace, ch),
            Some(ch @ '(') => tok = new_token(token::TokenType::LeftParenthesis, ch),
            Some(ch @ ')') => tok = new_token(token::TokenType::RightParenthesis, ch),

            Some(ch @ _) => {
                if is_letter(ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = token::lookup_ident(&tok.literal);
                    return tok;
                } else if ch.is_numeric() {
                    tok.token_type = token::TokenType::Integer;
                    tok.literal = self.read_number();
                    return tok;
                } else {
                    tok = new_token(token::TokenType::Illegal, ch);
                }
            }

            // Handle EOF
            None => {
                tok.literal = String::new();
                tok.token_type = token::TokenType::EndOfFile;
            }
        }

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

        while self.ch.expect("Error reading character").is_numeric() {
            self.read_char();
        }

        // Return new str containing the identifier
        self.input[position..self.position].to_owned()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn new_token(token_type: token::TokenType, ch: char) -> token::Token {
    token::Token {
        token_type: token_type,
        literal: ch.to_string(),
    }
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

    tests.push(token::Token {
        token_type: token::TokenType::Let,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "five".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Assign,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Let,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "ten".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Assign,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Let,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "add".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Assign,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Function,
        literal: "fn".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LeftParenthesis,
        literal: "(".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "x".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Comma,
        literal: ",".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "y".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::RightParenthesis,
        literal: ")".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LeftBrace,
        literal: "{".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "x".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Plus,
        literal: "+".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "y".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::RightBrace,
        literal: "}".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Let,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "result".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Assign,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "add".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LeftParenthesis,
        literal: "(".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "five".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Comma,
        literal: ",".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Ident,
        literal: "ten".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::RightParenthesis,
        literal: ")".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Bang,
        literal: "!".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Minus,
        literal: "-".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Slash,
        literal: "/".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Asterisk,
        literal: "*".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LowerThan,
        literal: "<".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::GreaterThan,
        literal: ">".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::If,
        literal: "if".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LeftParenthesis,
        literal: "(".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LowerThan,
        literal: "<".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::RightParenthesis,
        literal: ")".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LeftBrace,
        literal: "{".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Return,
        literal: "return".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::True,
        literal: "true".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::RightBrace,
        literal: "}".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Else,
        literal: "else".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::LeftBrace,
        literal: "{".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Return,
        literal: "return".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::False,
        literal: "false".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::RightBrace,
        literal: "}".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Equal,
        literal: "==".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::NotEqual,
        literal: "!=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Integer,
        literal: "9".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::Semicolon,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TokenType::EndOfFile,
        literal: "".to_string(),
    });

    let mut l = Lexer::new(input);
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok.token_type, t.token_type);
        assert_eq!(tok.literal, t.literal);
    }
}

#[test]
fn new_token_test() {
    let token = new_token(token::TokenType::EndOfFile, 'c');
    assert_eq!(token.token_type, token::TokenType::EndOfFile);
    assert_eq!(token.literal, "c");
}
