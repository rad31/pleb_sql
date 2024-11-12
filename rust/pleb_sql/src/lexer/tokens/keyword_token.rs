use std::collections::HashMap;

use crate::lexer::symbols::UNDERSCORE;

use super::Token;

#[derive(Debug)]
pub struct KeywordToken {
    pub value: Keyword,
}

impl KeywordToken {
    pub fn get_map() -> HashMap<&'static str, Keyword> {
        [
            (CREATE, Keyword::Create),
            (DATABASE, Keyword::Database),
            (TABLE, Keyword::Table),
            (PRIMARY, Keyword::Primary),
            (FOREIGN, Keyword::Foreign),
            (KEY, Keyword::Key),
            (INSERT, Keyword::Insert),
            (INTO, Keyword::Into),
            (VALUES, Keyword::Values),
            (SELECT, Keyword::Select),
            (FROM, Keyword::From),
            (UPDATE, Keyword::Update),
            (SET, Keyword::Set),
            (WHERE, Keyword::Where),
            (AND, Keyword::And),
            (OR, Keyword::Or),
            (BOOL, Keyword::Bool),
            (INT, Keyword::Int),
            (CHAR, Keyword::Char),
            (VARCHAR, Keyword::Varchar),
        ]
        .iter()
        .cloned()
        .collect()
    }
}

impl Token for KeywordToken {
    fn is_start(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn is_end(c: char, _: Option<char>) -> bool {
        !c.is_ascii_alphanumeric() && c != UNDERSCORE
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Create,
    Database,
    Table,
    Primary,
    Foreign,
    Key,
    Insert,
    Into,
    Values,
    Select,
    From,
    Update,
    Set,
    Where,
    And,
    Or,
    Bool,
    Int,
    Char,
    Varchar,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Keyword::Create => CREATE,
            Keyword::Database => DATABASE,
            Keyword::Table => TABLE,
            Keyword::Primary => PRIMARY,
            Keyword::Foreign => FOREIGN,
            Keyword::Key => KEY,
            Keyword::Insert => INSERT,
            Keyword::Into => INTO,
            Keyword::Values => VALUES,
            Keyword::Select => SELECT,
            Keyword::From => FROM,
            Keyword::Update => UPDATE,
            Keyword::Set => SET,
            Keyword::Where => WHERE,
            Keyword::And => AND,
            Keyword::Or => OR,
            Keyword::Bool => BOOL,
            Keyword::Int => INT,
            Keyword::Char => CHAR,
            Keyword::Varchar => VARCHAR,
        };
        write!(f, "{}", value)
    }
}

pub const CREATE: &str = "create";
pub const DATABASE: &str = "database";
pub const TABLE: &str = "table";
pub const PRIMARY: &str = "primary";
pub const FOREIGN: &str = "foreign";
pub const KEY: &str = "key";
pub const INSERT: &str = "insert";
pub const INTO: &str = "into";
pub const VALUES: &str = "values";
pub const SELECT: &str = "select";
pub const FROM: &str = "from";
pub const UPDATE: &str = "update";
pub const SET: &str = "set";
pub const WHERE: &str = "where";
pub const AND: &str = "and";
pub const OR: &str = "or";
pub const BOOL: &str = "bool";
pub const INT: &str = "int";
pub const CHAR: &str = "char";
pub const VARCHAR: &str = "varchar";
