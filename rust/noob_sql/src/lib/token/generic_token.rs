use std::marker::PhantomData;

use crate::lib::token::Token;

struct GenericToken<'a, T> {
    pub lexeme: &'a str,
    phantom: PhantomData<T>,
}

impl<T> Token for GenericToken<'_, T> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_lextant(&self) -> bool {
        true
    }
}
