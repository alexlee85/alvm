use crate::assembler::Token;
use crate::instruction::Opcode;

use nom::types::CompleteStr;
use nom::*;

named!(pub opcode<CompleteStr, Token>,
    do_parse!(
        opcode: alpha1 >> 
        (
            Token::Op{code: Opcode::from(opcode)}
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_opcode_load() {
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOAD});
        assert_eq!(rest, CompleteStr(""));

        let result = opcode(CompleteStr("xxxx"));
        assert_eq!(
            result, 
            Ok((CompleteStr(""), Token::Op{code: Opcode::IGL}))
        );
    }
}