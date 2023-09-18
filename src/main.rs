use std::env;
use std::fmt;
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
struct CtxToken {
    token: Token,
    line: usize,
    lexeme: String,
}

impl CtxToken {
    fn new(token: Token, line: usize) -> Self {
        Self {
            token: token,
            line: line,
            lexeme: "".to_string(),
        }
    }
}

impl fmt::Display for CtxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{:?}", self.line, self.token)
    }
}

struct Scanner {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    fn report(&self, location: String, message: String) {
        eprintln!("[line {0}] Error {location} where: {message}", self.line);
    }

    fn error(&mut self, message: String) {
        self.has_error = true;
        self.report("".to_string(), message);
    }

    fn get_current(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.get(self.current).copied()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current + 1).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 2).copied()
    }

    fn matches_next(&mut self, target: char) -> bool {
        match self.peek() {
            Some(next) => {
                if target == next {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn match_if_next(&mut self, second_char: char, if_matches: Token, otherwise: Token) -> Token {
        if self.matches_next(second_char) {
            let _ = self.advance();
            if_matches
        } else {
            otherwise
        }
    }

    fn lookup_keyword(literal: &str) -> Option<Token> {
        match literal {
            "and" => Some(Token::And),
            "class" => Some(Token::Class),
            "else" => Some(Token::Else),
            "false" => Some(Token::False),
            "fun" => Some(Token::Fun),
            "for" => Some(Token::For),
            "if" => Some(Token::If),
            "nil" => Some(Token::Nil),
            "or" => Some(Token::Or),
            "print" => Some(Token::Print),
            "return" => Some(Token::Return),
            "super" => Some(Token::Super),
            "this" => Some(Token::This),
            "true" => Some(Token::True),
            "var" => Some(Token::Var),
            "while" => Some(Token::While),
            _ => None,
        }
    }

    fn scan(&mut self) -> Result<Vec<CtxToken>, ()> {
        let mut tokens: Vec<CtxToken> = Vec::new();

        while let Some(c) = self.get_current() {
            let token: Option<Token> = match c {
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
                '!' => Some(self.match_if_next('=', Token::BangEqual, Token::Bang)),
                '=' => Some(self.match_if_next('=', Token::EqualEqual, Token::Equal)),
                '<' => Some(self.match_if_next('=', Token::LessEqual, Token::Less)),
                '>' => Some(self.match_if_next('=', Token::GreaterEqual, Token::Greater)),
                // comments
                '/' => match self.peek() {
                    Some('/') => {
                        // a comment goes until the end of the line or file
                        loop {
                            break match self.advance() {
                                Some('\n') => {
                                    self.line += 1;
                                    None
                                }
                                None => None,
                                _ => continue,
                            };
                        }
                    }
                    Some('*') => {
                        let mut depth = 1;
                        let _ = self.advance(); // eat star
                        while depth != 0 {
                            match self.advance() {
                                Some('*') => {
                                    if self.matches_next('/') {
                                        depth -= 1;
                                        let _ = self.advance(); // eat the '/'
                                    }
                                }
                                Some('/') => {
                                    if self.matches_next('*') {
                                        depth += 1;
                                        let _ = self.advance(); // eat the '*'
                                    }
                                }
                                Some('\n') => self.line += 1,
                                Some(_) => (),
                                None => break, // TODO: are unclosed multiline comments allowed?
                            };
                        }
                        None
                    }
                    _ => Some(Token::Slash),
                },
                ' ' | '\r' | '\t' => None,
                '\n' => {
                    self.line += 1;
                    None
                }
                // literals
                '"' => loop {
                    match self.advance() {
                        None => {
                            self.error("unterminated string".to_string());
                            break None;
                        }
                        Some('"') => {
                            let literal: String =
                                self.chars[self.start + 1..self.current].iter().collect();
                            break Some(Token::String(literal.to_string()));
                        }
                        Some('\n') => {
                            self.line += 1;
                        }
                        _ => (),
                    }
                },
                '0'..='9' => {
                    'outer: while let Some(next_char) = self.peek() {
                        match next_char {
                            '0'..='9' => {
                                let _ = self.advance();
                            }
                            '.' => {
                                match self.peek_next() {
                                    Some('0'..='9') => {
                                        // digits after dot i.e. the number is a float
                                        let _ = self.advance(); // consume the dot
                                        while let Some(next_digit) = self.peek() {
                                            match next_digit {
                                                '0'..='9' => {
                                                    let _ = self.advance();
                                                }
                                                _ => break 'outer,
                                            };
                                        }
                                    }
                                    _ => break 'outer,
                                }
                            }
                            _ => break 'outer,
                        }
                    }
                    let literal: String = self.chars[self.start..=self.current].iter().collect();
                    let value: f64 = literal.parse::<f64>().unwrap();
                    Some(Token::Number(value))
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    while let Some(next_char) = self.peek() {
                        match next_char {
                            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                                let _ = self.advance();
                            }
                            _ => break,
                        };
                    }

                    let literal: String = self.chars[self.start..=self.current].iter().collect();
                    Some(Scanner::lookup_keyword(&literal).unwrap_or(Token::Identifier(literal)))
                }
                _ => {
                    self.error("unexpected character".to_string());
                    None
                }
            };
            if let Some(token) = token {
                tokens.push(CtxToken::new(token, self.line));
            }
            let _ = self.advance();
            self.start = self.current;
        }
        tokens.push(CtxToken::new(Token::Eof, self.line));

        if self.has_error {
            Err(())
        } else {
            Ok(tokens)
        }
    }
}

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
