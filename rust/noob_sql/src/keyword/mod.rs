use std::collections::HashMap;

#[derive(Copy, Clone)]
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

pub static CREATE: &str = "create";
pub static TABLE: &str = "table";
pub static PRIMARY: &str = "primary";
pub static FOREIGN: &str = "foreign";
pub static KEY: &str = "key";
pub static INSERT: &str = "insert";
pub static INTO: &str = "into";
pub static VALUES: &str = "values";
pub static SELECT: &str = "select";
pub static FROM: &str = "from";
pub static UPDATE: &str = "update";
pub static SET: &str = "set";
pub static WHERE: &str = "where";
pub static AND: &str = "and";
pub static OR: &str = "or";
pub static BOOL: &str = "bool";
pub static INT: &str = "int";
pub static CHAR: &str = "char";
pub static VARCHAR: &str = "varchar";
