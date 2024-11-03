use crate::token::Token;

pub struct IdentifierToken<'a> {
    pub lexeme: &'a str,
}

impl Token for IdentifierToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn is_continuation(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}
