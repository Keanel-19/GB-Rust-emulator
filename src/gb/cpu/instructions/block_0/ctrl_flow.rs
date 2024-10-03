use crate::gb::cpu::{enums::{Instruction, OpCond}, structs::RW, Cpu};

opcode!{
    nop (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    stop (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    ld_u16_sp (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    jr_i8 (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    jr_cond_i8 (cpu: &mut Cpu, cond: OpCond) {
        todo!()
    }
}
