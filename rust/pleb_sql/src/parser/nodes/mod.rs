use crate::{database::table::Table, database::Database};

pub enum NodeVariant {
    DatabaseDefinition(Database),
    TableDefinition(Table),
}

pub trait Node {
    fn parent(&self) -> Option<&NodeVariant>;
    fn children(&self) -> Vec<NodeVariant>;
    #[cfg(test)]
    fn token(&self) -> &crate::lexer::tokens::TokenVariant;
}
