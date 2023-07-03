use super::error::LexerError;

#[derive(Debug)]
pub enum Mnemonic {
    LDA,
    LDAX,
    BRK,
    JMP,
}

#[derive(Debug)]
pub enum Token {
    LiteralHexByte(u8),
    LiteralHexAddress(u16),
    LiteralBinByte(u8),
    LiteralBinAddress(u16),
    Mnemonic(Mnemonic),
}

pub type TokenStream = Vec<Token>;

fn seek_till<T: PartialEq>(target: T, iter: &mut impl Iterator<Item = T>) {
    while match iter.next() {
        Some(next) => next != target,
        None => false,
    } {}
}

pub fn get_tokens(src: &str) -> Result<TokenStream, LexerError> {
    let mut tokens = vec![];
    let (mut start, mut end) = (0, 0);
    let mut bytes = src.bytes();
    loop {
        match bytes.next() {
            Some(next) => {
                if next == b';' {
                    seek_till(b'\n', &mut bytes);
                }
            }
            None => break,
        }
    }
    Ok(tokens)
}
