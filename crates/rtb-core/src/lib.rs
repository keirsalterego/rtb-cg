#[derive(Debug, Clone)]
pub enum Operand {
    Stack(String),
    Register(String), 
    Immediate(i32),
    Block(String),
}

#[derive(Debug)]
pub enum Instruction {
    Add { dest: String, op1: Operand, op2: Operand },
    Sub { dest: String, op1: Operand, op2: Operand },
    Phi { 
        dest: String, 
        incoming: Vec<(String, Operand)> 
    },
    Select {
        dest: String,
        cond: Operand,
        if_true: Operand,
        if_false: Operand,
    },
    Ret(Operand),
}

pub fn compile(inst: Instruction) {
    match inst {
        Instruction::Add { dest, op1, op2 } => {
            println!("[rtb-core] Mapping ADD: {} = {:?} + {:?}", dest, op1, op2);
        }
        Instruction::Phi { dest, incoming } => {
            println!("[rtb-core] Mapping PHI node for target: {}", dest);
            for (block, val) in incoming {
                println!("  <- Source: {} | Value: {:?}", block, val);
            }
        }
        Instruction::Ret(val) => {
            println!("[rtb-core] Mapping RET: {:?}", val);
        }
        _ => println!("[rtb-core] Mapping other instruction..."),
    }
}