use crate::lexer::punctuator::{BACK_SLASH, DOUBLE_QUOTE};

use super::Token;

#[derive(Debug)]
pub struct StringToken<'a> {
    pub lexeme: &'a str,
}

impl Token for StringToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c == DOUBLE_QUOTE
    }

    fn is_end(curr: char, prev: Option<char>) -> bool {
        curr == DOUBLE_QUOTE && prev.unwrap() != BACK_SLASH
    }
}
