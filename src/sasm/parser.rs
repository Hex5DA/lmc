use super::lex::Lexemes;
use super::{AddrType, Instruction};
use crate::errors::SasmErrors;

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
            op @ ("inp" | "out" | "hlt") => {
                if !arg.is_none() {
                    return Err(SasmErrors::UnexpectedArgPassedToOp);
                }
                match op {
                    "inp" => INP,
                    "out" => OUT,
                    "hlt" => HLT,
                    _ => unreachable!(),
                }
            },
            unknown => return Err(SasmErrors::InstructionNotRecognised(unknown.to_string())),
        })
    }
}

pub fn parse(lexemes: Vec<Lexemes>) -> Result<Vec<Instruction>, SasmErrors> {
    let mut instrs: Vec<Instruction> = Vec::new();

    for (idx, lexeme) in lexemes.iter().enumerate() {
        let instr = match lexeme.to_owned() {
            // TODO: Remove args when encountered whilst parsing OPs and change ARG -> unreachable!()
            Lexemes::Op(name) => {
                let next = lexemes
                    .get(idx + 1)
                    .ok_or(SasmErrors::UnexpectedEOF)?
                    .to_owned();

                Some(match next {
                    Lexemes::Arg(val) => Instruction::from_op_lexeme(name, Some(val))?,
                    Lexemes::Newline => Instruction::from_op_lexeme(name, None)?,
                    Lexemes::Label(label) => {
                        println!("Label '{label}' enocuntered");
                        Instruction::from_op_lexeme(name, Some(99))?
                    },
                    _ => return Err(SasmErrors::NoArgNewlineOrComment),
                })
            },
            Lexemes::Arg(_) => None,
            Lexemes::Newline => None,
            Lexemes::Label(_) | Lexemes::DecLabel(_) => unreachable!(),
        };

        if let Some(instr) = instr {
            instrs.push(instr);
        }
    }

    Ok(instrs)
}
