use crate::lexer::punctuator::Punctuator;

use super::Token;

#[derive(Debug)]
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

    fn is_end(_: char, _: Option<char>) -> bool {
        true
    }
}
