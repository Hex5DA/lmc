use std::fs::read_to_string;
use std::io;
use thiserror::Error;

use crate::parser::parse;

mod parser;
mod lex;

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
}

// type DataType = i64;
type AddrType = u64;

pub fn run(path: &str) -> Result<(), SasmErrors> {
    let mut contents = read_to_string(path)?;
    contents.push('\x04'); // Manually add an EOF character
    let tokens = lex::lex(contents)?;

    println!("Tokens: {:?}", tokens);

    let instrs = parse(tokens);
    println!("Instructions: {:?}", instrs);

    Ok(())
}
