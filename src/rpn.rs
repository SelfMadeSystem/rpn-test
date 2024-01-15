use std::str::FromStr;

use crate::math::{MathOperator, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum RpnItem {
    Operand(f64),
    Boolean(bool),
    Operator(MathOperator),
}

impl FromStr for RpnItem {
    type Err = String;

    fn from_str(s: &str) -> Result<RpnItem, Self::Err> {
        if let Ok(num) = s.parse::<f64>() {
            return Ok(RpnItem::Operand(num));
        }

        if let Ok(b) = s.parse::<bool>() {
            return Ok(RpnItem::Boolean(b));
        }

        if let Ok(op) = s.parse::<MathOperator>() {
            return Ok(RpnItem::Operator(op));
        }

        match s.chars().nth(0) {
            Some(c) if c.is_digit(10) => Err(format!("Invalid Number: {}", s).into()),
            _ => Err(format!("Invalid Operator: {}", s).into()),
        }
    }
}

pub fn execute_rpn(tokens: &[RpnItem]) -> Result<Value, String> {
    let mut stack = Vec::new();
    for token in tokens {
        match *token {
            RpnItem::Operand(num) => stack.push(Value::Number(num)),
            RpnItem::Boolean(b) => stack.push(Value::Boolean(b)),
            RpnItem::Operator(op) => op.rpn_exec(&mut stack)?,
        }
    }
    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("invalid syntax: too many operands".to_string())
    }
}

pub fn parse_rpn(s: &str) -> Result<Vec<RpnItem>, String> {
    s.split_whitespace().map(|token| token.parse()).collect()
}

mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_rpn() {
        assert_eq!(
            parse_rpn("1 2 +").unwrap(),
            vec![
                RpnItem::Operand(1.0),
                RpnItem::Operand(2.0),
                RpnItem::Operator(MathOperator::Add)
            ]
        );
        assert_eq!(
            parse_rpn("1 2 + 3 *").unwrap(),
            vec![
                RpnItem::Operand(1.0),
                RpnItem::Operand(2.0),
                RpnItem::Operator(MathOperator::Add),
                RpnItem::Operand(3.0),
                RpnItem::Operator(MathOperator::Mul),
            ]
        );
        assert_eq!(
            parse_rpn("1 2 + 3 * 4 /").unwrap(),
            vec![
                RpnItem::Operand(1.0),
                RpnItem::Operand(2.0),
                RpnItem::Operator(MathOperator::Add),
                RpnItem::Operand(3.0),
                RpnItem::Operator(MathOperator::Mul),
                RpnItem::Operand(4.0),
                RpnItem::Operator(MathOperator::Div),
            ]
        );
        assert_eq!(
            parse_rpn("1 2 + 3 * 4 / 5 -").unwrap(),
            vec![
                RpnItem::Operand(1.0),
                RpnItem::Operand(2.0),
                RpnItem::Operator(MathOperator::Add),
                RpnItem::Operand(3.0),
                RpnItem::Operator(MathOperator::Mul),
                RpnItem::Operand(4.0),
                RpnItem::Operator(MathOperator::Div),
                RpnItem::Operand(5.0),
                RpnItem::Operator(MathOperator::Sub),
            ]
        );
        assert_eq!(
            parse_rpn("1 2 + 3 * 4 / 5 - 6 7 * +").unwrap(),
            vec![
                RpnItem::Operand(1.0),
                RpnItem::Operand(2.0),
                RpnItem::Operator(MathOperator::Add),
                RpnItem::Operand(3.0),
                RpnItem::Operator(MathOperator::Mul),
                RpnItem::Operand(4.0),
                RpnItem::Operator(MathOperator::Div),
                RpnItem::Operand(5.0),
                RpnItem::Operator(MathOperator::Sub),
                RpnItem::Operand(6.0),
                RpnItem::Operand(7.0),
                RpnItem::Operator(MathOperator::Mul),
                RpnItem::Operator(MathOperator::Add),
            ]
        );
    }

    #[test]
    fn test_execute_rpn() {
        assert_eq!(
            execute_rpn(&parse_rpn("1 2 +").unwrap()).unwrap(),
            Value::Number(3.0)
        );
        assert_eq!(
            execute_rpn(&parse_rpn("1 2 + 3 *").unwrap()).unwrap(),
            Value::Number(9.0)
        );
        assert_eq!(
            execute_rpn(&parse_rpn("1 2 + 3 * 4 /").unwrap()).unwrap(),
            Value::Number(2.25)
        );
        assert_eq!(
            execute_rpn(&parse_rpn("1 2 + 3 * 4 / 5 -").unwrap()).unwrap(),
            Value::Number(-2.75)
        );
        assert_eq!(
            execute_rpn(&parse_rpn("1 2 + 3 * 4 / 5 - 6 7 * +").unwrap()).unwrap(),
            Value::Number(39.25)
        );
    }
}
