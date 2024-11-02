use crate::lib::token::Token;

struct IdentifierToken<'a> {
    pub lexeme: &'a str,
}

impl Token for IdentifierToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_lextant(&self) -> bool {
        true
    }
}
