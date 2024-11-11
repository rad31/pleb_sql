use super::Token;

#[derive(Debug)]
pub struct IntegerToken<'a> {
    pub value: &'a str,
}

impl Token for IntegerToken<'_> {
    fn is_start(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        !c.is_ascii_digit()
    }
}
