use crate::lexer::symbols::UNDERSCORE;

use super::Token;

#[derive(Debug)]
pub struct IdentifierToken<'a> {
    pub value: &'a str,
}

impl Token for IdentifierToken<'_> {
    fn is_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == UNDERSCORE
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        !c.is_ascii_alphanumeric() && c != UNDERSCORE
    }
}
