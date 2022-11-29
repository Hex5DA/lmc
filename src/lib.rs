use std::io::{stdin, stdout, Write};
use thiserror::Error;

const MEMORY_LIMIT: usize = 100;
type DataType = i64;
type AddrType = u16;

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
    #[error("The instruction code read was not recognised; got {0}, limit is {1}")]
    InstructionCodeNotRecognised(i64, u64),
}

#[derive(PartialEq, Eq)]
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

impl Into<DataType> for Instruction {
    fn into(self) -> DataType {
        match self {
            Instruction::ADD(addr) => (1 * 100 + addr).into(),
            Instruction::SUB(addr) => (2 * 100 + addr).into(),
            Instruction::STA(addr) => (3 * 100 + addr).into(),
            Instruction::LDA(addr) => (5 * 100 + addr).into(),
            Instruction::BRA(addr) => (6 * 100 + addr).into(),
            Instruction::BRZ(addr) => (7 * 100 + addr).into(),
            Instruction::BRP(addr) => (8 * 100 + addr).into(),
            Instruction::INP => (9 * 100 + 1).into(),
            Instruction::OUT => (9 * 100 + 2).into(),
            Instruction::HLT => (0 * 100 + 0).into(),
        }
    }
}

impl From<DataType> for Instruction {
    fn from(code: DataType) -> Instruction {
        let instr = ((code / 100) as f64).floor() as DataType;
        let payload = code % 100;

        match instr {
            0 => Instruction::HLT,
            1 => Instruction::ADD(payload as AddrType),
            2 => Instruction::SUB(payload as AddrType),
            3 => Instruction::STA(payload as AddrType),
            5 => Instruction::LDA(payload as AddrType),
            6 => Instruction::BRA(payload as AddrType),
            7 => Instruction::BRZ(payload as AddrType),
            8 => Instruction::BRP(payload as AddrType),
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

struct Memory([DataType; MEMORY_LIMIT]);
impl Memory {
    fn new() -> Self {
        Self {
            0: [0; MEMORY_LIMIT],
        }
    }

    fn get(&self, addr: AddrType) -> Result<&DataType, LMCErrors> {
        self.0
            .get(addr as usize)
            .ok_or(LMCErrors::MemoryOutOfBounds)
    }

    fn set(&mut self, addr: AddrType, new: DataType) -> Result<(), LMCErrors> {
        *self
            .0
            .get_mut(addr as usize)
            .ok_or(LMCErrors::MemoryOutOfBounds)? = new;
        Ok(())
    }
}

pub struct LMC {
    pc: u64,
    accumulator: DataType,
    memory: Memory,
}

impl LMC {
    pub fn new() -> Self {
        Self {
            pc: 0,
            accumulator: 0,
            memory: Memory::new(),
        }
    }

    fn execute(&mut self, instr: Instruction) -> Result<(), LMCErrors> {
        use Instruction::*;
        match instr {
            OUT => println!("(PC @ {}) Output: {}", self.pc, self.accumulator),
            HLT => return Err(LMCErrors::Halt),
            ADD(addr) => self.accumulator += self.memory.get(addr)?,
            SUB(addr) => self.accumulator -= self.memory.get(addr)?,
            STA(addr) => self.memory.set(addr, self.accumulator)?,
            LDA(addr) => self.accumulator = *self.memory.get(addr)?,
            BRA(addr) => self.pc = addr as u64,
            BRZ(addr) => {
                if self.accumulator == 0 {
                    self.pc = addr as u64;
                }
            }
            BRP(addr) => {
                if self.accumulator > 0 {
                    self.pc = addr as u64;
                }
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
            self.memory.set(idx as u16, instr.into())?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), LMCErrors> {
        loop {
            let data = self.memory.get(self.pc as u16)?; // Fetch
            let instr: Instruction = (*data).into(); // Decode
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
