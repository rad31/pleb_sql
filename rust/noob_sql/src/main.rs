mod lexical_analyzer;
mod token;

use lexical_analyzer::{LexicalAnalyzer, TokenVariant};
use token::Token;

fn main() {
    let input = "12 (24) \"abc\" 43;";
    let mut analyzer = LexicalAnalyzer::new(input);

    while let Some(token) = analyzer.next() {
        match token {
            TokenVariant::IntegerLiteral(t) => println!("{}", t.get_lexeme()),
            TokenVariant::Identifier(t) => println!("{}", t.get_lexeme()),
            TokenVariant::Punctuator(t) => println!("{}", t.get_lexeme()),
        }
    }
}
