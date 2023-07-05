use crate::assembler::lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token")]
    UnexpectedToken {
        pos: (usize, usize),
        expected: Vec<Token>,
        found: Token,
    },
    #[error(transparent)]
    LexerError(#[from] LexerError),
}

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("Unexpected Symbol")]
    UnexpectedSymbol {
        pos: (usize, usize),
        expected: String,
        found: char,
    },
    #[error("Lexeme is not a valid token")]
    InvalidToken {
        pos: (usize, usize),
        expected: Vec<String>,
        found: String,
    },
    #[error("Malformed expression: Failed to parse integer")]
    IntegerParseError { pos: (usize, usize), lexeme: String },
}
