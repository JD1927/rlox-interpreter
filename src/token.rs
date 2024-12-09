use std::fmt;

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn is(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?} - line {}",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character Tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    Semicolon,
    Slash,
    Star,
    Question,

    // One or Two Character Tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Break,
    // End of line
    Eof,
}
