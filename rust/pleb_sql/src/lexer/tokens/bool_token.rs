use super::{keyword_token::KeywordToken, Token};

#[derive(Debug)]
pub struct BoolToken {
    pub value: bool,
}

impl BoolToken {
    pub fn is(lexeme: &str) -> bool {
        lexeme == TRUE || lexeme == FALSE
    }

    pub fn value(lexeme: &str) -> bool {
        lexeme == TRUE
    }
}

impl Token for BoolToken {
    fn is_start(c: char) -> bool {
        KeywordToken::is_start(c)
    }

    fn is_end(curr: char, prev: Option<char>) -> bool {
        KeywordToken::is_end(curr, prev)
    }
}

pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
