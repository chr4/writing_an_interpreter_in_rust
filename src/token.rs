#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    // Literals are stored as strings
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
    // Choose an Illegal identifier as default
    // this should be overriden before being used
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(lookup_ident("fn"), Token::Function);
}
