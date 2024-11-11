use crate::lexer::Lexer;

use super::{nodes::NodeVariant, Parser};

#[test]
fn create_table_success() {
    let input = "create table my_table (col_1 bool, col_2 int, col_3 char(5));";
    let mut parser = Parser {
        lexer: Lexer::new(input),
    };

    let result = parser.parse_statement();

    let table = match result {
        Ok(NodeVariant::TableDefinition(table)) => table,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(table.name, "my_table");
}
