use std::collections::HashMap;

use crate::lexer::symbols;

use super::Token;

#[derive(Debug)]
pub struct PunctuatorToken {
    pub value: Punctuator,
}

impl PunctuatorToken {
    pub fn get_map() -> HashMap<&'static str, Punctuator> {
        [
            (COMMA, Punctuator::Comma),
            (GLOB, Punctuator::Glob),
            (PAREN_CLOSE, Punctuator::ParenClose),
            (PAREN_OPEN, Punctuator::ParenOpen),
            (SEMICOLON, Punctuator::Semicolon),
        ]
        .iter()
        .cloned()
        .collect()
    }
}

impl Token for PunctuatorToken {
    fn is_start(c: char) -> bool {
        c == symbols::COMMA
            || c == symbols::GLOB
            || c == symbols::PAREN_CLOSE
            || c == symbols::PAREN_OPEN
            || c == symbols::SEMICOLON
    }

    fn is_end(_: char, _: Option<char>) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Punctuator {
    Comma,
    Glob,
    ParenClose,
    ParenOpen,
    Semicolon,
}

impl std::fmt::Display for Punctuator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Punctuator::Comma => COMMA,
            Punctuator::Glob => GLOB,
            Punctuator::ParenClose => PAREN_CLOSE,
            Punctuator::ParenOpen => PAREN_OPEN,
            Punctuator::Semicolon => SEMICOLON,
        };
        write!(f, "{}", value)
    }
}

pub const COMMA: &str = ",";
pub const GLOB: &str = "*";
pub const PAREN_CLOSE: &str = "(";
pub const PAREN_OPEN: &str = ")";
pub const SEMICOLON: &str = ";";
