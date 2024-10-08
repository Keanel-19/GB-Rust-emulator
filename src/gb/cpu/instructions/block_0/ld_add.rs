use crate::gb::cpu::{enums::{Instruction, Reg16, Reg16Indirect, Reg8}, structs::RW, CpuContext};

opcode!{
    ld_r16_u16 (cpu: &mut CpuContext, r: Reg16) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Reg16(load_w, r)
    }
    load_w (cpu: &mut CpuContext, r: Reg16) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Reg16(write_wz, r)
    }
    write_wz (cpu: &mut CpuContext, r: Reg16) {
        cpu.write(r, cpu.regs.wz());
        cpu.fetch_pc()
    }
}

opcode!{
    ld_r16_a (cpu: &mut CpuContext, r: Reg16Indirect) {
        cpu.write(r, cpu.regs.a);
        if r == Reg16Indirect::HLIncr {
            cpu.regs.set_hl(cpu.regs.hl().wrapping_add(1));
        }
        if r == Reg16Indirect::HLDecr {
            cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
        }
        Instruction::default()
    }
}

opcode!{
    ld_a_r16 (cpu: &mut CpuContext, r: Reg16Indirect) {
        cpu.regs.z = cpu.read(r);
        if r == Reg16Indirect::HLIncr {
            cpu.regs.set_hl(cpu.regs.hl().wrapping_add(1));
        }
        if r == Reg16Indirect::HLDecr {
            cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1));
        }
        Instruction::default()
    }
    load_a (cpu: &mut CpuContext) {
        cpu.regs.a = cpu.regs.z;
        cpu.fetch_pc()
    }
}

opcode!{
    ld_r8_u8 (cpu: &mut CpuContext, r: Reg8) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Reg8(load_r8, r)
    }
    load_r8 (cpu: &mut CpuContext, r: Reg8) {
        cpu.write(r, cpu.regs.z);
        if r == Reg8::IndirectHL {
            return Instruction::default();
        }
        cpu.fetch_pc()
    }
}

opcode!{
    add_hl_r16 (cpu: &mut CpuContext, r: Reg16) {
        let value = cpu.regs.hl();
        let nib = value & 0x0fff;
        let (res, carry) = value.overflowing_add(cpu.read(r));

        cpu.regs.set_hl(res);
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = res & 0x0fff < nib;
        cpu.regs.flag_c = carry;
        Instruction::default()
    }
}
