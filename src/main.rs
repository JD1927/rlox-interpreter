// Modules
mod environment;
mod error;
mod expr;
mod interpreter;
mod lox;
mod object;
mod parser;
mod scanner;
mod stmt;
mod token;
mod utils;
// Imports
use std::env::args;

use lox::Lox;

fn main() {
    // TODO: Add a way to handle print AST an arg
    let args: Vec<String> = args().collect();
    let mut lox = Lox::new();
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]).expect("Could not run file!"),
        _ => {
            eprintln!("Usage: r-lox interpreter [script]");
            std::process::exit(64);
        }
    }
}
