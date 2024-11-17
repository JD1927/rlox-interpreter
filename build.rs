use generate_ast::*;
use std::env::args;
use std::io::{self};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 | 2 => {
            let output_dir = match args.get(1) {
                Some(value) => value.clone(),
                None => "src".to_string(),
            };
            define_ast(
                output_dir,
                "Expr".to_string(),
                &[
                    "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
                    "Grouping : Box<Expr> expression".to_string(),
                    "Literal  : Object value".to_string(),
                    "Unary    : Token operator, Box<Expr> right".to_string(),
                    "Comma    : Box<Expr> left, Box<Expr> right".to_string(),
                    "Ternary  : Box<Expr> condition, Box<Expr> then_branch, Box<Expr> else_branch".to_string(),
                ],
            )
        }
        _ => {
            eprintln!("Usage: generate-ast <output_directory>");
            std::process::exit(64);
        }
    }
}
