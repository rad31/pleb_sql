use super::Token;

pub struct PunctuatorToken<'a> {
    pub lexeme: &'a str,
}

impl Token for PunctuatorToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c == COMMA
            || c == DOUBLE_QUOTE
            || c == GLOB
            || c == PAREN_CLOSE
            || c == PAREN_OPEN
            || c == SEMICOLON
    }

    fn is_continuation(_: char) -> bool {
        false
    }

    // fn as_variant(&self) -> TokenVariant<'_> {
    //     return TokenVariant::Punctuator(self);
    // }
}

pub static COMMA: char = ',';
pub static DOUBLE_QUOTE: char = '"';
pub static GLOB: char = '*';
pub static PAREN_CLOSE: char = '(';
pub static PAREN_OPEN: char = ')';
pub static SEMICOLON: char = ';';
