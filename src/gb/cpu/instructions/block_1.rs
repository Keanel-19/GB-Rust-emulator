use crate::gb::cpu::{enums::{Instruction, Reg8}, structs::RW, CpuContext};


opcode!{
    ld_r8_r8 (cpu: &mut CpuContext , r1: Reg8, r2: Reg8) {
        if r1 == Reg8::IndirectHL {
            assert_ne!(r1,r2);
            cpu.write(r1, cpu.read(r2));
            return Instruction::Void(next_r1_hl);
        }
        if r2 == Reg8::IndirectHL {
            assert_ne!(r1,r2);
            cpu.regs.z = cpu.read(r2);
            return Instruction::Reg8(next_r2_hl, r1);
        }
        cpu.write(r1, cpu.read(r2));
        cpu.fetch_pc()
    }

    next_r1_hl (cpu: &mut CpuContext) {
        cpu.fetch_pc()
    }

    next_r2_hl (cpu: &mut CpuContext, r1: Reg8) {
        cpu.write(r1, cpu.regs.z);
        cpu.fetch_pc()
    }
}

opcode!{
    halt(cpu: &mut CpuContext) {
        !todo!()
    }
}