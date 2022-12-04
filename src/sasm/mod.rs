use std::fs::read_to_string;

pub use crate::emulator::{Instruction, DataType, AddrType};
pub use crate::errors::SasmErrors;

mod compiler;
mod parser;
mod lex;

use compiler::compile;
use parser::parse;
use lex::lex;

pub fn process(path: String) -> Result<Vec<DataType>, SasmErrors> {
    let mut contents = read_to_string(path)?;
    contents.push('\x04'); // Manually add an EOF character

    let tokens = lex(contents)?;
    let instrs = parse(tokens)?;
    let compiled = compile(instrs)?;

    Ok(compiled)
}
