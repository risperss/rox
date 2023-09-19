use crate::tokenizer::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expr<T: fmt::Display> {
    Binary {
        left: Box<Expr<T>>,
        operator: Token,
        right: Box<Expr<T>>,
    },
    Grouping {
        expression: Box<Expr<T>>,
    },
    Literal {
        literal: T,
    },
    Unary {
        operator: Token,
        expression: Box<Expr<T>>,
    },
}

pub fn to_str<T: fmt::Display>(expression: &Expr<T>) -> String {
    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => format!(
            "({} {} {})",
            operator.get_lexeme(),
            to_str(&*left),
            to_str(&*right)
        ),
        Expr::Grouping { expression } => format!("(group {})", to_str(&*expression)),
        Expr::Literal { literal } => format!("{}", literal),
        Expr::Unary {
            operator,
            expression,
        } => format!("({} {})", operator.get_lexeme(), to_str(&*expression)),
    }
}

impl<T: fmt::Display> fmt::Display for Expr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", to_str(self))
    }
}
