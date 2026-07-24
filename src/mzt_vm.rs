mod cat_instructions;

fn main() {
    let instruction = cat_instructions::Instruction::Load(0, 1);
    println!("{}", instruction);
    cat_instructions::execute_instruction(instruction);
}
