use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Token {
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    LT,
    GT,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NotEQ,
    ILLEGAL(char),
    IDENT(Rc<str>),
    INT(Rc<str>),
    EOF,
}
