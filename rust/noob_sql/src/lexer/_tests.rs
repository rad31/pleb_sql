use crate::lexer::Lexer;

use super::tokens::{
    keyword_token::KeywordToken, operator_token::OperatorToken, punctuator_token::PunctuatorToken,
    TokenVariant, CHAR, OPERATOR, STRING,
};

#[test]
fn read_integer_success() {
    let input = "123";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::Integer(inner) => assert_eq!(inner.value, 123),
        _ => panic!(),
    }
}

#[test]
fn read_integer_success_negative() {
    let input = "-123";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::Integer(inner) => assert_eq!(inner.value, -123),
        _ => panic!(),
    }
}

#[test]
fn read_char_success() {
    let input = "'a'";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::Char(inner) => assert_eq!(inner.value, 'a'),
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
        TokenVariant::String(inner) => assert_eq!(inner.value, "abc"),
        _ => panic!(),
    }
}

#[test]
fn read_string_success_empty_string() {
    let input = "\"\"";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::String(inner) => assert_eq!(inner.value, ""),
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
        TokenVariant::Identifier(inner) => assert_eq!(inner.value, input),
        _ => panic!(),
    }
}

#[test]
fn read_identifier_success_keyword_as_substring() {
    let input = "int_name";
    let mut lexer = Lexer::new(input);
    let token = lexer.next();

    match token.unwrap().unwrap() {
        TokenVariant::Identifier(inner) => assert_eq!(inner.value, input),
        _ => panic!(),
    }
}

#[test]
fn read_keyword_success() {
    for (keyword, value) in KeywordToken::get_map().iter() {
        let mut lexer = Lexer::new(keyword);
        let token = lexer.next();

        match token.unwrap().unwrap() {
            TokenVariant::Keyword(inner) => assert_eq!(inner.value, *value),
            _ => panic!(),
        }
    }
}

#[test]
fn read_operator_success() {
    for (operator, value) in OperatorToken::get_map().iter() {
        let mut lexer = Lexer::new(operator);
        let token = lexer.next();

        match token.unwrap().expect(operator) {
            TokenVariant::Operator(inner) => assert_eq!(inner.value, *value),
            _ => panic!("{}", operator),
        }
    }
}

#[test]
fn read_operator_error_does_not_exist() {
    let operator = "=>";
    let mut lexer = Lexer::new(operator);
    let token = lexer.next();

    match token.unwrap() {
        Err(err) => assert_eq!(err.variant, OPERATOR),
        Ok(o) => panic!("{}", o),
    }
}

#[test]
fn read_punctuator_success() {
    for (punctuator, value) in PunctuatorToken::get_map().iter() {
        let punctuator = &punctuator.to_string();
        let mut lexer = Lexer::new(punctuator);
        let token = lexer.next();

        match token.unwrap().unwrap() {
            TokenVariant::Punctuator(inner) => assert_eq!(inner.value, *value),
            _ => panic!(),
        }
    }
}

#[test]
fn read_bool_success() {
    for (lexeme, value) in [("true", true), ("false", false)].iter() {
        let mut lexer = Lexer::new(lexeme);
        let token = lexer.next();

        match token.unwrap().unwrap() {
            TokenVariant::Bool(inner) => assert_eq!(inner.value, *value),
            _ => panic!(),
        }
    }
}

#[test]
fn read_sql_command_success() {
    let commands = [
        "create table table_name(col_1 int primary key, col_2 bool, col_3 varchar(8));",
        "insert into table_name values(0, true, \"abcdefgh\");",
        "select * from table_name where col_1 = 0;",
        "update table_name set col_3 = \"hgfedcba\" where col_1 = 0;",
    ];
    for commmand in commands {
        let mut lexer = Lexer::new(commmand);
        while let Some(token) = lexer.next() {
            assert!(token.is_ok());
        }
    }
}
