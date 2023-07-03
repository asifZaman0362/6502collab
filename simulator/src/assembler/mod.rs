use bytes::{Bytes, BytesMut};

pub mod error;
pub mod lexer;

pub fn assemble(src: &str) -> Result<Bytes, error::ParseError> {
    let tokens = lexer::get_tokens(src)?;
    let bytes = BytesMut::with_capacity(1024);
    Ok(bytes.into())
}
