use crate::gb::cpu::{enums::{Instruction, Reg16}, structs::RW, CpuContext};

opcode!{
    pop_r16 (cpu: &mut CpuContext, r: Reg16) {
        todo!()
    }
}

opcode!{
    push_r16 (cpu: &mut CpuContext, r: Reg16) {
        todo!()
    }
}

opcode!{
    add_sp_i8 (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    ld_hl_sp_i8 (cpu: &mut CpuContext) {
        todo!()
    }
}

opcode!{
    ld_sp_hl (cpu: &mut CpuContext) {
        todo!()
    }
}
