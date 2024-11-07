use crate::lexer::symbols::SUBTRACT;

use super::Token;

#[derive(Debug)]
pub struct IntegerToken {
    pub value: i32,
}

impl Token for IntegerToken {
    fn is_start(c: char) -> bool {
        c.is_ascii_digit() || c == SUBTRACT
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        !c.is_ascii_digit()
    }
}
