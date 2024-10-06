use crate::gb::cpu::{enums::{Instruction, OpCond}, structs::RW, CpuContext};

opcode!{
    nop (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    stop (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    ld_u16_sp (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    jr_i8 (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    jr_cond_i8 (cpu: &mut CpuContext, cond: OpCond) {
        todo!()
    }
}
