// Modules
mod error;
mod expr;
mod parser;
mod scanner;
mod token;
mod utils;
// Imports
use error::LoxError;
use parser::Parser;
use scanner::Scanner;
use std::env::args;
use std::io::{self, Write};
use utils::ast_printer::AstPrinter;

fn main() {
    // TODO: Add a way to handle print AST an arg
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file!"),
        _ => {
            eprintln!("Usage: r-lox interpreter [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let source = std::fs::read_to_string(path)?;
    match run(source) {
        Ok(_) => Ok(()),
        Err(_) => std::process::exit(65),
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        let mut line = String::new();
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut line).unwrap();
        let _ = run(line);
    }
}

fn run(source: String) -> Result<(), LoxError> {
    // Lexical Analysis
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // Parsing
    let mut parser = Parser::new(tokens);
    let expression = parser.parse()?;

    AstPrinter::new().print(&expression);

    Ok(())
}
