use crate::keyword::Keyword;

use super::Token;

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

    fn is_continuation(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    // fn as_variant(&self) -> TokenVariant<'_> {
    //     return TokenVariant::Keyword(self);
    // }
}
