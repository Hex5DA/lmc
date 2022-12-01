use thiserror::Error;

#[derive(Error, Debug)]
pub enum LMCErrors {
    #[error("No instructions were given to LMC")]
    NoInstructionsGiven,
    #[error("Too many instructions given! Not enough memory.")]
    TooManyInstructionsGiven,
    #[error("The program halted")]
    Halt,
    #[error("Please enter an integer")]
    InvalidInput(#[from] std::num::ParseIntError),
    #[error("Error reading from standard input")]
    IOError(#[from] std::io::Error),
    #[error("Tried to access memory out of bounds.")]
    MemoryOutOfBounds,
}