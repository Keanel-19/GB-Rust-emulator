use crate::gb::cpu::{enums::{Instruction, OpCond}, CpuContext};

opcode!{
    nop (cpu: &mut CpuContext) {
        cpu.fetch_pc()
    }
}

opcode!{
    stop (cpu: &mut CpuContext) {
        todo!();
        cpu.fetch_pc()
    }
}

opcode!{
    ld_u16_sp (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(load_w)
    }

    load_w (cpu: &mut CpuContext) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(write_low_sp)
    }
    write_low_sp (cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.wz(), cpu.regs.sp_low());
        cpu.regs.set_wz(cpu.regs.wz().wrapping_add(1));
        Instruction::Void(write_high_sp)
    }
    write_high_sp (cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.wz(), cpu.regs.sp_high());
        Instruction::default()
    }
}

opcode!{
    jr_i8 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(add_relative)
    }
    add_relative (cpu: &mut CpuContext) {
        cpu.regs.set_wz(cpu.regs.pc.wrapping_add_signed(cpu.regs.z as i16));
        Instruction::Void(jump)
    }
    jump (cpu: &mut CpuContext) {
        let next = cpu.fetch_cycle(cpu.regs.wz());
        cpu.regs.pc = cpu.regs.wz() + 1;
        next
    }
}

opcode!{
    jr_cond_i8 (cpu: &mut CpuContext, cond: OpCond) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        if cond.check(cpu.regs) {
            Instruction::Void(add_relative)
        } else {
            Instruction::default()
        }
    }
    add_relative (cpu: &mut CpuContext) {
        cpu.regs.set_wz(cpu.regs.pc.wrapping_add_signed(cpu.regs.z as i16));
        Instruction::Void(jump)
    }
    jump (cpu: &mut CpuContext) {
        let next = cpu.fetch_cycle(cpu.regs.wz());
        cpu.regs.pc = cpu.regs.wz() + 1;
        next
    }
}
