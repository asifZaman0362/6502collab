use bytes::{Bytes, BytesMut};

pub mod error;
pub mod lexer;

use error::ParseError;
use lexer::{Token, TokenType};

use self::lexer::Mnemonic;

pub fn assemble(src: &str) -> Result<Bytes, error::ParseError> {
    let tokens = lexer::get_tokens(src)?;
    let mut iter = tokens.iter();
    let bytes = BytesMut::with_capacity(2048);
    loop {
        return match iter.next() {
            Some(token) => match &token.token_type {
                TokenType::Mnemonic(mnemonic) => Ok(Bytes::new()),
                _ => Err(ParseError::UnexpectedToken {
                    pos: token.pos,
                    expected: (vec![TokenType::Mnemonic(Mnemonic::Any)]),
                    found: token.clone(),
                }),
            },
            None => Ok(bytes.into()),
        };
    }
}
