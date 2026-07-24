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
            // Load / Store
            Instruction::Load(rd, rn, offset) => {
                write!(f, "LOAD R{}, R{}, {}", rd, rn, offset)
            }
            Instruction::Store(rd, rn, offset) => {
                write!(f, "STORE R{}, R{}, {}", rd, rn, offset)
            }
            Instruction::Push(rn) => write!(f, "PUSH R{}", rn),
            Instruction::Pop(rd) => write!(f, "POP R{}", rd),

            // Arithmetic
            Instruction::Add(rd, rn, rm) => {
                write!(f, "ADD R{}, R{}, R{}", rd, rn, rm)
            }
            Instruction::Sub(rd, rn, rm) => {
                write!(f, "SUB R{}, R{}, R{}", rd, rn, rm)
            }
            Instruction::Mul(rd, rn, rm) => {
                write!(f, "MUL R{}, R{}, R{}", rd, rn, rm)
            }
            Instruction::Div(rd, rn, rm) => {
                write!(f, "DIV R{}, R{}, R{}", rd, rn, rm)
            }
            Instruction::Mod(rd, rn, rm) => {
                write!(f, "MOD R{}, R{}, R{}", rd, rn, rm)
            }

            Instruction::Inc(rn) => write!(f, "INC R{}", rn),
            Instruction::Dec(rn) => write!(f, "DEC R{}", rn),
            Instruction::Neg(rn) => write!(f, "NEG R{}", rn),

            // Control Flow
            Instruction::Jmp(addr) => write!(f, "JMP {}", addr),
            Instruction::Jeq(label, rn, rm) => {
                write!(f, "JEQ {}, R{}, R{}", label, rn, rm)
            }
            Instruction::Jne(label, rn, rm) => {
                write!(f, "JNE {}, R{}, R{}", label, rn, rm)
            }
            Instruction::Ret => write!(f, "RET"),

            // Memory
            Instruction::Alloc(size) => write!(f, "ALLOC {}", size),
            Instruction::Free(ptr) => write!(f, "FREE R{}", ptr),
            Instruction::Read(rd, ptr) => {
                write!(f, "READ R{}, R{}", rd, ptr)
            }
            Instruction::Write(ptr, value) => {
                write!(f, "WRITE R{}, {}", ptr, value)
            }

            // Input / Output
            Instruction::In(rd) => write!(f, "IN R{}", rd),
            Instruction::Out(value) => write!(f, "OUT {}", value),
            Instruction::Readc(rd) => write!(f, "READC R{}", rd),
            Instruction::Writec(value) => write!(f, "WRITEC {}", value),

            // Misc
            Instruction::Info => write!(f, "INFO"),
            Instruction::Debug => write!(f, "DEBUG"),
            Instruction::Halt => write!(f, "HALT"),
            Instruction::Nop => write!(f, "NOP"),
        }
    }
}

// Define a function to execute an instruction
fn execute_instruction(instruction: Instruction) {
    match instruction {
        Instruction::Load(rd, rn, offset) => {
        write!(f, "LOAD R{}, R{}, {}", rd, rn, offset)
        }
        // ... and so on for all instructions
    }
}
