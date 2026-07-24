use std::fmt;

// Define an enum for the instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // Load / Store (0x01 - 0x04)

    /// LOAD Rd, Rn, offset
    Load(u8, u8, u8),

    /// STORE Rd, Rn, offset
    Store(u8, u8, u8),

    /// PUSH Rn
    Push(u8),

    /// POP Rd
    Pop(u8),

    // Arithmetic (0x10 - 0x17)

    /// ADD Rd, Rn, Rm
    Add(u8, u8, u8),

    /// SUB Rd, Rn, Rm
    Sub(u8, u8, u8),

    /// MUL Rd, Rn, Rm
    Mul(u8, u8, u8),

    /// DIV Rd, Rn, Rm
    Div(u8, u8, u8),

    /// MOD Rd, Rn, Rm
    Mod(u8, u8, u8),

    /// INC Rn
    Inc(u8),

    /// DEC Rn
    Dec(u8),

    /// NEG Rn
    Neg(u8),

    // Control Flow (0x20 - 0x23)

    /// JMP label
    Jmp(u8),

    /// JEQ label, Rn, Rm
    Jeq(u8, u8, u8),

    /// JNE label, Rn, Rm
    Jne(u8, u8, u8),

    /// RET
    Ret,

    // Memory Management (0x30 - 0x33)

    /// ALLOC size
    Alloc(u8),

    /// FREE ptr
    Free(u8),

    /// READ Rd, ptr
    Read(u8, u8),

    /// WRITE ptr, value
    Write(u8, u8),

    // Input / Output (0x40 - 0x43)

    /// IN Rd
    In(u8),

    /// OUT value
    Out(u8),

    /// READC Rd
    Readc(u8),

    /// WRITEC value
    Writec(u8),

    // Miscellaneous (0xFC - 0xFF)

    /// INFO
    Info,

    /// DEBUG
    Debug,

    /// HALT
    Halt,
    
    /// NOP
    Nop,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Load(rd, rn) => write!(f, "LOAD R{}, R{}", rd, rn),
            Instruction::Store(rd, rn) => write!(f, "STORE R{}, R{}", rd, rn),
            Instruction::Add(rd, rn) => write!(f, "ADD R{}, R{}", rd, rn),
            Instruction::Sub(rd, rn) => write!(f, "SUB R{}, R{}", rd, rn),
            Instruction::Mul(rd, rn) => write!(f, "MUL R{}, R{}", rd, rn),
            Instruction::Div(rd, rn) => write!(f, "DIV R{}, R{}", rd, rn),
            Instruction::Jmp(addr) => write!(f, "JMP {}", addr),
            Instruction::Jeq(label, rn, rm) => write!(f, "JEQ {}, R{}, R{}", label, rn, rm),
            Instruction::Jne(label, rn, rm) => write!(f, "JNE {}, R{}, R{}", label, rn, rm),
            // ... and so on for all instructions
        }
    }
}

// Define a function to execute an instruction
fn execute_instruction(instruction: Instruction) {
    match instruction {
        Instruction::Load(rd, rn) => {
            // Implement LOAD instruction
        }
        Instruction::Store(rd, rn) => {
            // Implement STORE instruction
        }
        // ... and so on for all instructions
    }
}
