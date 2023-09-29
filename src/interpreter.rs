use crate::parser::{Expr, Type};
use crate::tokenizer::{CtxToken, Token};

pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(expr: Box<Expr>) -> Result<Type, ()> {
        match *expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => Interpreter::evaluate_binary(left, operator, right),
            Expr::Grouping { expr } => Interpreter::evaluate_grouping(expr),
            Expr::Literal { value } => Ok(value),
            Expr::Unary { operator, expr } => Interpreter::evaluate_unary(operator, expr),
            Expr::Ternary {
                condition,
                then,
                otherwise,
            } => Interpreter::evaluate_ternary(condition, then, otherwise),
        }
    }

    fn evaluate_binary(left: Box<Expr>, operator: CtxToken, right: Box<Expr>) -> Result<Type, ()> {
        let left = Interpreter::evaluate(left)?;
        let right = Interpreter::evaluate(right)?;

        match operator.get_token() {
            Token::Plus => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Number(left + right)),
                (Type::String(left), Type::String(right)) => {
                    Ok(Type::String(format!("{}{}", left, right)))
                }
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::Minus => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Number(left - right)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::Star => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Number(left * right)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::Slash => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Number(left / right)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::Greater => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left > right)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::GreaterEqual => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left >= right)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::Less => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left < right)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::LessEqual => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left <= right)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::EqualEqual => Ok(Type::Bool(Interpreter::is_equal(left, right))),
            Token::BangEqual => Ok(Type::Bool(!Interpreter::is_equal(left, right))),
            _ => todo!(),
        }
    }

    fn evaluate_grouping(expr: Box<Expr>) -> Result<Type, ()> {
        Interpreter::evaluate(expr)
    }

    fn evaluate_unary(operator: CtxToken, expr: Box<Expr>) -> Result<Type, ()> {
        let literal = Interpreter::evaluate(expr)?;

        match operator.get_token() {
            Token::Minus => match literal {
                Type::Number(value) => Ok(Type::Number(-value)),
                _ => Err(Interpreter::error(operator, "invalid type(s) for operator")),
            },
            Token::Bang => Ok(Type::Bool(!Interpreter::is_truthy(literal))),
            _ => panic!(),
        }
    }

    fn evaluate_ternary(
        condition: Box<Expr>,
        then: Box<Expr>,
        otherwise: Box<Expr>,
    ) -> Result<Type, ()> {
        match Interpreter::is_truthy(Interpreter::evaluate(condition)?) {
            true => Interpreter::evaluate(then),
            false => Interpreter::evaluate(otherwise),
        }
    }

    fn is_truthy(value: Type) -> bool {
        match value {
            Type::Nil => false,
            Type::Bool(value) => value,
            Type::String(value) => value != "",
            Type::Number(value) => value != 0.0,
        }
    }

    fn is_equal(left: Type, right: Type) -> bool {
        match (left, right) {
            (Type::Nil, Type::Nil) => true,
            (Type::Bool(left), Type::Bool(right)) => left == right,
            (Type::String(left), Type::String(right)) => left == right,
            (Type::Number(left), Type::Number(right)) => left == right,
            _ => false,
        }
    }

    fn error(token: CtxToken, message: &str) {
        eprintln!("ERROR RUNTIME {}: {}", token, message);
    }
}
