use crate::token::Token;

pub struct NumberToken<'a> {
    pub lexeme: &'a str,
}

impl Token for NumberToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_continuation(c: char) -> bool {
        c.is_ascii_digit()
    }
}
