mod lexer;

use lexer::tokens::{Token, TokenVariant};
use lexer::Lexer;

fn main() {
    // let input = "int '12 (24) \"abc\\\"\" = 43; '_' 't' '\''";
    // let input = "create 'aa' table people(name varchar(2) primary key, age int);";
    let input = "=";
    let mut lexer = Lexer::new(input);

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => match token {
                TokenVariant::Char(t) => println!("Char {}", t.get_lexeme()),
                TokenVariant::Integer(t) => println!("Integer {}", t.get_lexeme()),
                TokenVariant::Identifier(t) => println!("Identifier {}", t.get_lexeme()),
                TokenVariant::Keyword(t) => println!("Keyword {}", t.get_lexeme()),
                TokenVariant::Operator(t) => println!("Operator {}", t.get_lexeme()),
                TokenVariant::Punctuator(t) => println!("Punctuator {}", t.get_lexeme()),
                TokenVariant::String(t) => println!("String {}", t.get_lexeme()),
            },
            Err(err) => panic!("{}", err),
        }
    }
}
