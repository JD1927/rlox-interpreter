use crate::{
    object::Object,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum LoxErrorResult {
    SystemError { message: String },
    Lexical { line: usize, message: String },
    Parser { token: Token, message: String },
    Interpreter { line: usize, message: String },
    Resolver { token: Token, message: String },
    ControlFlowBreak,
    ControlFlowReturn { value: Object },
}

impl LoxErrorResult {
    pub fn system_error(message: &str) -> LoxErrorResult {
        let error = LoxErrorResult::SystemError {
            message: message.to_string(),
        };
        error.report();
        error
    }

    pub fn lexical_error(line: usize, message: &str) -> LoxErrorResult {
        LoxErrorResult::Lexical {
            line,
            message: message.to_string(),
        }
    }

    pub fn parse_error(token: Token, message: &str) -> LoxErrorResult {
        LoxErrorResult::Parser {
            token,
            message: message.to_string(),
        }
    }

    pub fn interpreter_error(line: usize, message: &str) -> LoxErrorResult {
        LoxErrorResult::Interpreter {
            line,
            message: message.to_string(),
        }
    }

    pub fn resolver_error(token: Token, message: &str) -> LoxErrorResult {
        let error = LoxErrorResult::Resolver {
            token,
            message: message.to_string(),
        };
        error.report();
        error
    }

    pub fn break_signal() -> LoxErrorResult {
        let error = LoxErrorResult::ControlFlowBreak {};
        error.report();
        error
    }

    pub fn return_signal(value: Object) -> LoxErrorResult {
        let error = LoxErrorResult::ControlFlowReturn { value };
        error.report();
        error
    }

    pub fn is_control_break(&self) -> bool {
        matches!(&self, LoxErrorResult::ControlFlowBreak { .. })
    }

    pub fn report(&self) {
        match self {
            LoxErrorResult::SystemError { message } => {
                eprintln!("System error: {message}");
            }
            LoxErrorResult::Lexical { line, message } => {
                eprintln!("[Line {}] - Error: {}", line, message)
            }
            LoxErrorResult::Parser { token, message }
            | LoxErrorResult::Resolver { token, message } => {
                if token.is(TokenType::Eof) {
                    eprintln!("[Line {}] - Error at end: {}", token.line, message)
                } else {
                    eprintln!(
                        "[Line {}] - Error at '{}': {}",
                        token.line, token.lexeme, message
                    )
                };
            }
            LoxErrorResult::Interpreter { line, message } => {
                eprintln!("[Line {}] - Error: {}", line, message)
            }
            LoxErrorResult::ControlFlowBreak | LoxErrorResult::ControlFlowReturn { .. } => {}
        }
    }
}
