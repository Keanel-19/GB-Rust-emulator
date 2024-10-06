use crate::gb::cpu::{enums::{Instruction, OpCond}, structs::RW, CpuContext};

opcode!{
    ret (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    ret_cond (cpu: &mut CpuContext, cond: OpCond) {
        todo!()
    }
}

opcode!{
    jp_u16 (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    jp_cond_u16 (cpu: &mut CpuContext, cond: OpCond) {
        todo!()
    }
}

opcode!{
    jp_hl (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    call_u16 (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    call_cond_u16 (cpu: &mut CpuContext, cond: OpCond) {
        todo!()
    }
}

opcode!{
    rst_tgt (cpu: &mut CpuContext, tgt: u8) {
        todo!()
    }
}
