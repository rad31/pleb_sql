use crate::lexer::keyword::Keyword;

use super::Token;

#[derive(Debug)]
pub struct KeywordToken<'a> {
    pub lexeme: &'a str,
    pub keyword: Keyword,
}

impl Token for KeywordToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        !c.is_ascii_alphanumeric() && c != '_'
    }
}
