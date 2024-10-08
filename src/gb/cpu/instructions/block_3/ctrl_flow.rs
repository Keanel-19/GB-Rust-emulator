use crate::gb::cpu::{enums::{Instruction, OpCond}, CpuContext};

opcode!{
    ret (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.sp);
        cpu.regs.sp += 1;
        Instruction::Void(load_w)
    }
    load_w (cpu: &mut CpuContext) {
        cpu.regs.w = cpu.hw.read(cpu.regs.sp);
        cpu.regs.sp += 1;
        Instruction::Void(write_pc)
    }
    write_pc (cpu: &mut CpuContext) {
        cpu.regs.pc = cpu.regs.wz();
        Instruction::default()
    }
}

opcode!{
    ret_cond (cpu: &mut CpuContext, cond: OpCond) {
        if cond.check(cpu.regs) {
            Instruction::Void(load_z)
        } else {
            Instruction::default()
        }
    }
    load_z (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.sp);
        cpu.regs.sp += 1;
        Instruction::Void(load_w)
    }
    load_w (cpu: &mut CpuContext) {
        cpu.regs.w = cpu.hw.read(cpu.regs.sp);
        cpu.regs.sp += 1;
        Instruction::Void(write_pc)
    }
    write_pc (cpu: &mut CpuContext) {
        cpu.regs.pc = cpu.regs.wz();
        Instruction::default()
    }
}

opcode!{
    jp_u16 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(load_w)
    }
    load_w (cpu: &mut CpuContext) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(write_pc)
    }
    write_pc (cpu: &mut CpuContext) {
        cpu.regs.pc = cpu.regs.wz();
        Instruction::default()
    }
}

opcode!{
    jp_cond_u16 (cpu: &mut CpuContext, cond: OpCond) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::OpCond(load_w, cond)
    }
    load_w (cpu: &mut CpuContext, cond: OpCond) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        if cond.check(cpu.regs) {
            Instruction::Void(write_pc)
        } else {
            Instruction::default()
        }
        
    }
    write_pc (cpu: &mut CpuContext) {
        cpu.regs.pc = cpu.regs.wz();
        Instruction::default()
    }
}

opcode!{
    jp_hl (cpu: &mut CpuContext) {
        let next = cpu.fetch_cycle(cpu.regs.hl());
        cpu.regs.pc = cpu.regs.hl() + 1;
        next
    }
}

opcode!{
    call_u16 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(load_w)
    }
    load_w (cpu: &mut CpuContext) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(decr_sp)
    }
    decr_sp (cpu: &mut CpuContext) {
        cpu.regs.sp -= 1;
        Instruction::Void(store_msb_pc)
    }
    store_msb_pc (cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_high());
        cpu.regs.sp -= 1;
        Instruction::Void(store_lsb_pc)
    }
    store_lsb_pc (cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_low());
        cpu.regs.pc = cpu.regs.wz();
        Instruction::default()
    }
}

opcode!{
    call_cond_u16 (cpu: &mut CpuContext, cond: OpCond) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::OpCond(load_w,cond)
    }
    load_w (cpu: &mut CpuContext, cond: OpCond) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        if cond.check(cpu.regs) {
            Instruction::Void(decr_sp)
        } else {
            Instruction::default()
        }
    }
    decr_sp (cpu: &mut CpuContext) {
        cpu.regs.sp -= 1;
        Instruction::Void(store_msb_pc)
    }
    store_msb_pc (cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_high());
        cpu.regs.sp -= 1;
        Instruction::Void(store_lsb_pc)
    }
    store_lsb_pc (cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_low());
        cpu.regs.pc = cpu.regs.wz();
        Instruction::default()
    }
}

opcode!{
    rst_tgt (cpu: &mut CpuContext, tgt: u8) {
        cpu.regs.sp -= 1;
        Instruction::U8(store_msb_pc, tgt)
    }
    store_msb_pc (cpu: &mut CpuContext, tgt: u8) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_high());
        cpu.regs.sp -= 1;
        Instruction::U8(store_lsb_pc, tgt)
    }
    store_lsb_pc (cpu: &mut CpuContext, tgt: u8) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_low());
        cpu.regs.pc = tgt as u16;
        Instruction::default()
    }
}
