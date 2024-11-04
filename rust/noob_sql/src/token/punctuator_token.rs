use crate::punctuator::Punctuator;

use super::Token;

pub struct PunctuatorToken<'a> {
    pub lexeme: &'a str,
    pub punctuator: Punctuator,
}

impl Token for PunctuatorToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        let result = Punctuator::get(&c);
        match result {
            Some(_) => true,
            None => false,
        }
    }

    fn is_continuation(_: char) -> bool {
        false
    }

    // fn as_variant(&self) -> TokenVariant<'_> {
    //     return TokenVariant::Punctuator(self);
    // }
}
