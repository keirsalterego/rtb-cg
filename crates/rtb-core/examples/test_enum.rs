use rtb_core::{Instruction, Operand, compile};

fn main() {
    let ir = vec![
        Instruction::Add {
            dest: "%temp_1".to_string(),
            op1: Operand::Register("%arg_a".to_string()),
            op2: Operand::Register("%arg_b".to_string()),
        },
        Instruction::Ret(Operand::Register("%temp_1".to_string())),
    ];

    println!("--- Starting Compilation Mock ---");
    for inst in ir {
        compile(inst);
    }
    println!("--- Finished Compilation Mock ---");
}
