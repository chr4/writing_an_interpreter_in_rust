#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(char),
    EndOfFile,

    // Identifiers + literals
    Ident(String),
    Integer(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LowerThan,
    GreaterThan,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Default for Token {
    fn default() -> Self {
        Token::Illegal(' ')
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    // Note: The Go version uses a map[string]Token to select idents
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(String::from(ident)),
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(lookup_ident("fn"), Token::Function);
}
