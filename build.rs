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
                &output_dir,
                "Stmt".to_string(),
                &[
                    "Block      : Vec<Stmt> statements".to_string(),
                    "Class      : Token name, Vec<Stmt> methods".to_string(),
                    "Expression : Box<Expr> expression".to_string(),
                    "Function   : Token name, Vec<Token> params, Vec<Stmt> body".to_string(),
                    "If         : Box<Expr> condition, Box<Stmt> then_branch, Option<Box<Stmt>> else_branch"
                        .to_string(),
                    "Print      : Box<Expr> expression".to_string(),
                    "Return     : Token keyword, Option<Box<Expr>> value".to_string(),
                    "Var        : Token name, Option<Box<Expr>> initializer".to_string(),
                    "While      : Box<Expr> condition, Box<Stmt> body".to_string(),
                    "Break      : Token keyword".to_string(),
                ],
                false
            )?;
            define_ast(
                &output_dir,
                "Expr".to_string(),
                &[
                    "Assign   : usize uid, Token name, Box<Expr> value".to_string(),
                    "Binary   : usize uid, Box<Expr> left, Token operator, Box<Expr> right".to_string(),
                    "Call     : usize uid, Box<Expr> callee, Token paren, Vec<Expr> arguments".to_string(),
                    "Get      : usize uid, Box<Expr> object, Token name".to_string(),
                    "Grouping : usize uid, Box<Expr> expression".to_string(),
                    "Literal  : usize uid, Object value".to_string(),
                    "Logical  : usize uid, Box<Expr> left, Token operator, Box<Expr> right".to_string(),
                    "Set      : usize uid, Box<Expr> object, Token name, Box<Expr> value".to_string(),
                    "This     : usize uid, Token keyword".to_string(),
                    "Unary    : usize uid, Token operator, Box<Expr> right".to_string(),
                    "Ternary  : usize uid, Box<Expr> condition, Box<Expr> then_branch, Box<Expr> else_branch"
                        .to_string(),
                    "Variable : usize uid, Token name".to_string(),
                ],
                true
            )?;
            Ok(())
        }
        _ => {
            eprintln!("Usage: generate-ast <output_directory>");
            std::process::exit(64);
        }
    }
}
