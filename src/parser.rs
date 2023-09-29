use crate::tokenizer::{CtxToken, Token};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxType {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

impl fmt::Display for LoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LoxType::Nil => "nil".to_string(),
            LoxType::Bool(value) => format!("{}", value),
            LoxType::String(value) => format!("\"{}\"", value.clone()),
            LoxType::Number(value) => format!("{}", value),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: CtxToken,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        value: LoxType,
    },
    Unary {
        operator: CtxToken,
        expr: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        then: Box<Expr>,
        otherwise: Box<Expr>,
    },
}

impl Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                left.to_string(),
                operator.get_token().get_lexeme(),
                right.to_string()
            ),
            Expr::Grouping { expr } => format!("({})", expr.to_string()),
            Expr::Literal { value } => format!("{}", value),
            Expr::Unary { operator, expr } => format!(
                "({} {})",
                operator.get_token().get_lexeme(),
                expr.to_string()
            ),
            Expr::Ternary {
                condition,
                then,
                otherwise,
            } => format!(
                "({} ? {} : {})",
                condition.to_string(),
                then.to_string(),
                otherwise.to_string(),
            ),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Parser {
    current: usize,
    tokens: Vec<CtxToken>,
}

impl Parser {
    pub fn new(tokens: Vec<CtxToken>) -> Self {
        Self {
            current: 0,
            tokens: tokens,
        }
    }

    fn error(&self, message: &str) {
        let token = self
            .get_current()
            .unwrap_or_else(|| self.tokens.last().unwrap().clone());
        eprintln!("ERROR PARSER {}: {}", token, message);
    }
}

impl Parser {
    fn get_current(&self) -> Option<CtxToken> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<(), ()> {
        match self.get_current() {
            Some(ctx_token) => {
                return if ctx_token.get_token() == token {
                    self.advance();
                    Ok(())
                } else {
                    Err(self.error(message))
                }
            }
            _ => Err(self.error(message)),
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.get_current() {
            match token.get_token() {
                Token::SemiColon => {
                    self.advance();
                    return;
                }
                Token::Class
                | Token::Fun
                | Token::Var
                | Token::For
                | Token::If
                | Token::While
                | Token::Print
                | Token::Return => {
                    return;
                }
                _ => self.advance(),
            }
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Expr, ()> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expr, ()> {
        let expr = self.equality()?;

        match self.get_current() {
            Some(token) => match token.get_token() {
                Token::Quest => {
                    self.advance();
                    let then = self.expression()?;
                    let _ =
                        self.consume(Token::Colon, "expected colon inside ternary expression")?;
                    let otherwise = self.expression()?;
                    Ok(Expr::Ternary {
                        condition: Box::new(expr),
                        then: Box::new(then),
                        otherwise: Box::new(otherwise),
                    })
                }
                _ => Ok(expr),
            },
            None => Ok(expr),
        }
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.get_current() {
            match token.get_token() {
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

        while let Some(token) = self.get_current() {
            match token.get_token() {
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

        while let Some(token) = self.get_current() {
            match token.get_token() {
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

        while let Some(token) = self.get_current() {
            match token.get_token() {
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
        match self.get_current() {
            Some(token) => match token.get_token() {
                Token::Bang | Token::Minus => {
                    self.advance();
                    Ok(Expr::Unary {
                        operator: token.clone(),
                        expr: Box::new(self.unary()?.clone()),
                    })
                }
                _ => self.primary(),
            },
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        match self.get_current() {
            Some(token) => match token.get_token() {
                Token::False => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: LoxType::Bool(false),
                    })
                }
                Token::True => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: LoxType::Bool(true),
                    })
                }
                Token::Nil => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: LoxType::Nil,
                    })
                }
                Token::Number(value) => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: LoxType::Number(value),
                    })
                }
                Token::String(value) => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: LoxType::String(value.clone()),
                    })
                }
                Token::LeftParen => {
                    self.advance();
                    let expr = Box::new(self.expression()?.clone());
                    let _ = self.consume(Token::RightParen, "missing closing paren")?;

                    Ok(Expr::Grouping { expr: expr })
                }
                Token::EqualEqual
                | Token::BangEqual
                | Token::Less
                | Token::LessEqual
                | Token::Greater
                | Token::GreaterEqual
                | Token::Plus => Err(self.error("missing expression on left side of operator")),
                _ => Err(self.error("missing expression")),
            },
            _ => Err(self.error("missing expression")),
        }
    }
}
