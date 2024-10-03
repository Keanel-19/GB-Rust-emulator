use crate::gb::cpu::{enums::Instruction, Cpu};

opcode!{
    invalid (cpu: &mut Cpu) {
        Instruction::Void(invalid)
    }
}