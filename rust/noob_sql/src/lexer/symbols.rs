pub const GREATER: char = '>';
pub const LESS: char = '<';
pub const EQUAL: char = '=';
pub const SUBTRACT: char = '-';
pub const ADD: char = '+';
pub const MULTIPLY: char = '*';
pub const DIVIDE: char = '/';

pub const COMMA: char = ',';
pub const GLOB: char = '*';
pub const PAREN_CLOSE: char = '(';
pub const PAREN_OPEN: char = ')';
pub const SEMICOLON: char = ';';

pub const BACK_SLASH: char = '\\';
pub const SINGLE_QUOTE: char = '\'';
pub const DOUBLE_QUOTE: char = '"';

pub const UNDERSCORE: char = '_';

pub fn ending(c: &char) -> bool {
    match c {
        &PAREN_CLOSE => true,
        &SEMICOLON => true,
        _ => false,
    }
}

pub fn starting(c: &char) -> bool {
    match c {
        &PAREN_OPEN => true,
        _ => false,
    }
}
