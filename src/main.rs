use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::process;

mod tokenizer;
use crate::tokenizer::{Scanner, Token};

mod parser;
use crate::parser::Expr;

fn run(source: String) -> Result<(), ()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan()?;

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}

fn run_file(file_path: String) {
    let mut f = File::open(file_path).expect("failed to open file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)
        .expect("failed to read file contents");

    run(buffer).unwrap();
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        let _ = run(line);
    }
}

fn run_debug() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::Minus,
            expression: Box::new(Expr::Literal { literal: 123.0 }),
        }),
        operator: Token::Star,
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal { literal: 45.67 }),
        }),
    };
    println!("{}", expression);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: cargo run [script]");
        process::exit(64);
    } else if args.len() == 2 {
        if args[1] == "DEBUG" {
            run_debug();
        } else {
            run_file(args[1].clone());
        }
    } else {
        run_prompt();
    }
}
