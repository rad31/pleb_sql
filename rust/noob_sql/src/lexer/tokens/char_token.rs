use crate::lexer::punctuator::{BACK_SLASH, SINGLE_QUOTE};

use super::Token;

#[derive(Debug)]
pub struct CharToken<'a> {
    pub lexeme: &'a str,
}

impl Token for CharToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c == SINGLE_QUOTE
    }

    fn is_end(curr: char, prev: Option<char>) -> bool {
        curr == SINGLE_QUOTE && prev.unwrap() != BACK_SLASH
    }
}
