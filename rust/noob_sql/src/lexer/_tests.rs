use crate::lexer::Lexer;

use super::{
    keyword::Keyword,
    operator::Operator,
    punctuator::Punctuator,
    tokens::{TokenVariant, CHAR, OPERATOR, STRING},
};

#[test]
fn read_integer_success() {
    let input = "123";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::Integer(inner) => assert_eq!(inner.lexeme, input),
        _ => panic!(),
    }
}

#[test]
fn read_char_success() {
    let input = "'a'";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::Char(inner) => assert_eq!(inner.lexeme, "a"),
        _ => panic!(),
    }
}

#[test]
fn read_char_error_no_closing_quote() {
    let input = "'a";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap() {
        Ok(_) => panic!(),
        Err(err) => assert_eq!(err.variant, CHAR),
    }
}

#[test]
fn read_char_error_char_too_long() {
    let input = "'aa'";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap() {
        Ok(_) => panic!(),
        Err(err) => assert_eq!(err.variant, CHAR),
    }
}

#[test]
fn read_string_success() {
    let input = "\"abc\"";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::String(inner) => assert_eq!(inner.lexeme, "abc"),
        _ => panic!(),
    }
}

#[test]
fn read_string_success_empty_string() {
    let input = "\"\"";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::String(inner) => assert_eq!(inner.lexeme, ""),
        _ => panic!(),
    }
}

#[test]
fn read_string_error_no_closing_quote() {
    let input = "\"abc";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap() {
        Ok(_) => panic!(),
        Err(err) => assert_eq!(err.variant, STRING),
    }
}

#[test]
fn read_identifier_success() {
    let input = "variable_name";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::Identifier(inner) => assert_eq!(inner.lexeme, input),
        _ => panic!(),
    }
}

#[test]
fn read_keyword_success() {
    for keyword in Keyword::get_map().keys() {
        let mut lexer = Lexer::new(keyword);
        let token = lexer.next();

        match token.unwrap().unwrap() {
            TokenVariant::Keyword(inner) => assert_eq!(inner.lexeme, *keyword),
            _ => panic!(),
        }
    }
}

#[test]
fn read_operator_success() {
    for operator in Operator::get_map().keys() {
        let mut lexer = Lexer::new(operator);
        let token = lexer.next();

        match token.unwrap().unwrap() {
            TokenVariant::Operator(inner) => assert_eq!(inner.lexeme, *operator),
            _ => panic!("{}", operator),
        }
    }
}

#[test]
fn read_operator_error_terminated_early() {
    let operator = "<";
    let mut lexer = Lexer::new(operator);
    let token = lexer.next();

    match token.unwrap() {
        Err(err) => assert_eq!(err.variant, OPERATOR),
        Ok(_) => panic!(),
    }
}

#[test]
fn read_operator_error_does_not_exist() {
    let operator = "=>";
    let mut lexer = Lexer::new(operator);
    let token = lexer.next();

    match token.unwrap() {
        Err(err) => assert_eq!(err.variant, OPERATOR),
        Ok(_) => panic!(),
    }
}

#[test]
fn read_punctuator_success() {
    for punctuator in Punctuator::list() {
        let punctuator = &punctuator.to_string();
        let mut lexer = Lexer::new(punctuator);
        let token = lexer.next();

        match token.unwrap().unwrap() {
            TokenVariant::Punctuator(inner) => assert_eq!(inner.lexeme, *punctuator),
            _ => panic!("{}", punctuator),
        }
    }
}
