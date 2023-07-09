use core::slice::Iter;

use super::error::LexerError;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Mnemonic {
    Any,
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
pub enum TokenType {
    Immediate(u16),
    Numeral(u16),
    Mnemonic(Mnemonic),
    Register(Register),
    Comma,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub pos: (usize, usize),
    pub token_type: TokenType,
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
                true => {
                    return Ok(Token {
                        token_type: TokenType::Immediate(number),
                        pos: position,
                    })
                }
                false => {
                    return Ok(Token {
                        token_type: TokenType::Numeral(number),
                        pos: position,
                    })
                }
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
                true => {
                    return Ok(Token {
                        token_type: TokenType::Immediate(number),
                        pos: position,
                    })
                }
                false => {
                    return Ok(Token {
                        token_type: TokenType::Numeral(number),
                        pos: position,
                    })
                }
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
                true => {
                    return Ok(Token {
                        token_type: Token::Immediate(number),
                        pos: position,
                    })
                }
                false => {
                    return Ok(Token {
                        token_type: Token::Numeral(number),
                        pos: position,
                    })
                }
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
            "ADC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Adc),
                pos: position,
            }),
            "AND" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::And),
                pos: position,
            }),
            "ASL" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Asl),
                pos: position,
            }),
            "BCC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bcc),
                pos: position,
            }),
            "BCS" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bcs),
                pos: position,
            }),
            "BEQ" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Beq),
                pos: position,
            }),
            "BIT" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bit),
                pos: position,
            }),
            "BMI" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bmi),
                pos: position,
            }),
            "BNE" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bne),
                pos: position,
            }),
            "BPL" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bpl),
                pos: position,
            }),
            "BRK" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Brk),
                pos: position,
            }),
            "BVC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bvc),
                pos: position,
            }),
            "BVS" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Bvs),
                pos: position,
            }),
            "CLC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Clc),
                pos: position,
            }),
            "CLD" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Cld),
                pos: position,
            }),
            "CLI" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Cli),
                pos: position,
            }),
            "CLV" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Clv),
                pos: position,
            }),
            "CMP" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Cmp),
                pos: position,
            }),
            "CPX" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Cpx),
                pos: position,
            }),
            "CPY" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Cpy),
                pos: position,
            }),
            "DEC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Dec),
                pos: position,
            }),
            "DEX" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Dex),
                pos: position,
            }),
            "DEY" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Dey),
                pos: position,
            }),
            "EOR" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Eor),
                pos: position,
            }),
            "INC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Inc),
                pos: position,
            }),
            "INX" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Inx),
                pos: position,
            }),
            "INY" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Iny),
                pos: position,
            }),
            "JMP" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Jmp),
                pos: position,
            }),
            "JSR" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Jsr),
                pos: position,
            }),
            "LDA" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Lda),
                pos: position,
            }),
            "LDX" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Ldx),
                pos: position,
            }),
            "LDY" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Ldy),
                pos: position,
            }),
            "LSR" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Lsr),
                pos: position,
            }),
            "NOP" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Nop),
                pos: position,
            }),
            "ORA" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Ora),
                pos: position,
            }),
            "PHA" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Pha),
                pos: position,
            }),
            "PHP" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Php),
                pos: position,
            }),
            "PLA" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Pla),
                pos: position,
            }),
            "ROL" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Rol),
                pos: position,
            }),
            "ROR" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Ror),
                pos: position,
            }),
            "RTI" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Rti),
                pos: position,
            }),
            "RTS" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Rts),
                pos: position,
            }),
            "SBC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Sbc),
                pos: position,
            }),
            "SEC" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Sec),
                pos: position,
            }),
            "SED" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Sed),
                pos: position,
            }),
            "SEI" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Sei),
                pos: position,
            }),
            "STA" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Sta),
                pos: position,
            }),
            "STX" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Stx),
                pos: position,
            }),
            "STY" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Sty),
                pos: position,
            }),
            "TAX" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Tax),
                pos: position,
            }),
            "TAY" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Tay),
                pos: position,
            }),
            "TSX" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Tsx),
                pos: position,
            }),
            "TSY" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Tsy),
                pos: position,
            }),
            "TXA" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Txa),
                pos: position,
            }),
            "TXS" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Txs),
                pos: position,
            }),
            "TYA" => Ok(Token {
                token_type: Token::Mnemonic(Mnemonic::Tya),
                pos: position,
            }),
            "PC" => Ok(Token {
                token_type: Token::Register(Register::Pc),
                pos: position,
            }),
            "AC" => Ok(Token {
                token_type: Token::Register(Register::Ac),
                pos: position,
            }),
            "X" => Ok(Token {
                token_type: Token::Register(Register::X),
                pos: position,
            }),
            "Y" => Ok(Token {
                token_type: Token::Register(Register::Y),
                pos: position,
            }),
            "SR" => Ok(Token {
                token_type: Token::Register(Register::Sr),
                pos: position,
            }),
            "SP" => Ok(Token {
                token_type: Token::Register(Register::Sp),
                pos: position,
            }),
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
