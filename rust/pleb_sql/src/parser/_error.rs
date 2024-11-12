pub enum ParsingError {
    Lexing(String),
    ParseInt(String),
    Syntax(String),
}

impl From<crate::lexer::Error> for ParsingError {
    fn from(error: crate::lexer::Error) -> ParsingError {
        ParsingError::Lexing(format!("{}", error))
    }
}

impl From<std::num::ParseIntError> for ParsingError {
    fn from(error: std::num::ParseIntError) -> ParsingError {
        ParsingError::ParseInt(format!("{}", error))
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Lexing(message) => message,
            Self::ParseInt(message) => message,
            Self::Syntax(message) => message,
        };
        write!(f, "{}", message)
    }
}
