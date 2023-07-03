use crate::assembler::lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token")]
    UnexpectedToken {
        pos: (u8, u8),
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
        pos: (u8, u8),
        expected: String,
        found: char,
    },
}
