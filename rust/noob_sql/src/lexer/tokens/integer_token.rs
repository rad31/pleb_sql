use super::Token;

#[derive(Debug)]
pub struct IntegerToken<'a> {
    pub lexeme: &'a str,
}

impl Token for IntegerToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        !c.is_ascii_digit()
    }
}
