use std::str::FromStr;

use crate::{
    math::{Associativity, MathOperator},
    rpn::RpnItem,
    utils::normalize_whitespace,
};

#[derive(Clone, Debug, PartialEq)]
pub enum InfixItem {
    Operand(f64),
    Boolean(bool),
    Operator(MathOperator),
    OpenParen,
    CloseParen,
}

impl FromStr for InfixItem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => return Ok(InfixItem::OpenParen),
            ")" => return Ok(InfixItem::CloseParen),
            _ => {}
        }

        if let Ok(num) = s.parse::<f64>() {
            return Ok(InfixItem::Operand(num));
        }

        if let Ok(b) = s.parse::<bool>() {
            return Ok(InfixItem::Boolean(b));
        }

        if let Ok(op) = s.parse::<MathOperator>() {
            return Ok(InfixItem::Operator(op));
        }

        match s.chars().nth(0) {
            Some(c) if c.is_digit(10) => Err(format!("Invalid Number: {}", s).into()),
            _ => Err(format!("Invalid Operator: {}", s).into()),
        }
    }
}

impl InfixItem {
    fn to_rpn_item(&self) -> Result<RpnItem, bool> {
        match *self {
            InfixItem::Operand(num) => Ok(RpnItem::Operand(num)),
            InfixItem::Boolean(b) => Ok(RpnItem::Boolean(b)),
            InfixItem::Operator(op) => Ok(RpnItem::Operator(op)),
            InfixItem::OpenParen => Err(true),
            InfixItem::CloseParen => Err(false),
        }
    }
}

/// Replaces `<not a number> - <number>` with `<not a number> <negative number>`
fn fix_negative_numbers(vec: &mut Vec<InfixItem>) {
    let mut i = 0;
    while i < vec.len() - 1 {
        if let InfixItem::Operator(MathOperator::Sub) = vec[i] {
            if i > 0 && matches!(vec[i - 1], InfixItem::Operand(_)) {
                i += 1;
                continue;
            }
            if let InfixItem::Operand(num) = vec[i + 1] {
                vec[i] = InfixItem::Operand(-num);
                vec.remove(i + 1);
            }
        }
        i += 1;
    }
}

pub fn parse_infix(s: &str) -> Result<Vec<InfixItem>, String> {
    let mut vec = normalize_whitespace(s)
        .split_whitespace()
        .map(|token| token.parse())
        .collect::<Result<_, _>>()?;
    fix_negative_numbers(&mut vec);
    Ok(vec)
}

pub fn infix_to_rpn(tokens: &[InfixItem]) -> Result<Vec<RpnItem>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();

    for token in tokens {
        match *token {
            InfixItem::Operand(num) => output.push(RpnItem::Operand(num)),
            InfixItem::Boolean(b) => output.push(RpnItem::Boolean(b)),
            InfixItem::Operator(op) => {
                while let Some(top) = stack.last() {
                    if let InfixItem::Operator(top_op) = top {
                        if (op.associativity() == Associativity::Left
                            && op.precedence() <= top_op.precedence())
                            || (op.associativity() == Associativity::Right
                                && op.precedence() < top_op.precedence())
                        {
                            output.push(RpnItem::Operator(*top_op));
                            stack.pop();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                stack.push(InfixItem::Operator(op));
            }
            InfixItem::OpenParen => stack.push(InfixItem::OpenParen),
            InfixItem::CloseParen => loop {
                if let Some(top) = stack.last() {
                    if let InfixItem::OpenParen = top {
                        stack.pop();
                        break;
                    } else {
                        output.push(top.to_rpn_item().unwrap());
                        stack.pop();
                    }
                } else {
                    return Err("Mismatched Parentheses".into());
                }
            },
        }
    }

    while let Some(top) = stack.last() {
        if let InfixItem::OpenParen = top {
            return Err("Mismatched Parentheses".into());
        }
        output.push(top.to_rpn_item().unwrap());
        stack.pop();
    }

    Ok(output)
}
