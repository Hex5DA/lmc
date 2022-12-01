use crate::{AddrType, SasmErrors, Instruction};
use crate::lex::Lexemes;

impl Instruction {
    fn from_op_lexeme(op: String, arg: Option<AddrType>) -> Result<Self, SasmErrors> {
        use Instruction::*;

        Ok(match op.to_lowercase().as_str() {
            "add" => ADD(arg.ok_or(SasmErrors::NoArgumentPassedToOp)?),
            "sub" => SUB(arg.ok_or(SasmErrors::NoArgumentPassedToOp)?),
            "sta" => STA(arg.ok_or(SasmErrors::NoArgumentPassedToOp)?),
            "lda" => LDA(arg.ok_or(SasmErrors::NoArgumentPassedToOp)?),
            "bra" => BRA(arg.ok_or(SasmErrors::NoArgumentPassedToOp)?),
            "brz" => BRZ(arg.ok_or(SasmErrors::NoArgumentPassedToOp)?),
            "brp" => BRP(arg.ok_or(SasmErrors::NoArgumentPassedToOp)?),
            "inp" => INP,
            "out" => OUT,
            "hlt" => HLT,
            _ => return Err(SasmErrors::InstructionNotRecognised),
        })
    }
}

pub fn parse(lexemes: Vec<Lexemes>) -> Result<Vec<Instruction>, SasmErrors> {
    let mut instrs: Vec<Instruction> = Vec::new();
    for (idx, lexeme) in lexemes.iter().enumerate() {
        let instr = match lexeme.to_owned() {
            Lexemes::OP(name) => {
                let next = lexemes.get(idx + 1).ok_or(SasmErrors::UnexpectedEOF)?.to_owned();
                
                Some(match next {
                    Lexemes::ARG(val) => Instruction::from_op_lexeme(name, Some(val))?,
                    Lexemes::NEWLINE => Instruction::from_op_lexeme(name, None)?,
                    _ => return Err(SasmErrors::NoArgNewlineOrComment),
                })
            }
            Lexemes::NEWLINE => None,
            Lexemes::ARG(_) => None,
            Lexemes::LABEL(_name) => todo!()
        };

        if let Some(instr) = instr {
            instrs.push(instr);
        }
    }

    Ok(instrs)
}