use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum LoxType {
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
    pub fn parse(&mut self) -> Result<Expr, ()> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    // TODO: factor out all right recursive code into generic functions
    // don't quite have the skills for this yet

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::EqualEqual | Token::BangEqual => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.comparison()?),
                    };
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = self.term()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.factor()?.clone()),
                    };
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut expr = self.factor()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::Plus | Token::Minus => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.factor()?.clone()),
                    };
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expr = self.unary()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::Slash | Token::Star => {
                    self.advance();
                    expr = Expr::Binary {
                        left: Box::new(expr.clone()),
                        operator: token.clone(),
                        right: Box::new(self.unary()?.clone()),
                    };
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        let token = self.current_token().ok_or(())?;
        match token {
            Token::Bang | Token::Minus => {
                self.advance();
                Ok(Expr::Unary {
                    operator: token.clone(),
                    expression: Box::new(self.unary()?.clone()),
                })
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        let token = self.current_token().ok_or(())?;
        self.advance();
        match token {
            Token::False => Ok(Expr::Literal {
                value: LoxType::Bool(false),
            }),
            Token::True => Ok(Expr::Literal {
                value: LoxType::Bool(true),
            }),
            Token::Nil => Ok(Expr::Literal {
                value: LoxType::Nil,
            }),
            Token::Number(value) => Ok(Expr::Literal {
                value: LoxType::Number(value),
            }),
            Token::String(value) => Ok(Expr::Literal {
                value: LoxType::String(value.clone()),
            }),
            Token::LeftParen => {
                let expression = Box::new(self.expression()?.clone());

                let Some(Token::RightParen) = self.current_token() else {
                    return Err(());
                };
                self.advance(); // eat right paren

                Ok(Expr::Grouping {
                    expression: expression,
                })
            }
            _ => Err(()),
        }
    }
}
