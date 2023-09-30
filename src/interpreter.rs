use crate::parser::{Expr, Type};
use crate::tokenizer::{CtxToken, Token};

enum RuntimeError {
    TypeError(CtxToken),
    ZeroDivisionError(CtxToken),
}

pub struct Interpreter {}

impl Interpreter {
    fn error(token: CtxToken, message: &str) {
        eprintln!("ERROR RUNTIME {}: {}", token, message);
    }

    pub fn interpret(expr: Expr) -> Result<(), ()> {
        match Interpreter::evaluate(Box::new(expr)) {
            Ok(literal) => Ok(println!("{}", literal)),
            Err(RuntimeError::TypeError(token)) => {
                Err(Interpreter::error(token, "invalid type(s) for operator"))
            }
            Err(RuntimeError::ZeroDivisionError(token)) => {
                Err(Interpreter::error(token, "zero division error"))
            }
        }
    }

    fn evaluate(expr: Box<Expr>) -> Result<Type, RuntimeError> {
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

    // wrong lint: https://github.com/rust-lang/rust/issues/41620#issuecomment-1722194944
    #[allow(illegal_floating_point_literal_pattern)]
    fn evaluate_binary(
        left: Box<Expr>,
        operator: CtxToken,
        right: Box<Expr>,
    ) -> Result<Type, RuntimeError> {
        let left = Interpreter::evaluate(left)?;
        let right = Interpreter::evaluate(right)?;

        match operator.get_token() {
            Token::Plus => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Number(left + right)),
                (Type::String(left), Type::String(right)) => {
                    Ok(Type::String(format!("{}{}", left, right)))
                }
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::Minus => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Number(left - right)),
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::Star => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Number(left * right)),
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::Slash => match (left, right) {
                (Type::Number(left), Type::Number(right)) => match right {
                    0. => Err(RuntimeError::ZeroDivisionError(operator)),
                    _ => Ok(Type::Number(left / right)),
                },
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::Greater => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left > right)),
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::GreaterEqual => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left >= right)),
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::Less => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left < right)),
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::LessEqual => match (left, right) {
                (Type::Number(left), Type::Number(right)) => Ok(Type::Bool(left <= right)),
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::EqualEqual => Ok(Type::Bool(Interpreter::is_equal(left, right))),
            Token::BangEqual => Ok(Type::Bool(!Interpreter::is_equal(left, right))),
            _ => todo!(),
        }
    }

    fn evaluate_grouping(expr: Box<Expr>) -> Result<Type, RuntimeError> {
        Interpreter::evaluate(expr)
    }

    fn evaluate_unary(operator: CtxToken, expr: Box<Expr>) -> Result<Type, RuntimeError> {
        let literal = Interpreter::evaluate(expr)?;

        match operator.get_token() {
            Token::Minus => match literal {
                Type::Number(value) => Ok(Type::Number(-value)),
                _ => Err(RuntimeError::TypeError(operator)),
            },
            Token::Bang => Ok(Type::Bool(!Interpreter::is_truthy(literal))),
            _ => panic!(),
        }
    }

    fn evaluate_ternary(
        condition: Box<Expr>,
        then: Box<Expr>,
        otherwise: Box<Expr>,
    ) -> Result<Type, RuntimeError> {
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
            Type::Number(value) => value != 0.,
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
}
