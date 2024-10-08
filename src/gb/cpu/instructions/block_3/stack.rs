use crate::gb::cpu::{enums::{Instruction, Reg16}, structs::RW, CpuContext};

opcode!{
    pop_r16 (cpu: &mut CpuContext, r: Reg16) {
        cpu.regs.z = cpu.hw.read(cpu.regs.sp);
        cpu.regs.sp += 1;
        Instruction::Reg16(load_w, r)
    }
    load_w (cpu: &mut CpuContext, r: Reg16) {
        cpu.regs.w = cpu.hw.read(cpu.regs.sp);
        cpu.regs.sp += 1;
        Instruction::Reg16(write_r16, r)
    }
    write_r16 (cpu: &mut CpuContext, r: Reg16) {
        cpu.write(r,cpu.regs.wz());
        cpu.fetch_pc()
    }
}

opcode!{
    push_r16 (cpu: &mut CpuContext, r: Reg16) {
        cpu.regs.sp -= 1;
        Instruction::Reg16(write_msb, r)
    }
    write_msb (cpu: &mut CpuContext, r: Reg16) {
        cpu.hw.write(cpu.regs.sp, (cpu.read(r) >> 8)as u8);
        cpu.regs.sp -= 1;
        Instruction::Reg16(write_lsb, r)
    }
    write_lsb (cpu: &mut CpuContext, r: Reg16) {
        cpu.hw.write(cpu.regs.sp, cpu.read(r) as u8);
        Instruction::default()
    }
}

opcode!{
    add_sp_i8 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(flags)
    }
    flags (cpu: &mut CpuContext) {
        let value = cpu.regs.z;
        let nib = value & 0xf;
        let (res,carry) = cpu.regs.pc_low().overflowing_add(value);
        cpu.regs.flag_z = false;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = res & 0xf < nib;
        cpu.regs.flag_c = carry;
        Instruction::Void(eval)
    }
    eval (cpu: &mut CpuContext) {
        cpu.regs.sp = cpu.regs.sp.wrapping_add_signed(cpu.regs.z as i8 as i16);
        Instruction::default()
    }
}

opcode!{
    ld_hl_sp_i8 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(flags)
    }
    flags (cpu: &mut CpuContext) {
        let value = cpu.regs.z;
        let nib = value & 0xf;
        let (res,carry) = cpu.regs.pc_low().overflowing_add(value);
        cpu.regs.flag_z = false;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = res & 0xf < nib;
        cpu.regs.flag_c = carry;
        Instruction::Void(eval)
    }
    eval (cpu: &mut CpuContext) {
        cpu.regs.set_hl(cpu.regs.sp.wrapping_add_signed(cpu.regs.z as i8 as i16));
        cpu.fetch_pc()
    }
}

opcode!{
    ld_sp_hl (cpu: &mut CpuContext) {
        cpu.regs.sp = cpu.regs.hl();
        Instruction::default()
    }
}
