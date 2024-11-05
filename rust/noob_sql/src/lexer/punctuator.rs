#[derive(Debug)]
pub enum Punctuator {
    Comma,
    Glob,
    ParenClose,
    ParenOpen,
    Semicolon,
}

impl Punctuator {
    pub fn get(c: &char) -> Option<Punctuator> {
        match c {
            &COMMA => Some(Punctuator::Comma),
            &GLOB => Some(Punctuator::Glob),
            &PAREN_CLOSE => Some(Punctuator::ParenClose),
            &PAREN_OPEN => Some(Punctuator::ParenOpen),
            &SEMICOLON => Some(Punctuator::Semicolon),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn list() -> [char; 5] {
        [COMMA, GLOB, PAREN_CLOSE, PAREN_OPEN, SEMICOLON]
    }
}

pub const COMMA: char = ',';
pub const GLOB: char = '*';
pub const PAREN_CLOSE: char = '(';
pub const PAREN_OPEN: char = ')';
pub const SEMICOLON: char = ';';

// TODO: move these to new file
pub const BACK_SLASH: char = '\\';
pub const SINGLE_QUOTE: char = '\'';
pub const DOUBLE_QUOTE: char = '"';
