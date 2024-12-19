// Modules
mod environment;
mod error;
mod expr;
mod interpreter;
mod lox_callable;
mod lox_class;
mod lox_function;
mod lox_native_function;
mod object;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;
mod utils;
// Imports
use std::env::args;

use std::io::{self, Write};

use crate::{interpreter::Interpreter, parser::Parser, resolver::Resolver, scanner::Scanner};

fn main() {
    // TODO: Add a way to handle print AST an arg
    let args: Vec<String> = args().collect();
    let mut interpreter = Interpreter::new();
    match args.len() {
        1 => run_prompt(&mut interpreter),
        2 => run_file(&args[1], &mut interpreter).expect("Could not run file!"),
        _ => {
            eprintln!("Usage: r-lox interpreter [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str, interpreter: &mut Interpreter) -> io::Result<()> {
    let source = std::fs::read_to_string(path)?;
    run(source, interpreter);
    Ok(())
}

fn run_prompt(interpreter: &mut Interpreter) {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        run(line, interpreter);
    }
}

fn run(source: String, interpreter: &mut Interpreter) {
    // Lexical Analysis

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // Parsing
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    if parser.had_error {
        return; // Stop if there was a parse error.
    }

    // Resolving
    let mut resolver = Resolver::new(interpreter);
    resolver.resolve(&statements);

    if resolver.had_error {
        return; // Stop if there was a resolution error.
    }
    // Run Interpreter
    interpreter.interpret(&statements);
}
