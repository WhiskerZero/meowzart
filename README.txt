Instruction Set

The Meowzart VM will have a total of 28 instructions, divided into 6 categories:

1 - Load/Store (4 instructions)
2 - Arithmetic (8 instructions)
3 - Control Flow (4 instructions)
4 - Memory Management (4 instructions)
5 - Input/Output (4 instructions)
6 - Miscellaneous (4 instructions)

1 - Load/Store Instructions

LOAD: Load a value from memory into a register.
Opcode: 0x01
Syntax: LOAD Rd, Rn, offset
Description: Load a value from memory address Rn + offset into register Rd.
STORE: Store a value from a register into memory.
Opcode: 0x02
Syntax: STORE Rd, Rn, offset
Description: Store the value in register Rd into memory address Rn + offset.
PUSH: Push a value onto the stack.
Opcode: 0x03
Syntax: PUSH Rn
Description: Push the value in register Rn onto the stack.
POP: Pop a value from the stack into a register.
Opcode: 0x04
Syntax: POP Rd
Description: Pop a value from the stack into register Rd.

2 - Arithmetic Instructions

ADD: Add two values and store the result in a register.
Opcode: 0x10
Syntax: ADD Rd, Rn, Rm
Description: Add the values in registers Rn and Rm and store the result in register Rd.
SUB: Subtract two values and store the result in a register.
Opcode: 0x11
Syntax: SUB Rd, Rn, Rm
Description: Subtract the value in register Rm from the value in register Rn and store the result in register Rd.
MUL: Multiply two values and store the result in a register.
Opcode: 0x12
Syntax: MUL Rd, Rn, Rm
Description: Multiply the values in registers Rn and Rm and store the result in register Rd.
DIV: Divide two values and store the result in a register.
Opcode: 0x13
Syntax: DIV Rd, Rn, Rm
Description: Divide the value in register Rn by the value in register Rm and store the result in register Rd.
MOD: Compute the remainder of dividing two values.
Opcode: 0x14
Syntax: MOD Rd, Rn, Rm
Description: Compute the remainder of dividing the value in register Rn by the value in register Rm and store the result in register Rd.
INC: Increment a value in a register.
Opcode: 0x15
Syntax: INC Rn
Description: Increment the value in register Rn by 1.
DEC: Decrement a value in a register.
Opcode: 0x16
Syntax: DEC Rn
Description: Decrement the value in register Rn by 1.
NEG: Negate a value in a register.
Opcode: 0x17
Syntax: NEG Rn
Description: Negate the value in register Rn.

3 - Snow Control

JMP: Jump to a label.
Opcode: 0x20
Syntax: JMP label
Description: Jump to the label label.
JEQ: Jump if equal.
Opcode: 0x21
Syntax: JEQ label, Rn, Rm
Description: Jump to the label label if the values in registers Rn and Rm are equal.
JNE: Jump if not equal.
Opcode: 0x22
Syntax: JNE label, Rn, Rm
Description: Jump to the label label if the values in registers Rn and Rm are not equal.
RET: Return from a function.
Opcode: 0x23
Syntax: RET
Description: Return from a function.

4 - Memory Management

ALLOC: Allocate memory.
Opcode: 0x30
Syntax: ALLOC size
Description: Allocate size bytes of memory.
FREE: Free memory.
Opcode: 0x31
Syntax: FREE ptr
Description: Free the memory pointed to by ptr.
READ: Read from memory.
Opcode: 0x32
Syntax: READ Rd, ptr
Description: Read a value from memory address ptr into register Rd.
WRITE: Write to memory.
Opcode: 0x33
Syntax: WRITE ptr, value
Description: Write the value value to memory address ptr.

5 - Input/Output

IN: Read input from the user.
Opcode: 0x40
Syntax: IN Rd
Description: Read input from the user into register Rd.
OUT: Write output to the user.
Opcode: 0x41
Syntax: OUT value
Description: Write the value value to the user.
READC: Read a character from the user.
Opcode: 0x42
Syntax: READC Rd
Description: Read a character from the user into register Rd.
WRITEC: Write a character to the user.
Opcode: 0x43
Syntax: WRITEC value
Description: Write the character value to the user.

6 - Miscellaneous 

NOP: No operation.
Opcode: 0xFF
Syntax: NOP
Description: No operation.
HALT: Halt the VM.
Opcode: 0xFE
Syntax: HALT
Description: Halt the VM.
DEBUG: Debug the VM.
Opcode: 0xFD
Syntax: DEBUG
Description: Debug the VM.
INFO: Display information about the VM.
Opcode: 0xFC
Syntax: INFO
Description: Display information about the VM.
