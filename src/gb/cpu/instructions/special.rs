use crate::gb::cpu::{enums::Instruction, CpuContext};

opcode!{
    invalid(cpu: &mut CpuContext) {
        Instruction::Void(invalid)
    }
}

opcode!{
    dispatch_interrupt(cpu: &mut CpuContext) {
        cpu.regs.pc -= 1;
        Instruction::Void(decr_sp)
    }
    decr_sp(cpu: &mut CpuContext) {
        cpu.regs.sp -= 1;
        Instruction::Void(store_msb_pc)
    }
    store_msb_pc(cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_high());
        cpu.regs.sp -= 1;
        Instruction::Void(store_lsb_pc)
    }
    store_lsb_pc(cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.sp, cpu.regs.pc_low());
        cpu.regs.pc = todo!("Need to implement get addr & clear flag IRQ (fn ack ?)");
        Instruction::default()
    }
}
