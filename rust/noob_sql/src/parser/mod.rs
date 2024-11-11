use crate::lexer::{
    tokens::{keyword_token::Keyword, TokenVariant},
    Lexer,
};
use crate::{
    lexer::tokens::punctuator_token::Punctuator,
    table::{column::Column, data_type::DataType, Table},
};
use nodes::NodeVariant;

pub mod _error;
mod _tests;
pub mod nodes;

pub type Error = _error::ParsingError;
pub type Result<T> = core::result::Result<T, Error>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new(lexer: Lexer) -> Parser<'_> {
        Parser { lexer }
    }

    pub fn parse_statement(&mut self) -> Result<NodeVariant> {
        let keyword = self.read_keyword()?;

        match keyword {
            Keyword::Create => self.parse_create_statement(),
            _ => Err(Error::Syntax("Expected statement".to_string())),
        }
    }

    pub fn parse_create_statement(&mut self) -> Result<NodeVariant> {
        self.consume_keyword(Keyword::Table)?;
        let table_name = self.read_identifier()?;
        self.consume_punctuator(Punctuator::ParenOpen)?;

        let mut table = Table::new(table_name.to_string());

        loop {
            let name = self.read_identifier()?.to_string();
            let data_type = self.read_type()?;
            let punctuator = self.read_punctuator()?;
            table.columns.push(Column { name, data_type });

            if punctuator == Punctuator::ParenClose {
                break;
            }
        }

        self.consume_punctuator(Punctuator::Semicolon)?;

        Ok(NodeVariant::TableDefinition(table))
    }

    fn read_keyword(&mut self) -> Result<Keyword> {
        let token = self.read_token()?;

        match token {
            TokenVariant::Keyword(token) => Ok(token.value),
            _ => Err(Error::Syntax(format!("Expected keyword"))),
        }
    }

    fn read_type(&mut self) -> Result<DataType> {
        let token = self.read_token()?;

        let keyword = match token {
            TokenVariant::Keyword(token) => token.value,
            _ => return Err(Error::Syntax(format!("Expected type"))),
        };

        let data_type = match keyword {
            Keyword::Char => DataType::Char(0),
            Keyword::Bool => DataType::Bool,
            Keyword::Int => DataType::Int,
            _ => return Err(Error::Syntax(format!("Expected type"))),
        };

        if data_type != DataType::Char(0) {
            return Ok(data_type);
        }

        self.consume_punctuator(Punctuator::ParenOpen)?;
        let size = self.read_size()?;
        self.consume_punctuator(Punctuator::ParenClose)?;

        Ok(DataType::Char(size))
    }

    fn read_size(&mut self) -> Result<u8> {
        let token = self.read_token()?;

        let value = match token {
            TokenVariant::Integer(token) => token.value.parse::<u8>()?,
            _ => return Err(Error::Syntax(format!("Expected size"))),
        };

        Ok(value)
    }

    fn read_punctuator(&mut self) -> Result<Punctuator> {
        let token = self.read_token()?;

        match token {
            TokenVariant::Punctuator(token) => Ok(token.value),
            _ => return Err(Error::Syntax(format!("Expected punctuator"))),
        }
    }

    fn read_identifier(&mut self) -> Result<String> {
        let token = self.read_token()?;

        match token {
            TokenVariant::Identifier(token) => Ok(token.value.to_string()),
            _ => Err(Error::Syntax(format!("Expected identifier"))),
        }
    }

    fn read_token(&mut self) -> Result<TokenVariant> {
        let result = self.lexer.next();
        let token_result = match result {
            Some(token) => token,
            None => return Err(Error::Syntax(format!("Expected token"))),
        };

        match token_result {
            Ok(token) => Ok(token),
            Err(err) => return Err(err.into()),
        }
    }

    fn consume_punctuator(&mut self, expected: Punctuator) -> Result<()> {
        let punctuator = self.read_punctuator()?;

        match punctuator == expected {
            true => Ok(()),
            false => Err(Error::Syntax(format!("Expected punctuator {}", expected))),
        }
    }

    fn consume_keyword(&mut self, expected: Keyword) -> Result<()> {
        let keyword = self.read_keyword()?;

        match keyword == expected {
            true => Ok(()),
            false => Err(Error::Syntax(format!("Expected keyword {}", expected))),
        }
    }
}
