use core::slice::Iter;

use super::error::LexerError;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Mnemonic {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Tsy,
    Txa,
    Txs,
    Tya,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Register {
    Pc,
    Ac,
    X,
    Y,
    Sr,
    Sp,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Immediate(u16),
    Numeral(u16),
    Mnemonic(Mnemonic),
    Register(Register),
    Comma,
}

fn seek_till<T: PartialEq>(target: T, iter: &mut Iter<T>) -> usize {
    let mut idx = 0usize;
    while match iter.next() {
        Some(next) => *next != target,
        None => false,
    } {
        idx += 1;
    }
    idx
}

fn make_token(lexeme: &str, position: (usize, usize)) -> Result<Token, LexerError> {
    let lexeme = lexeme.to_uppercase();
    let mut lexeme = lexeme.as_str();
    let mut immediate = false;
    if lexeme.starts_with("#") {
        immediate = true;
        lexeme = &lexeme[1..];
    }
    if lexeme.starts_with("$") {
        match u16::from_str_radix(&lexeme[1..], 16) {
            Ok(number) => match immediate {
                true => return Ok(Token::Immediate(number)),
                false => return Ok(Token::Numeral(number)),
            },
            Err(_) => {
                return Err(LexerError::IntegerParseError {
                    pos: position,
                    lexeme: lexeme.to_owned(),
                });
            }
        }
    } else if lexeme.starts_with("%") {
        match u16::from_str_radix(&lexeme[1..], 2) {
            Ok(number) => match immediate {
                true => return Ok(Token::Immediate(number)),
                false => return Ok(Token::Numeral(number)),
            },
            Err(_) => {
                return Err(LexerError::IntegerParseError {
                    pos: position,
                    lexeme: lexeme.to_owned(),
                });
            }
        }
    } else if (b'0'..b'9').contains(&lexeme.as_bytes()[0]) {
        match u16::from_str_radix(&lexeme, 10) {
            Ok(number) => match immediate {
                true => return Ok(Token::Immediate(number)),
                false => return Ok(Token::Numeral(number)),
            },
            Err(_) => {
                return Err(LexerError::IntegerParseError {
                    pos: position,
                    lexeme: lexeme.to_owned(),
                });
            }
        }
    } else {
        let token = match lexeme {
            "ADC" => Ok(Token::Mnemonic(Mnemonic::Adc)),
            "AND" => Ok(Token::Mnemonic(Mnemonic::And)),
            "ASL" => Ok(Token::Mnemonic(Mnemonic::Asl)),
            "BCC" => Ok(Token::Mnemonic(Mnemonic::Bcc)),
            "BCS" => Ok(Token::Mnemonic(Mnemonic::Bcs)),
            "BEQ" => Ok(Token::Mnemonic(Mnemonic::Beq)),
            "BIT" => Ok(Token::Mnemonic(Mnemonic::Bit)),
            "BMI" => Ok(Token::Mnemonic(Mnemonic::Bmi)),
            "BNE" => Ok(Token::Mnemonic(Mnemonic::Bne)),
            "BPL" => Ok(Token::Mnemonic(Mnemonic::Bpl)),
            "BRK" => Ok(Token::Mnemonic(Mnemonic::Brk)),
            "BVC" => Ok(Token::Mnemonic(Mnemonic::Bvc)),
            "BVS" => Ok(Token::Mnemonic(Mnemonic::Bvs)),
            "CLC" => Ok(Token::Mnemonic(Mnemonic::Clc)),
            "CLD" => Ok(Token::Mnemonic(Mnemonic::Cld)),
            "CLI" => Ok(Token::Mnemonic(Mnemonic::Cli)),
            "CLV" => Ok(Token::Mnemonic(Mnemonic::Clv)),
            "CMP" => Ok(Token::Mnemonic(Mnemonic::Cmp)),
            "CPX" => Ok(Token::Mnemonic(Mnemonic::Cpx)),
            "CPY" => Ok(Token::Mnemonic(Mnemonic::Cpy)),
            "DEC" => Ok(Token::Mnemonic(Mnemonic::Dec)),
            "DEX" => Ok(Token::Mnemonic(Mnemonic::Dex)),
            "DEY" => Ok(Token::Mnemonic(Mnemonic::Dey)),
            "EOR" => Ok(Token::Mnemonic(Mnemonic::Eor)),
            "INC" => Ok(Token::Mnemonic(Mnemonic::Inc)),
            "INX" => Ok(Token::Mnemonic(Mnemonic::Inx)),
            "INY" => Ok(Token::Mnemonic(Mnemonic::Iny)),
            "JMP" => Ok(Token::Mnemonic(Mnemonic::Jmp)),
            "JSR" => Ok(Token::Mnemonic(Mnemonic::Jsr)),
            "LDA" => Ok(Token::Mnemonic(Mnemonic::Lda)),
            "LDX" => Ok(Token::Mnemonic(Mnemonic::Ldx)),
            "LDY" => Ok(Token::Mnemonic(Mnemonic::Ldy)),
            "LSR" => Ok(Token::Mnemonic(Mnemonic::Lsr)),
            "NOP" => Ok(Token::Mnemonic(Mnemonic::Nop)),
            "ORA" => Ok(Token::Mnemonic(Mnemonic::Ora)),
            "PHA" => Ok(Token::Mnemonic(Mnemonic::Pha)),
            "PHP" => Ok(Token::Mnemonic(Mnemonic::Php)),
            "PLA" => Ok(Token::Mnemonic(Mnemonic::Pla)),
            "ROL" => Ok(Token::Mnemonic(Mnemonic::Rol)),
            "ROR" => Ok(Token::Mnemonic(Mnemonic::Ror)),
            "RTI" => Ok(Token::Mnemonic(Mnemonic::Rti)),
            "RTS" => Ok(Token::Mnemonic(Mnemonic::Rts)),
            "SBC" => Ok(Token::Mnemonic(Mnemonic::Sbc)),
            "SEC" => Ok(Token::Mnemonic(Mnemonic::Sec)),
            "SED" => Ok(Token::Mnemonic(Mnemonic::Sed)),
            "SEI" => Ok(Token::Mnemonic(Mnemonic::Sei)),
            "STA" => Ok(Token::Mnemonic(Mnemonic::Sta)),
            "STX" => Ok(Token::Mnemonic(Mnemonic::Stx)),
            "STY" => Ok(Token::Mnemonic(Mnemonic::Sty)),
            "TAX" => Ok(Token::Mnemonic(Mnemonic::Tax)),
            "TAY" => Ok(Token::Mnemonic(Mnemonic::Tay)),
            "TSX" => Ok(Token::Mnemonic(Mnemonic::Tsx)),
            "TSY" => Ok(Token::Mnemonic(Mnemonic::Tsy)),
            "TXA" => Ok(Token::Mnemonic(Mnemonic::Txa)),
            "TXS" => Ok(Token::Mnemonic(Mnemonic::Txs)),
            "TYA" => Ok(Token::Mnemonic(Mnemonic::Tya)),
            "PC" => Ok(Token::Register(Register::Pc)),
            "AC" => Ok(Token::Register(Register::Ac)),
            "X" => Ok(Token::Register(Register::X)),
            "Y" => Ok(Token::Register(Register::Y)),
            "SR" => Ok(Token::Register(Register::Sr)),
            "SP" => Ok(Token::Register(Register::Sp)),
            _ => Err(LexerError::InvalidToken {
                pos: position,
                expected: vec![
                    "Mnemonic".to_string(),
                    "Register".to_string(),
                    "Integer".to_string(),
                ],
                found: lexeme.to_string(),
            }),
        };
        return token;
    }
}

pub fn get_tokens(src: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = vec![];
    let mut iter = src.as_bytes().iter();
    let mut lexeme = String::with_capacity(60);
    let (mut row, mut col) = (0usize, 0usize);
    loop {
        match iter.next() {
            Some(next) => match next {
                b';' => {
                    if !lexeme.is_empty() {
                        tokens.push(make_token(&lexeme, (row, col))?);
                        lexeme.clear();
                    }
                    seek_till(b'\n', &mut iter);
                    col = 0;
                    row += 1;
                }
                b' ' | b'\t' | b'\n' => {
                    if *next == b'\n' {
                        col = 0;
                        row += 1;
                    } else {
                        col += 1;
                    }
                    if !lexeme.is_empty() {
                        tokens.push(make_token(&lexeme, (row, col))?);
                        lexeme.clear();
                    }
                }
                _ => {
                    lexeme.push(*next as char);
                    col += 1;
                }
            },
            None => {
                if !lexeme.is_empty() {
                    tokens.push(make_token(&lexeme, (row, col))?);
                }
                break;
            }
        }
    }
    Ok(tokens)
}
