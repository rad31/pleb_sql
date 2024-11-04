use super::Token;

pub struct IntegerLiteralToken<'a> {
    pub lexeme: &'a str,
}

impl Token for IntegerLiteralToken<'_> {
    fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    fn is_start(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_continuation(c: char) -> bool {
        c.is_ascii_digit()
    }

    // fn as_variant(&self) -> TokenVariant<'_> {
    //     return TokenVariant::IntegerLiteral(self);
    // }
}
