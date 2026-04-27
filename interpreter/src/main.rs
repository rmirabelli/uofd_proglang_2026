#[derive(Clone)]
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Subtract(Vec<Expression>),
    Multiply(Vec<Expression>),
    Variable(String),
    Number(i32)
}

pub struct Environment {
    key: String,
    value: Expression
}

impl Environment {
    fn value_for_key(self: &Environment, key: &String) -> &Expression {
        if &self.key == key {
            &self.value
        } else {
            panic!("Key not found in environment");
        }
    }
    fn new() -> Environment {
        Environment { key: String::from(""), value: Expression::Number(0) }
    }
}

pub fn evaluate_addition(add: &Expression, environment: &Environment) -> i32 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0, |total, next| total + evaluate(next, environment))
    } else {
        panic!("Addition not provided");
    }
}

pub fn evaluate_multiplication(mult: &Expression, environment: &Environment) -> i32 {
    if let Expression::Multiply(expressions) = mult {
        let iter = expressions.iter();
        iter.fold(1,|total, next| total * evaluate(next, environment))
    } else {
        panic!("Multiply not provided");
    }
}

pub fn evaluate_subtraction(sub: &Expression, environment: &Environment) -> i32 {
    if let Expression::Subtract(expressions) = sub {
            let mut iter = expressions.iter();
            let first = iter.next().unwrap();
            iter.fold(evaluate(first, environment), |total, next| total - evaluate(next, environment))
    } else {
        panic!("Not subtraction");
    }
}

fn evaluate(expression: &Expression, environment: &Environment) -> i32 {
    match expression {
        Expression::Add(_) => evaluate_addition(expression, environment),
        Expression::Subtract(_) => evaluate_subtraction(expression, environment),
        Expression::Multiply(_) => evaluate_multiplication(expression, environment),
        Expression::Variable(key) => evaluate(&environment.value_for_key(key), environment),
        Expression::Number(val) => *val,
    }
}

fn main() {
    let mut expressions = Vec::new();
    expressions.push(Expression::Number(3));
    expressions.push(Expression::Number(4));
    expressions.push(Expression::Number(5));
    let add = Expression::Add(expressions);
    let env = crate::Environment::new();
    let multiply = Expression::Multiply(vec![add, Expression::Number(2)]);
    let result = evaluate(&multiply, &env);
    println!("The result is {result}");

}

#[cfg(test)]
mod tests {
use crate::{evaluate_addition, evaluate_subtraction, Environment, Expression};
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_basic_addition() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];
        let env = crate::Environment::new();
        // act
        let sum = evaluate_addition(&Expression::Add(values), &env);
        // assert
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_basic_subtraction() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];
        let env = crate::Environment::new();
        // act
        let difference = evaluate_subtraction(&Expression::Subtract(values), &env);
        // assert
        assert_eq!(difference, 0);
    }
    #[test]
    fn test_basic_addition_not5() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];
        let env = crate::Environment::new();
        // act
        let sum = evaluate_addition(&Expression::Add(values), &env);
        // assert
        assert_ne!(sum, 5);
    }

    #[test]
    fn test_new_environment() {
        let new_env = crate::Environment::new();
        let expr = new_env.value;
        if let crate::Expression::Number(value) = expr {
            assert_eq!(value, 0);
        } else {
            assert_eq!(1,0);
        }
    }

    #[test]
    fn test_value_for_key() {
        let mut new_env = crate::Environment::new();
        new_env.key = String::from("foo");
        new_env.value = crate::Expression::Number(2);
        let expr = new_env.value_for_key(&String::from("foo"));
        if let crate::Expression::Number(value) = expr {
            assert_eq!(*value, 2);
        } else {
            assert_eq!(2,0);
        }
    }

    #[test]
    fn simple_environment() {
        let env = crate::Environment {key: String::from("x"), value: crate::Expression::Number(5)};
        let values = vec![crate::Expression::Number(2), crate::Expression::Variable(String::from("x"))];
        let sum = crate::evaluate(&crate::Expression::Add(values), &env);
        assert_eq!(sum, 7);
    }

}




