mod lexer;
mod parser;
mod table;

use lexer::tokens::{Token, TokenVariant};
use lexer::Lexer;
use parser::Parser;

fn main() {
    // let input = "int '12 (24) \"abc\\\"\" = 43; '_' 't' '\''";
    // let input = "create 'aa' table people(name varchar(2) primary key, age int);";
    let input = "= ";
    let mut lexer = Lexer::new(input);

    // while let Some(token) = lexer.next() {
    //     match token {
    //         Ok(token) => match token {
    //             TokenVariant::Bool(t) => println!("Bool {}", t.value),
    //             TokenVariant::Char(t) => println!("Char {}", t.value),
    //             TokenVariant::Integer(t) => println!("Integer {}", t.value),
    //             TokenVariant::Identifier(t) => println!("Identifier {}", t.value),
    //             TokenVariant::Keyword(t) => println!("Keyword {}", t.value),
    //             TokenVariant::Operator(t) => println!("Operator {}", t.value),
    //             TokenVariant::Punctuator(t) => println!("Punctuator {}", t.value),
    //             TokenVariant::String(t) => println!("String {}", t.value),
    //         },
    //         Err(err) => panic!("{}", err),
    //     }
    // }

    let mut parser = Parser::new(lexer);
}
