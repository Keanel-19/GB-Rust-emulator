mod math;
mod ctrl_flow;
mod ld;
mod stack;
mod interrupt;

mod special {
    use crate::gb::cpu::{enums::{Instruction, Reg8}, structs::RW, Cpu};

    opcode!{
        prefix (cpu: &mut Cpu) {
            todo!()
        }
    }
}
