use crate::table::Table;

pub mod table_definition_node;

pub enum NodeVariant {
    TableDefinition(Table),
}

pub trait Node {
    fn parent(&self) -> Option<&NodeVariant>;
    fn children(&self) -> Vec<NodeVariant>;
    #[cfg(test)]
    fn token(&self) -> &crate::lexer::tokens::TokenVariant;
}
