use crate::gb::cpu::{enums::Instruction, CpuContext};

opcode!{
    reti (cpu: &mut CpuContext) {
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
        cpu.regs.interrupt_enable = true;
        Instruction::default()
    }
}

opcode!{
    di (cpu: &mut CpuContext) {
        cpu.regs.interrupt_enable = false;
        cpu.fetch_pc()
    }
}

opcode!{
    ei (cpu: &mut CpuContext) {
        let next = cpu.fetch_cycle(cpu.regs.pc);
        cpu.regs.pc += 1;
        cpu.regs.interrupt_enable = true;
        next
    }
}
