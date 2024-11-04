use identifier_token::IdentifierToken;
use integer_literal_token::IntegerLiteralToken;
use keyword_token::KeywordToken;
use punctuator_token::PunctuatorToken;

pub mod identifier_token;
pub mod integer_literal_token;
pub mod keyword_token;
pub mod punctuator_token;

pub trait Token {
    fn get_lexeme(&self) -> &str;
    fn is_start(c: char) -> bool;
    fn is_continuation(c: char) -> bool;
}

pub enum TokenVariant<'a> {
    IntegerLiteral(IntegerLiteralToken<'a>),
    Identifier(IdentifierToken<'a>),
    Keyword(KeywordToken<'a>),
    Punctuator(PunctuatorToken<'a>),
}
