pub mod script_iterator;

use std::collections::HashMap;

use crate::{
    keyword::Keyword,
    punctuator::Punctuator,
    token::{
        identifier_token::IdentifierToken, integer_literal_token::IntegerLiteralToken,
        keyword_token::KeywordToken, punctuator_token::PunctuatorToken, Token, TokenVariant,
    },
};
use script_iterator::ScriptIterator;

pub struct LexicalAnalyzer<'a> {
    pub input: &'a str,
    pub iterator: ScriptIterator<'a>,
    pub keywords: HashMap<&'static str, Keyword>,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new(input: &str) -> LexicalAnalyzer<'_> {
        LexicalAnalyzer {
            input,
            iterator: ScriptIterator::new(&input),
            keywords: Keyword::get_map(),
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

        let (index, c) = self.iterator.next()?;

        if is_start::<IntegerLiteralToken>(c) {
            return Some(self.parse_number(index));
        }

        if is_start::<IdentifierToken>(c) {
            return Some(self.parse_identifier_or_keyword(index));
        }

        if is_start::<PunctuatorToken>(c) {
            return Some(self.parse_punctuator(index, c));
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

    fn parse_identifier_or_keyword(&mut self, start: usize) -> TokenVariant {
        while let Some((index, c)) = self.iterator.peek() {
            if !is_continuation::<IdentifierToken>(*c) {
                let lexeme: &str = &self.input[start..*index];
                if let Some(keyword) = self.keywords.get(lexeme) {
                    return TokenVariant::Keyword(KeywordToken {
                        lexeme,
                        keyword: keyword.clone(),
                    });
                }
                return TokenVariant::Identifier(IdentifierToken { lexeme });
            }
            self.iterator.next();
        }

        let end = self.input.len();

        return TokenVariant::Identifier(IdentifierToken {
            lexeme: &self.input[start..end],
        });
    }

    fn parse_punctuator(&mut self, start: usize, c: char) -> TokenVariant {
        TokenVariant::Punctuator(PunctuatorToken {
            lexeme: &self.input[start..start + 1],
            punctuator: Punctuator::get(&c).unwrap(),
        })
    }
}

fn is_start<T: Token>(c: char) -> bool {
    T::is_start(c)
}

fn is_continuation<T: Token>(c: char) -> bool {
    T::is_continuation(c)
}
