use crate::lexer::symbols::{BACK_SLASH, SINGLE_QUOTE};

use super::Token;

#[derive(Debug)]
pub struct CharToken {
    pub value: char,
}

impl Token for CharToken {
    fn is_start(c: char) -> bool {
        c == SINGLE_QUOTE
    }

    fn is_end(curr: char, prev: Option<char>) -> bool {
        curr == SINGLE_QUOTE && prev.unwrap() != BACK_SLASH
    }
}
