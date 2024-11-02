use crate::lib::token::Token;

pub struct NumberToken<'a> {
    pub lexeme: &'a str,
}

impl Token for NumberToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_lextant(&self) -> bool {
        true
    }
}
