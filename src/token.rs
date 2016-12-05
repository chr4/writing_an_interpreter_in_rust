pub type TokenType = &'static str;

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

// Identifiers + literals
pub const IDENT: TokenType = "IDENT"; // add, foobar, x, y, ...
pub const INT: TokenType = "INT";   // 1343456

// Operators
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";

pub const LT: TokenType = "<";
pub const GT: TokenType = ">";

pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";

// Delimiters
pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

// Keywords
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "IF";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    // Note: The Go version uses a map[string]TokenType to select idents
    match ident {
        "fn" => FUNCTION,
        "let" => LET,
        "true" => TRUE,
        "false" => FALSE,
        "if" => IF,
        "else" => ELSE,
        "return" => RETURN,
        _ => IDENT,
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(lookup_ident("fn"), FUNCTION);
}
