use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sqrt,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
    Not,
}

impl FromStr for MathOperator {
    type Err = ();

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "+" => Ok(MathOperator::Add),
            "-" => Ok(MathOperator::Sub),
            "*" => Ok(MathOperator::Mul),
            "/" => Ok(MathOperator::Div),
            "^" => Ok(MathOperator::Pow),
            "sqrt" => Ok(MathOperator::Sqrt),
            "=" => Ok(MathOperator::Eq),
            "!=" => Ok(MathOperator::Ne),
            ">" => Ok(MathOperator::Gt),
            ">=" => Ok(MathOperator::Ge),
            "<" => Ok(MathOperator::Lt),
            "<=" => Ok(MathOperator::Le),
            "&" => Ok(MathOperator::And),
            "|" => Ok(MathOperator::Or),
            "!" => Ok(MathOperator::Not),
            _ => Err(()),
        }
    }
}

impl MathOperator {
    pub fn precedence(self) -> u8 {
        // https://en.wikipedia.org/wiki/Order_of_operations
        match self {
            MathOperator::Pow | MathOperator::Sqrt => 4,
            MathOperator::Mul | MathOperator::Div => 3,
            MathOperator::Add | MathOperator::Sub => 2,
            MathOperator::Eq
            | MathOperator::Ne
            | MathOperator::Gt
            | MathOperator::Ge
            | MathOperator::Lt
            | MathOperator::Le => 1,
            MathOperator::And | MathOperator::Or => 0,
            MathOperator::Not => 5,
        }
    }

    pub fn associativity(self) -> Associativity {
        match self {
            MathOperator::Pow | MathOperator::Sqrt => Associativity::Right,
            _ => Associativity::Left,
        }
    }

    pub fn rpn_exec(self, stack: &mut Vec<Value>) -> Result<(), String> {
        match self {
            MathOperator::Sqrt | MathOperator::Not => self.unary_op(stack)?,
            _ => self.binary_op(stack)?,
        }
        Ok(())
    }

    fn unary_op(self, stack: &mut Vec<Value>) -> Result<(), String> {
        let x = stack
            .pop()
            .ok_or_else(|| "invalid syntax: too few operands")?;
        let result = match self {
            MathOperator::Sqrt => match x {
                Value::Number(num) => Value::Number(num.sqrt()),
                _ => return Err("invalid type: sqrt".into()),
            },
            MathOperator::Not => match x {
                Value::Boolean(b) => Value::Boolean(!b),
                _ => return Err("invalid type: not".into()),
            },
            _ => unreachable!(),
        };
        stack.push(result);
        Ok(())
    }

    fn binary_op(self, stack: &mut Vec<Value>) -> Result<(), String> {
        let y = stack
            .pop()
            .ok_or_else(|| "invalid syntax: too few operands")?;
        let x = stack
            .pop()
            .ok_or_else(|| "invalid syntax: too few operands")?;
        let result = match self {
            MathOperator::Add => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                _ => return Err("invalid type: +".into()),
            },
            MathOperator::Sub => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
                _ => return Err("invalid type: -".into()),
            },
            MathOperator::Mul => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                _ => return Err("invalid type: *".into()),
            },
            MathOperator::Div => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x / y),
                _ => return Err("invalid type: /".into()),
            },
            MathOperator::Pow => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Number(x.powf(y)),
                _ => return Err("invalid type: ^".into()),
            },
            MathOperator::Eq => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x == y),
                (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x == y),
                _ => return Err("invalid type: =".into()),
            },
            MathOperator::Ne => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x != y),
                (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x != y),
                _ => return Err("invalid type: !=".into()),
            },
            MathOperator::Gt => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x > y),
                _ => return Err("invalid type: >".into()),
            },
            MathOperator::Ge => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x >= y),
                _ => return Err("invalid type: >=".into()),
            },
            MathOperator::Lt => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x < y),
                _ => return Err("invalid type: <".into()),
            },
            MathOperator::Le => match (x, y) {
                (Value::Number(x), Value::Number(y)) => Value::Boolean(x <= y),
                _ => return Err("invalid type: <=".into()),
            },
            MathOperator::And => match (x, y) {
                (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x && y),
                _ => return Err("invalid type: &&".into()),
            },
            MathOperator::Or => match (x, y) {
                (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x || y),
                _ => return Err("invalid type: ||".into()),
            },
            _ => unreachable!(),
        };
        stack.push(result);
        Ok(())
    }
}

mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_math_operator_from_str() {
        assert_eq!(MathOperator::from_str("+"), Ok(MathOperator::Add));
        assert_eq!(MathOperator::from_str("-"), Ok(MathOperator::Sub));
        assert_eq!(MathOperator::from_str("*"), Ok(MathOperator::Mul));
        assert_eq!(MathOperator::from_str("/"), Ok(MathOperator::Div));
        assert_eq!(MathOperator::from_str("^"), Ok(MathOperator::Pow));
        assert_eq!(MathOperator::from_str("sqrt"), Ok(MathOperator::Sqrt));
        assert_eq!(MathOperator::from_str("="), Ok(MathOperator::Eq));
        assert_eq!(MathOperator::from_str("!="), Ok(MathOperator::Ne));
        assert_eq!(MathOperator::from_str(">"), Ok(MathOperator::Gt));
        assert_eq!(MathOperator::from_str(">="), Ok(MathOperator::Ge));
        assert_eq!(MathOperator::from_str("<"), Ok(MathOperator::Lt));
        assert_eq!(MathOperator::from_str("<="), Ok(MathOperator::Le));
        assert_eq!(MathOperator::from_str("&"), Ok(MathOperator::And));
        assert_eq!(MathOperator::from_str("|"), Ok(MathOperator::Or));
        assert_eq!(MathOperator::from_str("!"), Ok(MathOperator::Not));
        assert_eq!(MathOperator::from_str("foo"), Err(()));
    }

    #[test]
    fn test_math_operator_rpn_exec() {
        let mut stack = vec![Value::Number(1.0), Value::Number(2.0)];
        MathOperator::Add.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Number(3.0)]);

        let mut stack = vec![Value::Number(1.0), Value::Number(2.0)];
        MathOperator::Sub.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Number(-1.0)]);

        let mut stack = vec![Value::Number(1.0), Value::Number(2.0)];
        MathOperator::Mul.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Number(2.0)]);

        let mut stack = vec![Value::Number(1.0), Value::Number(2.0)];
        MathOperator::Div.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Number(0.5)]);

        let mut stack = vec![Value::Number(2.0), Value::Number(3.0)];
        MathOperator::Pow.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Number(8.0)]);

        let mut stack = vec![Value::Number(4.0)];
        MathOperator::Sqrt.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Number(2.0)]);

        let mut stack = vec![Value::Number(1.0), Value::Number(1.0)];
        MathOperator::Eq.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);

        let mut stack = vec![Value::Number(1.0), Value::Number(1.0)];
        MathOperator::Ne.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Number(2.0), Value::Number(1.0)];
        MathOperator::Gt.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);

        let mut stack = vec![Value::Number(1.0), Value::Number(2.0)];
        MathOperator::Ge.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Number(1.0), Value::Number(2.0)];
        MathOperator::Lt.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);

        let mut stack = vec![Value::Number(2.0), Value::Number(1.0)];
        MathOperator::Le.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Boolean(true), Value::Boolean(true)];
        MathOperator::And.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);

        let mut stack = vec![Value::Boolean(true), Value::Boolean(false)];
        MathOperator::And.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Boolean(false), Value::Boolean(true)];
        MathOperator::And.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Boolean(false), Value::Boolean(false)];
        MathOperator::And.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Boolean(true), Value::Boolean(true)];
        MathOperator::Or.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);

        let mut stack = vec![Value::Boolean(true), Value::Boolean(false)];
        MathOperator::Or.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);

        let mut stack = vec![Value::Boolean(false), Value::Boolean(true)];
        MathOperator::Or.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);

        let mut stack = vec![Value::Boolean(false), Value::Boolean(false)];
        MathOperator::Or.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Boolean(true)];
        MathOperator::Not.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(false)]);

        let mut stack = vec![Value::Boolean(false)];
        MathOperator::Not.rpn_exec(&mut stack).unwrap();
        assert_eq!(stack, vec![Value::Boolean(true)]);
    }
}
