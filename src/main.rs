use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::process;

#[derive(Debug, Clone)]
enum Token {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // literals
    Identifier(String),
    String(String),
    Number(f64),
    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // --
    Eof,
}

#[derive(Debug, Clone)]
struct ContextualToken {
    token: Token,
    line: usize,
    lexeme: String,
}

impl ContextualToken {
    fn new(token: Token, line: usize) -> Self {
        Self {
            token: token,
            line: line,
            lexeme: "".to_string(),
        }
    }
}

fn scan_tokens(source: String) -> Result<Vec<ContextualToken>, ()> {
    let keywords = HashMap::from([
        ("and".to_string(), Token::And),
        ("class".to_string(), Token::Class),
        ("else".to_string(), Token::Else),
        ("false".to_string(), Token::False),
        ("fun".to_string(), Token::Fun),
        ("for".to_string(), Token::For),
        ("if".to_string(), Token::If),
        ("nil".to_string(), Token::Nil),
        ("or".to_string(), Token::Or),
        ("print".to_string(), Token::Print),
        ("return".to_string(), Token::Return),
        ("super".to_string(), Token::Super),
        ("this".to_string(), Token::This),
        ("true".to_string(), Token::True),
        ("var".to_string(), Token::Var),
        ("while".to_string(), Token::While),
    ]);
    let chars: Vec<char> = source.chars().collect();

    let mut tokens = Vec::new();
    let mut start: usize = 0;
    let mut current: usize = 0;
    let mut line: usize = 1;
    let mut has_error = false;

    while let Some(first_char) = chars.get(current) {
        let token: Option<Token> = match first_char {
            // single char lexemes
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '-' => Some(Token::Minus),
            '+' => Some(Token::Plus),
            ';' => Some(Token::SemiColon),
            '*' => Some(Token::Star),
            // single or double char lexemes
            '!' => match chars.get(current + 1) {
                Some('=') => {
                    current += 1;
                    Some(Token::BangEqual)
                }
                _ => Some(Token::Bang),
            },
            '=' => match chars.get(current + 1) {
                Some('=') => {
                    current += 1;
                    Some(Token::EqualEqual)
                }
                _ => Some(Token::Equal),
            },
            '<' => match chars.get(current + 1) {
                Some('=') => {
                    current += 1;
                    Some(Token::LessEqual)
                }
                _ => Some(Token::Less),
            },
            '>' => match chars.get(current + 1) {
                Some('=') => {
                    current += 1;
                    Some(Token::GreaterEqual)
                }
                _ => Some(Token::Greater),
            },
            // comments
            '/' => match chars.get(current + 1) {
                Some('/') => {
                    current += 1;
                    // a comment goes until the end of the line
                    while let Some(next_char) = chars.get(current + 1) {
                        match next_char {
                            '\n' => break,
                            _ => current += 1,
                        };
                    }
                    None
                }
                _ => Some(Token::Slash),
            },
            ' ' | '\r' | '\t' => None,
            '\n' => {
                line += 1;
                None
            }
            // literals
            '"' => loop {
                current += 1;
                let Some(next_char) = chars.get(current) else {
                    has_error = true;
                    error(line, "unterminated string".to_string());
                    break None;
                };
                match next_char {
                    '"' => {
                        let literal: String = chars[start + 1..current].iter().collect();
                        break Some(Token::String(literal.to_string()));
                    }
                    '\n' => {
                        line += 1;
                    }
                    _ => (),
                };
            },
            '0'..='9' => {
                'outer: while let Some(next_char) = chars.get(current + 1) {
                    match next_char {
                        '0'..='9' => current += 1,
                        '.' => {
                            if let Some(next_next_char) = chars.get(current + 2) {
                                match next_next_char {
                                    '0'..='9' => {
                                        current += 1;
                                        while let Some(next_char) = chars.get(current + 1) {
                                            match next_char {
                                                '0'..='9' => current += 1,
                                                _ => break 'outer,
                                            };
                                        }
                                    }
                                    _ => break,
                                }
                            }
                        }
                        _ => break,
                    }
                }
                let literal: String = chars[start..=current].iter().collect();
                let value: f64 = literal.parse::<f64>().unwrap();
                Some(Token::Number(value))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while let Some(next_char) = chars.get(current + 1) {
                    match next_char {
                        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => current += 1,
                        _ => break,
                    };
                }
                let literal: String = chars[start..=current].iter().collect();
                match keywords.get(&literal) {
                    Some(token) => Some((*token).clone()),
                    None => Some(Token::Identifier(literal)),
                }
            }
            _ => {
                has_error = true;
                error(line, "unexpected character".to_string());
                None
            }
        };
        if let Some(token) = token {
            tokens.push(ContextualToken::new(token, line));
        }
        current += 1;
        start = current;
    }

    if has_error {
        Err(())
    } else {
        Ok(tokens)
    }
}

fn report(line: usize, location: String, message: String) {
    eprintln!("[line {line}] Error {location} where: {message}");
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

fn run(source: String) -> Result<(), ()> {
    let tokens = scan_tokens(source)?;

    for token in tokens {
        println!("{:?}", token);
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: cargo run [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let file_path = args[1].to_string();
        run_file(file_path);
    } else {
        run_prompt();
    }
}
