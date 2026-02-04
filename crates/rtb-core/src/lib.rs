// The Data : This represents values in the IR (like %0, %1, or 42)
#[derive(Debug)]
pub enum Operand {
    Integer(i32),
    Register(String),
}

#[derive(Debug)]
pub enum Instruction {
    Add {
        dest: String,
        op1: Operand,
        op2: Operand,
    },
    Ret(Operand),
}

pub fn compile(inst: Instruction) {
    match inst {
        Instruction::Add { dest, op1, op2 } => {
            println!("Context: Mapping ADD Instruction");
            println!(
                "-> TPDE : Create ADD node for {} = {:?} + {:?}",
                dest, op1, op2
            );
        }

        Instruction::Ret(val) => {
            println!("COntext: Mapping RET instruction");
            println!("-> TPDE: Create RETURN node for {:?}", val);
        }
    }
}
