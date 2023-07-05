mod assembler;

#[cfg(test)]
mod tests {

    use crate::assembler::lexer;

    use super::*;

    #[test]
    fn test_lexer() {
        let code = r#"
            LDA #$10AB ; load accumulator
            ADC #%0110 ; add 6 to acc
            STA #$10AC ; store accumulator
        "#;
        let tokens = lexer::get_tokens(code).unwrap();
        assert_eq!(tokens[0], lexer::Token::Mnemonic(lexer::Mnemonic::Lda));
        assert_eq!(tokens[1], lexer::Token::Immediate(0x10ab));
        assert_eq!(tokens[2], lexer::Token::Mnemonic(lexer::Mnemonic::Adc));
        assert_eq!(tokens[3], lexer::Token::Immediate(0b0110));
        assert_eq!(tokens[4], lexer::Token::Mnemonic(lexer::Mnemonic::Sta));
        assert_eq!(tokens[5], lexer::Token::Immediate(0x10ac));
    }
}
