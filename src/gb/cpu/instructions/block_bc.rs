use crate::gb::cpu::{enums::{Instruction, Reg8}, structs::RW, CpuContext};

opcode!{
    rlc_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r).rotate_left(1);
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = res & 1 > 0;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z.rotate_left(1);
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = res & 1 > 0;
        Instruction::default()
    }
}

opcode!{
    rrc_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r).rotate_right(1);
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = res & 0x80 > 0;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z.rotate_right(1);
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = res & 0x80 > 0;
        Instruction::default()
    }
}

opcode!{
    rl_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r);
        let carry = res & 0x80 > 0;
        let res = res << 1 | cpu.regs.flag_c as u8;
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z;
        let carry = res & 0x80 > 0;
        let res = res << 1 | cpu.regs.flag_c as u8;
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        Instruction::default()
    }
}

opcode!{
    rr_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r);
        let carry = res & 0x1 > 0;
        let res = res >> 1 | (cpu.regs.flag_c as u8) << 8;
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z;
        let carry = res & 0x1 > 0;
        let res = res >> 1 | (cpu.regs.flag_c as u8) << 8;
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        Instruction::default()
    }
}

opcode!{
    sla_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r);
        let carry = res & 0x80 > 0;
        let res = res << 1;
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z;
        let carry = res & 0x80 > 0;
        let res = res << 1;
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        Instruction::default()
    }
}

opcode!{
    sra_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r);
        let carry = res & 0x1 > 0;
        let bit7 = res  & 0x80;
        let res = bit7 | res >> 1;
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z;
        let carry = res & 0x1 > 0;
        let bit7 = res  & 0x80;
        let res = bit7 | res >> 1;
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        Instruction::default()
    }
}

opcode!{
    swap_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r).rotate_left(4);
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = false;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z.rotate_left(4);
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = false;
        Instruction::default()
    }
}

opcode!{
    srl_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu);
        }
        let res = cpu.read(r);
        let carry = res & 0x1 > 0;
        let res = res >> 1;
        cpu.write(r, res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::Void(hl2)
    }

    hl2 (cpu: &mut CpuContext) {
        let res = cpu.regs.z;
        let carry = res & 0x1 > 0;
        let res = res >> 1;
        cpu.hw.write(cpu.regs.hl(), res);
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        Instruction::default()
    }
}

opcode!{
    bit_b3_r8 (cpu: &mut CpuContext, index: u8, r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu, index);
        }
        let res = cpu.read(r) & 1 << index;
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = true;
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext, index: u8) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::U8(hl2, index)
    }

    hl2 (cpu: &mut CpuContext, index: u8) {
        let res = cpu.regs.z & 1 << index;
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = true;
        cpu.fetch_pc()
    }
}

opcode!{
    res_b3_r8 (cpu: &mut CpuContext , index: u8, r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu, index);
        }
        let mask = 0xff ^ 1 << index;
        let res = cpu.read(r) & mask;
        cpu.write(r, res);
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext, index: u8) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::U8(hl2, index)
    }

    hl2 (cpu: &mut CpuContext, index: u8) {
        let mask = 0xff ^ 1 << index;
        let res = cpu.regs.z & mask;
        cpu.hw.write(cpu.regs.hl(), res);
        Instruction::default()
    }
}

opcode!{
    set_b3_r8 (cpu: &mut CpuContext , index: u8, r: Reg8) {
        if r == Reg8::IndirectHL {
            return hl1(cpu, index);
        }
        let mask = 1 << index;
        let res = cpu.read(r) | mask;
        cpu.write(r, res);
        cpu.fetch_pc()
    }

    hl1 (cpu: &mut CpuContext, index: u8) {
        cpu.regs.z = cpu.hw.read(cpu.regs.hl());
        Instruction::U8(hl2, index)
    }

    hl2 (cpu: &mut CpuContext, index: u8) {
        let mask = 1 << index;
        let res = cpu.regs.z | mask;
        cpu.hw.write(cpu.regs.hl(), res);
        Instruction::default()
    }
}
