pub mod script_iterator;

use crate::token::{
    identifier_token::IdentifierToken, integer_literal_token::IntegerLiteralToken,
    punctuator_token::PunctuatorToken, Token,
};
use script_iterator::ScriptIterator;

pub enum TokenVariant<'a> {
    IntegerLiteral(IntegerLiteralToken<'a>),
    Identifier(IdentifierToken<'a>),
    Punctuator(PunctuatorToken<'a>),
}

pub struct LexicalAnalyzer<'a> {
    pub input: &'a str,
    pub iterator: ScriptIterator<'a>,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new(input: &str) -> LexicalAnalyzer<'_> {
        LexicalAnalyzer {
            input,
            iterator: ScriptIterator::new(&input),
        }
    }

    pub fn next(&mut self) -> Option<TokenVariant> {
        loop {
            let (_, c) = self.iterator.peek()?;

            if !c.is_ascii_whitespace() {
                break;
            }

            self.iterator.next();
        }

        let (index, c) = self.iterator.peek()?;
        let index = index.clone();

        if is_start::<IntegerLiteralToken>(*c) {
            return Some(self.parse_number(index));
        }

        if is_start::<IdentifierToken>(*c) {
            return Some(self.parse_identifier(index));
        }

        if is_start::<PunctuatorToken>(*c) {
            return Some(self.parse_punctuator(index));
        }

        panic!(
            "Invalid token at line {}, position {}",
            self.iterator.line, self.iterator.index
        );
    }

    fn parse_number(&mut self, start: usize) -> TokenVariant {
        while let Some((index, c)) = self.iterator.peek() {
            if !is_continuation::<IntegerLiteralToken>(*c) {
                return TokenVariant::IntegerLiteral(IntegerLiteralToken {
                    lexeme: &self.input[start..*index],
                });
            }
            self.iterator.next();
        }

        let end = self.input.len();

        TokenVariant::IntegerLiteral(IntegerLiteralToken {
            lexeme: &self.input[start..end],
        })
    }

    fn parse_identifier(&mut self, start: usize) -> TokenVariant {
        while let Some((index, c)) = self.iterator.peek() {
            if !is_continuation::<IdentifierToken>(*c) {
                return TokenVariant::Identifier(IdentifierToken {
                    lexeme: &self.input[start..*index],
                });
            }
            self.iterator.next();
        }

        let end = self.input.len();

        return TokenVariant::Identifier(IdentifierToken {
            lexeme: &self.input[start..end],
        });
    }

    fn parse_punctuator(&mut self, start: usize) -> TokenVariant {
        self.iterator.next();

        TokenVariant::Punctuator(PunctuatorToken {
            lexeme: &self.input[start..start + 1],
        })
    }
}

fn is_start<T: Token>(c: char) -> bool {
    T::is_start(c)
}

fn is_continuation<T: Token>(c: char) -> bool {
    T::is_continuation(c)
}
