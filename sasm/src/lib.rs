use std::fs::read_to_string;
use std::io;
use thiserror::Error;

mod lex;

#[derive(Error, Debug)]
pub enum SasmErrors {
    #[error("The file given does not exist")]
    FileDoesNotExist(#[from] io::Error),
    #[error("An unfamiliar token was encountered when lexing!")]
    LexemeNotRecognised,
}

type DataType = i64;

pub fn run(path: &str) -> Result<(), SasmErrors> {
    let mut contents = read_to_string(path)?;
    contents.push('\x04'); // Manually add an EOF character
    let tokens = lex::lex(contents)?;

    println!("Tokens: {:?}", tokens);

    Ok(())
}
