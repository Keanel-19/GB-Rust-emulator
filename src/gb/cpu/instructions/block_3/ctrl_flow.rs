use crate::gb::cpu::{enums::{Instruction, OpCond}, structs::RW, Cpu};

opcode!{
    ret (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    ret_cond (cpu: &mut Cpu, cond: OpCond) {
        todo!()
    }
}

opcode!{
    jp_u16 (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    jp_cond_u16 (cpu: &mut Cpu, cond: OpCond) {
        todo!()
    }
}

opcode!{
    jp_hl (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    call_u16 (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    call_cond_u16 (cpu: &mut Cpu, cond: OpCond) {
        todo!()
    }
}

opcode!{
    rst_tgt (cpu: &mut Cpu, tgt: u8) {
        todo!()
    }
}
