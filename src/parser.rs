use crate::tokenizer::Token;
use std::fmt;

#[derive(Debug, Clone)]
enum LoxType {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LoxType,
    },
    Unary {
        operator: Token,
        expression: Box<Expr>,
    },
}

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens: tokens,
        }
    }
}

impl Parser {
    fn current_token(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) {
        self.current += 1;
    }
}

impl Parser {
    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Some(token) = self.current_token() {
            match token {
                Token::EqualEqual | Token::BangEqual => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.comparison()),
                    };
                }
                _ => break,
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(token) = self.current_token() {
            match token {
                Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.factor().clone()),
                    };
                }
                _ => break,
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(token) = self.current_token() {
            match token {
                Token::Plus | Token::Minus => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.factor().clone()),
                    };
                }
                _ => break,
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Some(token) = self.current_token() {
            match token {
                Token::Slash | Token::Star => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.unary().clone()),
                    };
                }
                _ => break,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let token = self.current_token().unwrap();
        match token {
            Token::Bang | Token::Minus => {
                self.advance();
                Expr::Unary {
                    operator: token.clone(),
                    expression: Box::new(self.unary().clone()),
                }
            },
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.current_token().unwrap();
        self.advance();
        match token {
            Token::False => Expr::Literal {
                value: LoxType::Bool(false),
            },
            Token::True => Expr::Literal {
                value: LoxType::Bool(true),
            },
            Token::Nil => Expr::Literal {
                value: LoxType::Nil,
            },
            Token::Number(value) => Expr::Literal {
                value: LoxType::Number(value),
            },
            Token::String(value) => Expr::Literal {
                value: LoxType::String(value.clone()),
            },
            Token::LeftParen => {
                let expression = Box::new(self.expression().clone());

                let Some(Token::RightParen) = self.current_token() else {
                    panic!();
                };
                self.advance();// eat right paren

                Expr::Grouping { expression: expression }
            }
            _ => panic!(),
        }
    }
}
