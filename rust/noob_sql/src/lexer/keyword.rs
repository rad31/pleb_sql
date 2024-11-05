use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Keyword {
    Create,
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

impl Keyword {
    pub fn get_map() -> HashMap<&'static str, Keyword> {
        [
            (CREATE, Keyword::Create),
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

pub const CREATE: &str = "create";
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
