use super::{Instruction, DataType, AddrType, SasmErrors};

// turn Instruction into DataType for compilation
// turn DataType into Instruction for execution

impl Into<DataType> for Instruction {
    fn into(self) -> DataType {
        match self {
            Instruction::ADD(addr) => (1 * 100 + addr) as DataType,
            Instruction::SUB(addr) => (2 * 100 + addr) as DataType,
            Instruction::STA(addr) => (3 * 100 + addr) as DataType,
            Instruction::LDA(addr) => (5 * 100 + addr) as DataType,
            Instruction::BRA(addr) => (6 * 100 + addr) as DataType,
            Instruction::BRZ(addr) => (7 * 100 + addr) as DataType,
            Instruction::BRP(addr) => (8 * 100 + addr) as DataType,
            Instruction::INP => (9 * 100 + 1) as DataType,
            Instruction::OUT => (9 * 100 + 2) as DataType,
            Instruction::HLT => (0 * 100 + 0) as DataType,
        }
    }
}

impl From<DataType> for Instruction {
    fn from(code: DataType) -> Instruction {
        let op = ((code / 100) as f64).floor() as DataType;
        let payload = code % 100;

        match op {
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
                    SasmErrors::InstructionCodeNotRecognised(payload as i64, 2)
                ),
            },
            _ => panic!(
                "{}",
                SasmErrors::InstructionCodeNotRecognised(op as i64, 9)
            ),
        }
    }
}

pub fn compile(instrs: Vec<Instruction>) -> Result<Vec<DataType>, SasmErrors> {
    let mut compiled: Vec<DataType> = Vec::new();
    for instr in instrs {
        compiled.push(instr.into());
    }

    Ok(compiled)
}