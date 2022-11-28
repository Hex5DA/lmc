use std::io::{stdin, stdout, Write};
use thiserror::Error;

const MEMORY_LIMIT: usize = 100;
type DataType = u64;

#[derive(Error, Debug)]
pub enum LMCErrors {
    #[error("No instructions were given to LMC")]
    NoInstructionsGiven,
    #[error("The program never halted")]
    ProgramDidntHalt,
    #[error("The program halted")]
    Halt,
    #[error("Please enter an integer")]
    InvalidInput(#[from] std::num::ParseIntError),
    #[error("Error reading from standard input")]
    IOError(#[from] std::io::Error),
    #[error("Tried to access memory out of bounds")]
    MemoryOutOfBounds,
}

#[derive(PartialEq, Eq)]
pub enum Instruction {
    STA(DataType),
    INP,
    ADD(DataType),
    OUT,
    HALT,
}

pub struct LMC {
    pc: u64,
    accumulator: DataType,
    memory: [DataType; MEMORY_LIMIT],
}

impl LMC {
    pub fn new() -> Self {
        Self {
            pc: 0,
            accumulator: 0,
            memory: [0; MEMORY_LIMIT],
        }
    }

    fn execute(&mut self, instr: Instruction) -> Result<(), LMCErrors> {
        use Instruction::*;
        match instr {
            OUT => println!("(PC @ {}) Output: {}", self.pc, self.accumulator),
            HALT => return Err(LMCErrors::Halt),
            ADD(addr) => {
                self.accumulator += self
                    .memory
                    .get(addr as usize)
                    .ok_or(LMCErrors::MemoryOutOfBounds)? // made more complex because I want to use LMCError
            },
            STA(addr) => {
                *(self
                    .memory
                    .get_mut(addr as usize)
                    .ok_or(LMCErrors::MemoryOutOfBounds)?) = self.accumulator; // Is this good? No. Should I make a wrapper? Yes. Will I? Probably not
            },
            INP => {
                let mut buf = String::new();
                print!("(PC @ {}) Input: ", self.pc);
                _ = stdout().flush();
                stdin().read_line(&mut buf)?;
                let as_int: DataType = buf.trim().parse()?;
                self.accumulator = as_int;
            }
        }

        Ok(())
    }

    pub fn run(&mut self, program: Vec<Instruction>) -> Result<(), LMCErrors> {
        if program.len() <= 0 { // If program.len() is less than 0 I'm deeply concerned but may as well :)
            return Err(LMCErrors::NoInstructionsGiven);
        }

        for instr in program {
            self.pc += 1;
            match self.execute(instr) {
                Ok(_) => {},
                Err(LMCErrors::Halt) => return Ok(()),
                Err(err) => return Err(err) // Could just do `err => return err` but I like this more
            }
        }

        Err(LMCErrors::ProgramDidntHalt)
    }
}
