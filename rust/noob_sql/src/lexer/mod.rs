#[cfg(test)]
mod _tests;
pub mod error;
mod inspectable;
pub mod keyword;
pub mod operator;
pub mod punctuator;
mod script_iterator;
pub mod tokens;

use std::collections::HashMap;

use error::LexingError;
use inspectable::Inspectable;
use keyword::Keyword;
use operator::Operator;
use punctuator::Punctuator;
use script_iterator::ScriptIterator;

use tokens::{
    char_token::CharToken, identifier_token::IdentifierToken, integer_token::IntegerToken,
    keyword_token::KeywordToken, operator_token::OperatorToken, punctuator_token::PunctuatorToken,
    string_token::StringToken, TokenVariant, CHAR, INVALID, OPERATOR, STRING,
};

pub struct Lexer<'a> {
    pub input: &'a str,
    pub iterator: ScriptIterator<'a>,
    pub keywords: HashMap<&'static str, Keyword>,
    pub operators: HashMap<&'static str, Operator>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer {
            input,
            iterator: ScriptIterator::new(&input),
            keywords: Keyword::get_map(),
            operators: Operator::get_map(),
        }
    }

    pub fn next(&mut self) -> Option<Result<TokenVariant, LexingError>> {
        loop {
            let (_, c) = self.iterator.peek()?;

            if !c.is_ascii_whitespace() {
                break;
            }

            self.iterator.next();
        }

        let (index, c) = self.iterator.next()?;

        if c.starts::<IntegerToken>() {
            return Some(Ok(self.read_number(index)));
        }

        if c.starts::<PunctuatorToken>() {
            return Some(Ok(self.read_punctuator(index, c)));
        }

        if c.starts::<IdentifierToken>() {
            return Some(Ok(self.read_identifier_or_keyword(index)));
        }

        if c.starts::<OperatorToken>() {
            return Some(self.read_operator(index));
        }

        if c.starts::<StringToken>() {
            return Some(self.read_string(index, c));
        }

        if c.starts::<CharToken>() {
            return Some(self.read_char(index));
        }

        Some(Err(LexingError::new(INVALID, self.iterator.line, index)))
    }

    fn read_number(&mut self, start: usize) -> TokenVariant {
        while let Some((index, c)) = self.iterator.peek() {
            if c.terminates::<IntegerToken>(None) {
                return TokenVariant::Integer(IntegerToken {
                    lexeme: &self.input[start..*index],
                });
            }
            self.iterator.next();
        }

        let end = self.input.len();

        TokenVariant::Integer(IntegerToken {
            lexeme: &self.input[start..end],
        })
    }

    fn read_punctuator(&mut self, start: usize, c: char) -> TokenVariant {
        TokenVariant::Punctuator(PunctuatorToken {
            lexeme: &self.input[start..start + 1],
            punctuator: Punctuator::get(&c).unwrap(),
        })
    }

    fn read_identifier_or_keyword(&mut self, start: usize) -> TokenVariant {
        let mut end = start + 1;
        while let Some((_, c)) = self.iterator.peek() {
            end += 1;
            if c.terminates::<IdentifierToken>(None) {
                break;
            }
            self.iterator.next();
        }

        let lexeme: &str = &self.input[start..end];
        match self.keywords.get(lexeme) {
            Some(keyword) => TokenVariant::Keyword(KeywordToken {
                lexeme,
                keyword: keyword.clone(),
            }),
            None => TokenVariant::Identifier(IdentifierToken { lexeme }),
        }
    }

    fn read_operator(&mut self, start: usize) -> Result<TokenVariant, LexingError> {
        let mut end = start + 1;
        while let Some((_, c)) = self.iterator.peek() {
            end += 1;
            if c.terminates::<OperatorToken>(None) {
                break;
            }
            self.iterator.next();
        }

        let lexeme: &str = &self.input[start..end];
        match self.operators.get(lexeme) {
            Some(operator) => Ok(TokenVariant::Operator(OperatorToken {
                lexeme,
                operator: operator.clone(),
            })),
            None => Err(LexingError::new(OPERATOR, self.iterator.line, start)),
        }
    }

    fn read_string(&mut self, start: usize, c: char) -> Result<TokenVariant, LexingError> {
        let start = start + 1;
        let curr = self.iterator.peek();
        if curr.is_none() {
            return Err(LexingError::new(STRING, self.iterator.line, start));
        }
        let (_, curr_char) = curr.unwrap();

        if curr_char.terminates::<StringToken>(Some(c)) {
            let variant = TokenVariant::String(StringToken {
                lexeme: &self.input[start..start],
            });
            self.iterator.next();
            return Ok(variant);
        }

        loop {
            let prev = self.iterator.next();
            if prev.is_none() {
                return Err(LexingError::new(STRING, self.iterator.line, start));
            }
            let (__, prev_char) = prev.unwrap();

            let curr = self.iterator.peek();
            if curr.is_none() {
                return Err(LexingError::new(STRING, self.iterator.line, start));
            }
            let (curr_index, curr_char) = curr.unwrap();

            if curr_char.terminates::<StringToken>(Some(prev_char)) {
                let variant = TokenVariant::String(StringToken {
                    lexeme: &self.input[start..*curr_index],
                });
                self.iterator.next();
                return Ok(variant);
            }
        }
    }

    fn read_char(&mut self, start: usize) -> Result<TokenVariant, LexingError> {
        let start = start + 1;
        let prev = self.iterator.next();
        let curr = self.iterator.peek();

        if prev.is_none() || curr.is_none() {
            return Err(LexingError::new(CHAR, self.iterator.line, start));
        }

        let (_, prev_char) = prev.unwrap();
        let (curr_index, curr_char) = curr.unwrap();

        if curr_char.terminates::<CharToken>(Some(prev_char)) {
            let variant = TokenVariant::Char(CharToken {
                lexeme: &self.input[start..*curr_index],
            });
            self.iterator.next();
            return Ok(variant);
        }

        Err(LexingError::new(CHAR, self.iterator.line, start))
    }
}
