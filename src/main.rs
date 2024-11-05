// Modules
mod error;
mod scanner;
mod token;
// Imports
use error::LoxError;
use scanner::Scanner;
use std::env::args;
use std::io::{self, Write};

fn main() {
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

fn run_file(path: &String) -> io::Result<()> {
    let source = std::fs::read_to_string(path)?;
    match run(source) {
        Ok(_) => Ok(()),
        Err(err) => {
            err.report("");
            std::process::exit(65)
        }
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

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
