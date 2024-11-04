pub enum Punctuator {
    Comma,
    DoubleQuote,
    Glob,
    ParenClose,
    ParenOpen,
    Semicolon,
}

impl Punctuator {
    pub fn get(c: &char) -> Option<Punctuator> {
        match c {
            ',' => Some(Punctuator::Comma),
            '"' => Some(Punctuator::DoubleQuote),
            '*' => Some(Punctuator::Glob),
            '(' => Some(Punctuator::ParenOpen),
            ')' => Some(Punctuator::ParenClose),
            ';' => Some(Punctuator::Semicolon),
            _ => None,
        }
    }
}

pub static COMMA: char = ',';
pub static DOUBLE_QUOTE: char = '"';
pub static GLOB: char = '*';
pub static PAREN_CLOSE: char = '(';
pub static PAREN_OPEN: char = ')';
pub static SEMICOLON: char = ';';
