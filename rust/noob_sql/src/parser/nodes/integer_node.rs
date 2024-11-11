use crate::lexer::tokens::{integer_token::IntegerToken, TokenVariant};

use super::{Node, NodeVariant};

pub struct IntegerNode<'a> {
    pub parent: &'a NodeVariant<'a>,
    pub value: i32,
    #[cfg(test)]
    pub token: &'a TokenVariant<'a>,
}

impl Node for IntegerNode<'_> {
    fn parent(&self) -> Option<&NodeVariant> {
        Some(self.parent)
    }

    fn children(&self) -> Vec<NodeVariant> {
        vec![]
    }

    #[cfg(test)]
    fn token(&self) -> &TokenVariant {
        self.token
    }
}
