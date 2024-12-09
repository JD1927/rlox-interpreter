use std::fmt;

use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum ControlFlowSignal {
    Break,
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    Lexical,
    Parser,
    Interpreter,
    SystemError,
    ControlFlow(ControlFlowSignal),
}

#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
    pub error_type: Option<ErrorType>,
}

impl LoxError {
    pub fn error(line: usize, message: &str) -> LoxError {
        LoxError {
            line,
            message: message.to_string(),
            error_type: None,
        }
    }

    pub fn report_column(&self, column: &str) {
        let error = format!(
            "[Line {}: Col {}] - Error: {}",
            self.line, column, self.message
        );
        eprintln!("{}", error)
    }

    pub fn report_location(&self, location: &str) {
        let error = format!("[Line {}] - Error{}: {}", self.line, location, self.message);
        eprintln!("{}", error)
    }

    pub fn system_error(message: &str) -> LoxError {
        let mut lox_error = LoxError::error(0, message);
        lox_error.error_type = Some(ErrorType::SystemError);
        eprintln!("Error: {message}");
        lox_error
    }

    pub fn lexical_error(line: usize, message: &str) -> LoxError {
        let mut lox_error = LoxError::error(line, message);
        lox_error.error_type = Some(ErrorType::Lexical);
        lox_error
    }

    pub fn parse_error(token: Token, message: &str) -> LoxError {
        let mut lox_error = LoxError::error(token.line, message);
        lox_error.error_type = Some(ErrorType::Parser);
        match token.is(TokenType::Eof) {
            true => lox_error.report_location(" at end"),
            false => lox_error.report_location(format!(" at '{}'", token.lexeme).as_str()),
        }
        lox_error
    }

    pub fn interpreter_error(line: usize, message: &str) -> LoxError {
        let mut lox_error = LoxError::error(line, message);
        lox_error.error_type = Some(ErrorType::Interpreter);
        lox_error.report_location("");
        lox_error
    }

    pub fn break_signal(line: usize, message: &str) -> LoxError {
        let mut lox_error = LoxError::error(line, message);
        lox_error.error_type = Some(ErrorType::ControlFlow(ControlFlowSignal::Break));
        lox_error
    }

    pub fn is_control_break(&self) -> bool {
        matches!(
            self.error_type,
            Some(ErrorType::ControlFlow(ControlFlowSignal::Break))
        )
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
