use std::io::{stdin, stdout, Write};
use thiserror::Error;

const MEMORY_LIMIT: usize = 100;
type DataType = u64;

#[derive(Error, Debug)]
pub enum LMCErrors {
    #[error("No instructions were given to LMC")]
    NoInstructionsGiven,
    #[error("Too many instructions given! Not enough memory.")]
    TooManyInstructionsGiven,
    #[error("The program never halted. Note: This could also be triggered if the program jumped outside of the memory limits.")]
    ProgramDidntHalt,
    #[error("The program halted")]
    Halt,
    #[error("Please enter an integer")]
    InvalidInput(#[from] std::num::ParseIntError),
    #[error("Error reading from standard input")]
    IOError(#[from] std::io::Error),
    #[error("Tried to access memory out of bounds")]
    MemoryOutOfBounds,
    #[error("The instruction code read was not recognised; got {0}, limit is {1}")]
    InstructionCodeNotRecognised(i64, u64),
}

#[derive(PartialEq, Eq)]
pub enum Instruction {
    ADD(DataType),
    SUB(DataType),
    STA(DataType),
    LDA(DataType),
    BRA(DataType),
    BRZ(DataType),
    BRP(DataType),
    INP,
    OUT,
    HLT,
}

impl Into<DataType> for Instruction {
    fn into(self) -> DataType {
        match self {
            Instruction::ADD(addr) => 1 * 100 + addr,
            Instruction::SUB(addr) => 2 * 100 + addr,
            Instruction::STA(addr) => 3 * 100 + addr,
            Instruction::LDA(addr) => 5 * 100 + addr,
            Instruction::BRA(addr) => 6 * 100 + addr,
            Instruction::BRZ(addr) => 7 * 100 + addr,
            Instruction::BRP(addr) => 8 * 100 + addr,
            Instruction::INP => 9 * 100 + 1,
            Instruction::OUT => 9 * 100 + 2,
            Instruction::HLT => 000,
        }
    }
}

impl From<DataType> for Instruction {
    fn from(code: DataType) -> Instruction {
        let instr = ((code / 100) as f64).floor() as DataType;
        let payload = code % 100;

        match instr {
            0 => Instruction::HLT,
            1 => Instruction::ADD(payload),
            2 => Instruction::SUB(payload),
            3 => Instruction::STA(payload),
            5 => Instruction::LDA(payload),
            6 => Instruction::BRA(payload),
            7 => Instruction::BRZ(payload),
            8 => Instruction::BRP(payload),
            9 => match payload {
                1 => Instruction::INP,
                2 => Instruction::OUT,
                _ => panic!(
                    "{}",
                    LMCErrors::InstructionCodeNotRecognised(payload as i64, 2)
                ),
            },
            _ => panic!(
                "{:?}",
                LMCErrors::InstructionCodeNotRecognised(instr as i64, 9)
            ),
        }
    }
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
            HLT => return Err(LMCErrors::Halt),
            ADD(addr) => {
                self.accumulator += self
                    .memory
                    .get(addr as usize)
                    .ok_or(LMCErrors::MemoryOutOfBounds)? // made more complex because I want to use LMCError
            }
            SUB(addr) => {
                self.accumulator -= self
                    .memory
                    .get(addr as usize)
                    .ok_or(LMCErrors::MemoryOutOfBounds)? // made more complex because I want to use LMCError
            }
            STA(addr) => {
                *(self
                    .memory
                    .get_mut(addr as usize)
                    .ok_or(LMCErrors::MemoryOutOfBounds)?) = self.accumulator; // Is this good? No. Should I make a wrapper? Yes. Will I? Probably not
            }
            LDA(addr) => {
                self.accumulator = *(self
                    .memory
                    .get_mut(addr as usize)
                    .ok_or(LMCErrors::MemoryOutOfBounds)?);
            }
            BRA(addr) => self.pc = addr,
            BRZ(addr) => {
                if self.accumulator == 0 {
                    self.pc = addr
                }
            }
            BRP(_addr) => {
                panic!("Negative numbers not yet implemented!");
                // if self.accumulator > 0 {
                //     self.pc = addr
                // }
            }
            INP => {
                let mut buf = String::new();
                print!("(PC @ {}) Input: ", self.pc);
                _ = stdout().flush(); // Would be nice if there was an easier way to do this..
                stdin().read_line(&mut buf)?;
                let as_int: DataType = buf.trim().parse()?;
                self.accumulator = as_int;
            }
        }

        Ok(())
    }

    pub fn load(&mut self, program: Vec<Instruction>) -> Result<(), LMCErrors> {
        if program.len() <= 0 {
            // If program.len() is less than 0 I'm deeply concerned but may as well :)
            return Err(LMCErrors::NoInstructionsGiven);
        }

        if program.len() > MEMORY_LIMIT {
            return Err(LMCErrors::TooManyInstructionsGiven);
        }

        for (idx, instr) in program.into_iter().enumerate() {
            self.memory[idx] = instr.into();
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), LMCErrors> {
        loop {
            let data = *(self // Ive written this code so many times, its so gross but I cba to change it
                .memory
                .get(self.pc as usize)
                .ok_or(LMCErrors::ProgramDidntHalt)?); // Fetch
            let instr: Instruction = data.into(); // Decode
            self.pc += 1;

            match self.execute(instr) {
                // Execute
                Ok(_) => {}
                Err(LMCErrors::Halt) => return Ok(()),
                Err(err) => return Err(err), // Could just do `err => return err` but I like this more
            }
        } // Cycle

        // Err(LMCErrors::ProgramDidntHalt)
    }
}
