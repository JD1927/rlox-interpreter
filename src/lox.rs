use std::io::{self, Write};

use crate::{error::LoxError, interpreter::Interpreter, parser::Parser, scanner::Scanner};

pub struct Lox {
    pub interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new(),
        }
    }
    pub fn run_file(&mut self, path: &str) -> io::Result<()> {
        let source = std::fs::read_to_string(path)?;
        match self.run(source) {
            Ok(_) => Ok(()),
            Err(_) => std::process::exit(65),
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut line).unwrap();
            let _ = self.run(line);
        }
    }

    fn run(&mut self, source: String) -> Result<(), LoxError> {
        // Lexical Analysis
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        // Parsing
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        // Stop if there was a syntax error
        if expression.is_err() {
            return Err(expression.err().unwrap());
        }

        match expression {
            Ok(expr) => {
                let _ = self.interpreter.interpret(&expr);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
