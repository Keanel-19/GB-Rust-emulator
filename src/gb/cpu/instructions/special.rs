use crate::gb::cpu::{enums::Instruction, CpuContext};

opcode!{
    invalid(cpu: &mut CpuContext) {
        Instruction::Void(invalid)
    }
}

opcode!{
    dispatch_interrupt(cpu: &mut CpuContext, addr: u8) {
        todo!()
    }
}
