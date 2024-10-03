use crate::gb::cpu::{enums::{Instruction, Reg16}, structs::RW, Cpu};

opcode!{
    pop_r16 (cpu: &mut Cpu, r: Reg16) {
        todo!()
    }
}

opcode!{
    push_r16 (cpu: &mut Cpu, r: Reg16) {
        todo!()
    }
}

opcode!{
    add_sp_i8 (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    ld_hl_sp_i8 (cpu: &mut Cpu) {
        todo!()
    }
}

opcode!{
    ld_sp_hl (cpu: &mut Cpu) {
        todo!()
    }
}
