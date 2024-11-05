use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Equal,
    NotEqual,
}

impl Operator {
    pub fn get_map() -> HashMap<&'static str, Operator> {
        [(EQUAL, Operator::Equal), (NOT_EQUAL, Operator::NotEqual)]
            .iter()
            .cloned()
            .collect()
    }

    pub fn is_start(c: &char) -> bool {
        *c == '=' || *c == '<'
    }
}

pub const EQUAL: &str = "=";
pub const NOT_EQUAL: &str = "<>";
