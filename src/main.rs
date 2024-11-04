use core::str;
use std::env::args;
use std::io::{self, Write};

struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn error(&mut self, line: usize, message: String) {}
}

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
    let buffer = std::fs::read_to_string(path)?;
    run(buffer.as_str());
    Ok(())
}

fn run_prompt() {
    loop {
        print!(">> ");
        let mut line = String::new();
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut line).unwrap();
        run(&line);
    }
    // print!(">> ");
    // let stdin = io::stdin();
    // for line in stdin.lock().lines().map_while(Result::ok) {
    //     if line.is_empty() {
    //         break;
    //     }
    //     run(line.as_str());
    // }
}
fn run(source: &str) {
    // Lexical Analysis
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
