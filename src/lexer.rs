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
            token_type: token::ILLEGAL,
            literal: String::new(),
        };

        self.skip_whitespace();

        match self.ch {
            Some(ch @ '=') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = token::Token {
                        token_type: token::EQ,
                        literal: format!("{}{}", ch, self.ch.unwrap()),
                    };
                } else {
                    tok = new_token(token::ASSIGN, ch);
                }
            }
            Some(ch @ '+') => tok = new_token(token::PLUS, ch),
            Some(ch @ '-') => tok = new_token(token::MINUS, ch),
            Some(ch @ '!') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = token::Token {
                        token_type: token::NOT_EQ,
                        literal: format!("{}{}", ch, self.ch.unwrap()),
                    };
                } else {
                    tok = new_token(token::BANG, ch);
                }
            }
            Some(ch @ '/') => tok = new_token(token::SLASH, ch),
            Some(ch @ '*') => tok = new_token(token::ASTERISK, ch),
            Some(ch @ '<') => tok = new_token(token::LT, ch),
            Some(ch @ '>') => tok = new_token(token::GT, ch),
            Some(ch @ ';') => tok = new_token(token::SEMICOLON, ch),
            Some(ch @ ',') => tok = new_token(token::COMMA, ch),
            Some(ch @ '{') => tok = new_token(token::LBRACE, ch),
            Some(ch @ '}') => tok = new_token(token::RBRACE, ch),
            Some(ch @ '(') => tok = new_token(token::LPAREN, ch),
            Some(ch @ ')') => tok = new_token(token::RPAREN, ch),

            Some(ch @ _) => {
                if is_letter(ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = token::lookup_ident(&tok.literal);
                    return tok;
                } else if is_digit(ch) {
                    tok.token_type = token::INT;
                    tok.literal = self.read_number();
                    return tok;
                } else {
                    tok = new_token(token::ILLEGAL, ch);
                }
            }

            // Handle EOF
            None => {
                tok.literal = String::new();
                tok.token_type = token::EOF;
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

    // TODO: There's a peekable() function:
    //       https://doc.rust-lang.org/std/str/struct.Chars.html
    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            self.ch = None;
            // TODO: Use Option here, so we can return EOF
            return ' ';
        }

        return self.input
            .chars()
            .nth(self.read_position)
            .unwrap(); // TODO: Unwrap sucks
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

fn new_token(token_type: token::TokenType, ch: char) -> token::Token {
    token::Token {
        token_type: token_type,
        literal: ch.to_string(),
    }
}

#[test]
fn new_token_test() {
    let token = new_token(token::EOF, 'c');
    assert_eq!(token.token_type, token::EOF);
    assert_eq!(token.literal, "c");
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
        token_type: token::LET,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "five".to_string(),
    });
    tests.push(token::Token {
        token_type: token::ASSIGN,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LET,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "ten".to_string(),
    });
    tests.push(token::Token {
        token_type: token::ASSIGN,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LET,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "add".to_string(),
    });
    tests.push(token::Token {
        token_type: token::ASSIGN,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::FUNCTION,
        literal: "fn".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LPAREN,
        literal: "(".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "x".to_string(),
    });
    tests.push(token::Token {
        token_type: token::COMMA,
        literal: ",".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "y".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RPAREN,
        literal: ")".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LBRACE,
        literal: "{".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "x".to_string(),
    });
    tests.push(token::Token {
        token_type: token::PLUS,
        literal: "+".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "y".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RBRACE,
        literal: "}".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LET,
        literal: "let".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "result".to_string(),
    });
    tests.push(token::Token {
        token_type: token::ASSIGN,
        literal: "=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "add".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LPAREN,
        literal: "(".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "five".to_string(),
    });
    tests.push(token::Token {
        token_type: token::COMMA,
        literal: ",".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IDENT,
        literal: "ten".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RPAREN,
        literal: ")".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::BANG,
        literal: "!".to_string(),
    });
    tests.push(token::Token {
        token_type: token::MINUS,
        literal: "-".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SLASH,
        literal: "/".to_string(),
    });
    tests.push(token::Token {
        token_type: token::ASTERISK,
        literal: "*".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LT,
        literal: "<".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::GT,
        literal: ">".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::IF,
        literal: "if".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LPAREN,
        literal: "(".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "5".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LT,
        literal: "<".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RPAREN,
        literal: ")".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LBRACE,
        literal: "{".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RETURN,
        literal: "return".to_string(),
    });
    tests.push(token::Token {
        token_type: token::TRUE,
        literal: "true".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RBRACE,
        literal: "}".to_string(),
    });
    tests.push(token::Token {
        token_type: token::ELSE,
        literal: "else".to_string(),
    });
    tests.push(token::Token {
        token_type: token::LBRACE,
        literal: "{".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RETURN,
        literal: "return".to_string(),
    });
    tests.push(token::Token {
        token_type: token::FALSE,
        literal: "false".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::RBRACE,
        literal: "}".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::EQ,
        literal: "==".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "10".to_string(),
    });
    tests.push(token::Token {
        token_type: token::NOT_EQ,
        literal: "!=".to_string(),
    });
    tests.push(token::Token {
        token_type: token::INT,
        literal: "9".to_string(),
    });
    tests.push(token::Token {
        token_type: token::SEMICOLON,
        literal: ";".to_string(),
    });
    tests.push(token::Token {
        token_type: token::EOF,
        literal: "".to_string(),
    });

    let mut l = Lexer::new(input);
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok.token_type, t.token_type);
        assert_eq!(tok.literal, t.literal);
    }
}
