use std::collections::HashMap;

use crate::lexer::symbols;

use super::Token;

#[derive(Debug)]
pub struct OperatorToken {
    pub value: Operator,
}

impl OperatorToken {
    pub fn get_map() -> HashMap<&'static str, Operator> {
        [
            (ADD, Operator::Add),
            // (SUBTRACT, Operator::Subtract), // TODO: fix bug
            // (MULTIPLY, Operator::Multiply), // TODO: fix bug
            (DIVIDE, Operator::Divide),
            (EQUAL, Operator::Equal),
            (NOT_EQUAL, Operator::NotEqual),
            (GREATER, Operator::Greater),
            (GREATER_EQUAL, Operator::GreaterEqual),
            (LESS, Operator::Less),
            (LESS_EQUAL, Operator::LessEqual),
        ]
        .iter()
        .cloned()
        .collect()
    }
}

impl Token for OperatorToken {
    fn is_start(c: char) -> bool {
        c == symbols::ADD
            || c == symbols::SUBTRACT
            || c == symbols::MULTIPLY
            || c == symbols::DIVIDE
            || c == symbols::EQUAL
            || c == symbols::LESS
            || c == symbols::GREATER
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        c != symbols::GREATER && c != symbols::EQUAL
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Operator::Add => ADD,
            Operator::Subtract => SUBTRACT,
            Operator::Multiply => MULTIPLY,
            Operator::Divide => DIVIDE,
            Operator::Equal => EQUAL,
            Operator::NotEqual => NOT_EQUAL,
            Operator::Greater => GREATER,
            Operator::GreaterEqual => GREATER_EQUAL,
            Operator::Less => LESS,
            Operator::LessEqual => LESS_EQUAL,
        };
        write!(f, "{}", value)
    }
}

pub const ADD: &str = "+";
pub const SUBTRACT: &str = "-";
pub const MULTIPLY: &str = "*";
pub const DIVIDE: &str = "/";
pub const EQUAL: &str = "=";
pub const NOT_EQUAL: &str = "<>";
pub const GREATER: &str = ">";
pub const GREATER_EQUAL: &str = ">=";
pub const LESS: &str = "<";
pub const LESS_EQUAL: &str = "<=";
