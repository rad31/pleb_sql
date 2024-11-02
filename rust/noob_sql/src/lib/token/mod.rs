pub mod generic_token;
pub mod identifier_token;
pub mod number_token;

pub trait Token {
    fn get_lexeme(&self) -> &str;
    fn is_lextant(&self) -> bool;
}
