use crate::lexer::operator::Operator;

use super::Token;

#[derive(Debug)]
pub struct OperatorToken<'a> {
    pub lexeme: &'a str,
    pub operator: Operator,
}

impl Token for OperatorToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        Operator::is_start(&c)
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        c.is_ascii_whitespace()
    }
}
