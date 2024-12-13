use std::io::{self, Write};

use crate::{error::LoxErrorResult, interpreter::Interpreter, parser::Parser, scanner::Scanner};

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
            let _ = io::stdout().flush();
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let _ = self.run(line);
        }
    }

    fn run(&mut self, source: String) -> Result<(), LoxErrorResult> {
        // Lexical Analysis
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        // Parsing
        let mut parser = Parser::new(tokens);
        // Stop if there was a syntax error
        let statements = parser.parse()?;

        // Run Interpreter
        self.interpreter.interpret(&statements)?;

        Ok(())
    }
}
