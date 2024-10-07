use crate::gb::cpu::{enums::{Instruction, Reg16, Reg8}, structs::RW, CpuContext};

opcode!{
    inc_r16 (cpu: &mut CpuContext, r: Reg16) {
        cpu.write(r, cpu.read(r).wrapping_add(1));
        Instruction::default()
    }
}

opcode!{
    dec_r16 (cpu: &mut CpuContext, r: Reg16) {
        cpu.write(r, cpu.read(r).wrapping_sub(1));
        Instruction::default()
    }
}

opcode!{
    inc_r8 (cpu: &mut CpuContext, r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r);
        let nib = res & 0xf;
        let res = res.wrapping_add(1);
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = res & 0xf < nib;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z;
        let nib = res & 0xf;
        let res = res.wrapping_add(1);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = res & 0xf < nib;
        Instruction::default()
    }
}

opcode!{
    dec_r8 (cpu: &mut CpuContext, r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r);
        let nib = res & 0xf;
        let res = res.wrapping_sub(1);
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = true;
        cpu.regs.flag_h = res & 0xf > nib;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z;
        let nib = res & 0xf;
        let res = res.wrapping_sub(1);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = true;
        cpu.regs.flag_h = res & 0xf > nib;
        Instruction::default()
    }
}
