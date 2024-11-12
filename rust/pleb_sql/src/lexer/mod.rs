pub mod _error;
#[cfg(test)]
mod _tests;
mod inspectable;
mod script_iterator;
pub mod symbols;
pub mod tokens;

use std::collections::HashMap;

use inspectable::Inspectable;
use script_iterator::ScriptIterator;

pub type Error = _error::LexingError;
pub type Result<T> = core::result::Result<T, Error>;

use tokens::{
    bool_token::{BoolToken, FALSE, TRUE},
    char_token::CharToken,
    identifier_token::IdentifierToken,
    integer_token::IntegerToken,
    keyword_token::{Keyword, KeywordToken},
    operator_token::{Operator, OperatorToken},
    punctuator_token::{Punctuator, PunctuatorToken},
    string_token::StringToken,
    Token, TokenVariant, CHAR, INVALID, OPERATOR, PUNCTUATOR, STRING,
};

pub struct Lexer<'a> {
    input: &'a str,
    iterator: ScriptIterator<'a>,
    keywords: HashMap<&'static str, Keyword>,
    operators: HashMap<&'static str, Operator>,
    punctuators: HashMap<&'static str, Punctuator>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer {
            input,
            iterator: ScriptIterator::new(&input),
            keywords: KeywordToken::get_map(),
            operators: OperatorToken::get_map(),
            punctuators: PunctuatorToken::get_map(),
        }
    }

    pub fn next(&mut self) -> Option<Result<TokenVariant>> {
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
            return Some(self.read_punctuator(index));
        }

        if c.starts::<KeywordToken>() {
            return Some(Ok(self.read_word(index)));
        }

        if c.starts::<IdentifierToken>() {
            return Some(Ok(self.read_identifier(index)));
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

        // TODO: parse BoolToken

        Some(Err(Error::new(INVALID, self.iterator.line, index)))
    }

    fn read_number(&mut self, start: usize) -> TokenVariant {
        let end = self.read_until_end::<IntegerToken>(start + 1);

        let value = &self.input[start..end];
        TokenVariant::Integer(IntegerToken { value })
    }

    fn read_punctuator(&mut self, start: usize) -> Result<TokenVariant> {
        let lexeme = &self.input[start..start + 1];
        let punctuator = self.punctuators.get(lexeme);

        match punctuator {
            Some(value) => Ok(TokenVariant::Punctuator(PunctuatorToken { value: *value })),
            None => Err(Error::new(PUNCTUATOR, self.iterator.line, start)),
        }
    }

    fn read_word(&mut self, start: usize) -> TokenVariant {
        let mut end = start + 1;
        let mut is_identifier = false;

        while let Some((_, c)) = self.iterator.peek() {
            if c.terminates::<KeywordToken>(None) {
                is_identifier = !c.terminates::<IdentifierToken>(None);
                break;
            }
            end += 1;
            self.iterator.next();
        }

        if is_identifier {
            end = self.read_until_end::<IdentifierToken>(end);
            let lexeme: &str = &self.input[start..end];
            return TokenVariant::Identifier(IdentifierToken { value: lexeme });
        }

        let lexeme: &str = &self.input[start..end];

        if let Some(keyword) = self.keywords.get(lexeme) {
            return TokenVariant::Keyword(KeywordToken { value: *keyword });
        }

        if lexeme == TRUE || lexeme == FALSE {
            return TokenVariant::Bool(BoolToken {
                value: BoolToken::value(lexeme),
            });
        }

        TokenVariant::Identifier(IdentifierToken { value: lexeme })
    }

    fn read_identifier(&mut self, start: usize) -> TokenVariant {
        let end = self.read_until_end::<IdentifierToken>(start + 1);
        let lexeme: &str = &self.input[start..end];
        TokenVariant::Identifier(IdentifierToken { value: lexeme })
    }

    fn read_operator(&mut self, start: usize) -> Result<TokenVariant> {
        let end = self.read_until_end::<OperatorToken>(start + 1);
        let lexeme: &str = &self.input[start..end];
        match self.operators.get(lexeme) {
            Some(operator) => Ok(TokenVariant::Operator(OperatorToken { value: *operator })),
            None => Err(Error::new(OPERATOR, self.iterator.line, start)),
        }
    }

    fn read_string(&mut self, start: usize, c: char) -> Result<TokenVariant> {
        let start = start + 1;
        let curr = self.iterator.peek();
        if curr.is_none() {
            return Err(Error::new(STRING, self.iterator.line, start));
        }
        let (_, curr_char) = curr.unwrap();

        if curr_char.terminates::<StringToken>(Some(c)) {
            let variant = TokenVariant::String(StringToken {
                value: &self.input[start..start],
            });
            self.iterator.next();
            return Ok(variant);
        }

        loop {
            let prev = self.iterator.next();
            if prev.is_none() {
                return Err(Error::new(STRING, self.iterator.line, start));
            }
            let (__, prev_char) = prev.unwrap();

            let curr = self.iterator.peek();
            if curr.is_none() {
                return Err(Error::new(STRING, self.iterator.line, start));
            }
            let (curr_index, curr_char) = curr.unwrap();

            if curr_char.terminates::<StringToken>(Some(prev_char)) {
                let variant = TokenVariant::String(StringToken {
                    value: &self.input[start..*curr_index],
                });
                self.iterator.next();
                return Ok(variant);
            }
        }
    }

    fn read_char(&mut self, start: usize) -> Result<TokenVariant> {
        let start = start + 1;
        let prev = self.iterator.next();
        let curr = self.iterator.peek();

        if prev.is_none() || curr.is_none() {
            return Err(Error::new(CHAR, self.iterator.line, start));
        }

        let (_, prev_char) = prev.unwrap();
        let (_, curr_char) = curr.unwrap();

        if curr_char.terminates::<CharToken>(Some(prev_char)) {
            let variant = TokenVariant::Char(CharToken { value: prev_char });
            self.iterator.next();
            return Ok(variant);
        }

        Err(Error::new(CHAR, self.iterator.line, start))
    }

    pub fn read_until_end<T: Token>(&mut self, start: usize) -> usize {
        let mut end = start;
        while let Some((_, c)) = self.iterator.peek() {
            if c.terminates::<T>(None) {
                break;
            }
            end += 1;
            self.iterator.next();
        }
        end
    }
}
