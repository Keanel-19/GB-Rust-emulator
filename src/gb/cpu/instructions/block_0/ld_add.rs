use crate::gb::cpu::{enums::{Instruction, Reg16, Reg16Indirect, Reg8}, structs::RW, Cpu};

opcode!{
    ld_r16_u16 (cpu: &mut Cpu, r: Reg16) {
        todo!()
    }
}

opcode!{
    ld_r16_a (cpu: &mut Cpu, r: Reg16Indirect) {
        todo!()
    }
}

opcode!{
    ld_a_r16 (cpu: &mut Cpu, r: Reg16Indirect) {
        todo!()
    }
}

opcode!{
    ld_r8_u8 (cpu: &mut Cpu, r: Reg8) {
        todo!()
    }
}

opcode!{
    add_hl_r16 (cpu: &mut Cpu, r: Reg16) {
        todo!()
    }
}
