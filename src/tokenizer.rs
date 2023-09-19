use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
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

impl Token {
    pub fn get_lexeme(&self) -> String {
        match self {
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::Comma => ".to_string(),".to_string(),
            Token::Dot => ".".to_string(),
            Token::Minus => "-".to_string(),
            Token::Plus => "+".to_string(),
            Token::SemiColon => ";".to_string(),
            Token::Slash => "/".to_string(),
            Token::Star => "*".to_string(),
            Token::Bang => "!".to_string(),
            Token::BangEqual => "!=".to_string(),
            Token::Equal => "=".to_string(),
            Token::EqualEqual => "==".to_string(),
            Token::Greater => ">".to_string(),
            Token::GreaterEqual => ">=".to_string(),
            Token::Less => "<".to_string(),
            Token::LessEqual => "<=".to_string(),
            Token::Identifier(literal) => literal.clone(),
            Token::String(literal) => literal.clone(),
            Token::Number(value) => value.to_string(),
            Token::And => "and".to_string(),
            Token::Class => "class".to_string(),
            Token::Else => "else".to_string(),
            Token::False => "false".to_string(),
            Token::Fun => "fun".to_string(),
            Token::For => "for".to_string(),
            Token::If => "if".to_string(),
            Token::Nil => "nil".to_string(),
            Token::Or => "or".to_string(),
            Token::Print => "print".to_string(),
            Token::Return => "return".to_string(),
            Token::Super => "super".to_string(),
            Token::This => "this".to_string(),
            Token::True => "true".to_string(),
            Token::Var => "var".to_string(),
            Token::While => "while".to_string(),
            Token::Eof => "EOF".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CtxToken {
    token: Token,
    line: usize,
}

impl CtxToken {
    fn new(token: Token, line: usize) -> Self {
        Self {
            token: token,
            line: line,
        }
    }
}

impl fmt::Display for CtxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{:?}", self.line, self.token)
    }
}

pub struct Scanner {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            has_error: false,
        }
    }

    fn report(&self, message: String) {
        eprintln!("[{0}:{1}]\tERROR: {message}", self.line, self.column);
    }

    fn error(&mut self, message: String) {
        self.has_error = true;
        self.report(message);
    }

    fn get_current(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.column += 1;
        self.chars.get(self.current).copied()
    }

    fn advance_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current + 1).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 2).copied()
    }

    fn matches_next(&self, target: char) -> bool {
        self.peek() == Some(target)
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

    pub fn scan(&mut self) -> Result<Vec<CtxToken>, ()> {
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
                                    self.advance_line();
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
                                Some('\n') => self.advance_line(),
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
                    self.advance_line();
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
                            self.advance_line();
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
                    Scanner::lookup_keyword(&literal).or_else(|| Some(Token::Identifier(literal)))
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
