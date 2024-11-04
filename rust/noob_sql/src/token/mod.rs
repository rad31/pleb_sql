pub mod identifier_token;
pub mod integer_literal_token;
pub mod punctuator_token;

pub trait Token {
    fn get_lexeme(&self) -> &str;
    fn is_start(c: char) -> bool;
    fn is_continuation(c: char) -> bool;
}
