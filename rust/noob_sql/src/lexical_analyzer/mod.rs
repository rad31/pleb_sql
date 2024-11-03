pub mod script_iterator;

use crate::token::{identifier_token::IdentifierToken, number_token::NumberToken, Token};
use script_iterator::ScriptIterator;

pub enum TokenVariant<'a> {
    Number(NumberToken<'a>),
    Identifier(IdentifierToken<'a>),
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

        let (_, c) = self.iterator.peek()?;

        if is_start::<NumberToken>(*c) {
            return Some(self.parse_number());
        }

        if is_start::<IdentifierToken>(*c) {
            return Some(self.parse_identifier());
        }

        panic!(
            "Invalid token at line {}, position {}",
            self.iterator.line, self.iterator.index
        );
    }

    fn parse_number(&mut self) -> TokenVariant {
        let (start, _) = self.iterator.next().unwrap();

        while let Some((index, c)) = self.iterator.next() {
            if !is_continuation::<IdentifierToken>(c) {
                return TokenVariant::Number(NumberToken {
                    lexeme: &self.input[start..index],
                });
            }
        }

        let end = self.input.len();

        TokenVariant::Number(NumberToken {
            lexeme: &self.input[start..end],
        })
    }

    fn parse_identifier(&mut self) -> TokenVariant {
        let (start, _) = self.iterator.next().unwrap();

        while let Some((index, c)) = self.iterator.next() {
            if !is_continuation::<IdentifierToken>(c) {
                return TokenVariant::Identifier(IdentifierToken {
                    lexeme: &self.input[start..index],
                });
            }
        }

        let end = self.input.len();

        return TokenVariant::Identifier(IdentifierToken {
            lexeme: &self.input[start..end],
        });
    }
}

fn is_start<T: Token>(c: char) -> bool {
    T::is_start(c)
}

fn is_continuation<T: Token>(c: char) -> bool {
    T::is_continuation(c)
}
