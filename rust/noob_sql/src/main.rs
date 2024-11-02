mod lib;

use lib::lexical_analyzer::LexicalAnalyzer;
use lib::token::Token;

fn main() {
    let input = "1 2 3";
    let mut analyzer = LexicalAnalyzer::new(input);

    while let Some(num) = analyzer.next() {
        println!("({})", num.get_lexeme())
    }
}
