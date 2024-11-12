use std::{iter::Peekable, str::CharIndices};

#[cfg(test)]
mod _tests;

pub struct ScriptIterator<'a> {
    pub line: usize,
    pub index: usize,
    pub iterator: Peekable<CharIndices<'a>>,
}

impl ScriptIterator<'_> {
    pub fn new(input: &str) -> ScriptIterator {
        ScriptIterator {
            line: 0,
            index: 0,
            iterator: input.char_indices().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&(usize, char)> {
        self.iterator.peek()
    }

    fn on_consume(&mut self) {
        if let Some((_, c)) = self.iterator.peek() {
            match c {
                '\n' => {
                    self.line += 1;
                    self.index = 0;
                }
                _ => {
                    self.index += 1;
                }
            }
        };
    }
}

impl<'a> Iterator for ScriptIterator<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.on_consume();
        self.iterator.next()
    }
}
