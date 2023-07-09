use bytes::{Bytes, BytesMut};

pub mod error;
pub mod lexer;

use error::ParseError;
use lexer::Token;

use self::lexer::Mnemonic;

pub fn assemble(src: &str) -> Result<Bytes, error::ParseError> {
    let mut tokens = lexer::get_tokens(src)?.iter();
    let bytes = BytesMut::with_capacity(2048);
    loop {
        match tokens.next() {
            Some(token) => match token {
                Token::Mnemonic(mnemonic) => {}
                _ => Err(ParseError::UnexpectedToken {
                    pos: token.pos,
                    expected: (vec![TokenType::Mnemonic(Mnemonic::Any)]),
                    found: token,
                }),
            },
            None => Ok(bytes.into()),
        }
    }
}
