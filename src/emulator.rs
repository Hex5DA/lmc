use crate::errors::*;

use sasm::{Instruction, DataType, AddrType};
use std::io::{stdin, stdout, Write};

const MEMORY_LIMIT: usize = 100;

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

    pub fn load(&mut self, program: Vec<DataType>) -> Result<(), LMCErrors> {
        if program.is_empty() {
            return Err(LMCErrors::NoInstructionsGiven);
        }

        if program.len() > MEMORY_LIMIT {
            return Err(LMCErrors::TooManyInstructionsGiven);
        }

        for (idx, instr) in program.into_iter().enumerate() {
            self.memory.set(idx as AddrType, instr.into())?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), LMCErrors> {
        loop {
            let data = self.memory.get(self.pc as AddrType)?; // Fetch
            let instr: Instruction = (*data).into(); // Decode
            self.pc += 1;

            match self.execute(instr) {
                // Execute
                Ok(_) => {}
                Err(LMCErrors::Halt) => return Ok(()),
                Err(err) => return Err(err), // Could just do `err => return err` but I like this more
            }
        } // Cycle
    }
}
