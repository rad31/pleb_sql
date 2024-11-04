mod keyword;
mod lexical_analyzer;
mod punctuator;
mod token;

use lexical_analyzer::LexicalAnalyzer;
use token::{Token, TokenVariant};

fn main() {
    let input = "int 12 (24) \"abc\" 43;";
    let mut analyzer = LexicalAnalyzer::new(input);

    while let Some(token) = analyzer.next() {
        match token {
            TokenVariant::IntegerLiteral(t) => println!("IntegerLiteral {}", t.get_lexeme()),
            TokenVariant::Identifier(t) => println!("Identifier {}", t.get_lexeme()),
            TokenVariant::Keyword(t) => println!("Keyword {}", t.get_lexeme()),
            TokenVariant::Punctuator(t) => println!("Punctuator {}", t.get_lexeme()),
        }
    }
}
