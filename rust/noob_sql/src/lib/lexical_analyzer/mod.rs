pub mod lextant;
pub mod script_iterator;

use crate::lib::token::{number_token::NumberToken, Token};
use script_iterator::ScriptIterator;

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

    pub fn next(&'a mut self) -> Option<impl Token + 'a> {
        let (_, c) = self.iterator.peek()?;

        if c.is_ascii_digit() {
            return Some(self.scan_number());
        }

        // TODO: handle invalid token
        None
    }

    fn scan_number(&'a mut self) -> NumberToken<'a> {
        let (start, _) = self.iterator.peek().unwrap().clone();
        self.iterator
            .by_ref()
            .skip_while(|(_, c)| c.is_ascii_digit());
        let (end, _) = self
            .iterator
            .peek()
            .unwrap_or(&(self.input.len(), ' '))
            .clone();

        NumberToken {
            lexeme: &self.input[start..end],
        }
    }
}
