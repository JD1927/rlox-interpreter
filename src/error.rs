use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum ErrorType {
    Lexical,
    Parser,
}

#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
    pub error_type: ErrorType,
}

impl LoxError {
    pub fn new(line: usize, message: &str) -> LoxError {
        LoxError {
            line,
            message: message.to_string(),
            error_type: ErrorType::Lexical,
        }
    }
    pub fn report_column(&self, column: &str) {
        let error = format!(
            "[line {}] - Error: {} => at column {}",
            self.line, self.message, column
        );
        eprintln!("{}", error)
    }

    pub fn report_location(&self, location: &str) {
        let error = format!("[line {}] - Error{}: {}", self.line, location, self.message);
        eprintln!("{}", error)
    }

    pub fn parse_error(token: Token, message: &str) -> LoxError {
        let mut lox_error = LoxError::new(token.line, message);
        lox_error.error_type = ErrorType::Parser;
        match token.token_type == TokenType::Eof {
            true => lox_error.report_location(" at end"),
            false => lox_error.report_location(format!(" at '{}'", token.lexeme).as_str()),
        }
        lox_error
    }
}
