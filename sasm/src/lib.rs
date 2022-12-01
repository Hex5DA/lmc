use std::fs::read_to_string;
use std::io;
use thiserror::Error;

use crate::compiler::compile;
use crate::parser::parse;

mod compiler;
mod parser;
mod lex;

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    ADD(AddrType),
    SUB(AddrType),
    STA(AddrType),
    LDA(AddrType),
    BRA(AddrType),
    BRZ(AddrType),
    BRP(AddrType),
    INP,
    OUT,
    HLT,
}

#[derive(Error, Debug)]
pub enum SasmErrors {
    #[error("The file given does not exist")]
    FileDoesNotExist(#[from] io::Error),
    #[error("An unfamiliar token was encountered when lexing!")]
    LexemeNotRecognised,
    #[error("The instruction lexed was not recognised")]
    InstructionNotRecognised,
    #[error("No argument was passed to an instruction")]
    NoArgumentPassedToOp,
    #[error("Encountered an end of file while parsing")]
    UnexpectedEOF,
    #[error("No argument newline or comment followed an instruction")]
    NoArgNewlineOrComment,
    #[error("The instruction code read was not recognised; got {0}, limit is {1}")]
    InstructionCodeNotRecognised(i64, u64),
}

pub type DataType = i64;
pub type AddrType = u16;

pub fn process(path: &str) -> Result<Vec<DataType>, SasmErrors> {
    let mut contents = read_to_string(path)?;
    contents.push('\x04'); // Manually add an EOF character

    let tokens = lex::lex(contents)?;
    let instrs = parse(tokens)?;
    let compiled = compile(instrs)?;

    Ok(compiled)
}
