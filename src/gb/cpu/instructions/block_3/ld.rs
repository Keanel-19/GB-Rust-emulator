use crate::gb::cpu::{enums::Instruction, CpuContext};

opcode!{
    ldh_c_a (cpu: &mut CpuContext) {
        cpu.hw.write(0xff00 + cpu.regs.c as u16, cpu.regs.a);
        Instruction::default()
    }
}

opcode!{
    ldh_a_c (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(0xff00 + cpu.regs.c as u16);
        Instruction::Void(load)
    }
    load (cpu: &mut CpuContext) {
        cpu.regs.a = cpu.regs.z;
        cpu.fetch_pc()
    }
}

opcode!{
    ldh_u8_a (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(write_mem)
    }
    write_mem (cpu: &mut CpuContext) {
        cpu.hw.write(0xff00 + cpu.regs.z as u16, cpu.regs.a);
        Instruction::default()
    }
}

opcode!{
    ldh_a_u8 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(read_mem)
    }
    read_mem (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(0xff00 + cpu.regs.z as u16);
        Instruction::Void(load)
    }
    load (cpu: &mut CpuContext) {
        cpu.regs.a = cpu.regs.z;
        cpu.fetch_pc()
    }
}

opcode!{
    ld_u16_a (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(load_w)
    }
    load_w (cpu: &mut CpuContext) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(write_mem)
    }
    write_mem (cpu: &mut CpuContext) {
        cpu.hw.write(cpu.regs.wz(), cpu.regs.a);
        Instruction::default()
    }
}

opcode!{
    ld_a_u16 (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(load_w)
    }
    load_w (cpu: &mut CpuContext) {
        cpu.regs.w = cpu.hw.read(cpu.regs.pc);
        cpu.regs.pc += 1;
        Instruction::Void(read_mem)
    }
    read_mem (cpu: &mut CpuContext) {
        cpu.regs.z = cpu.hw.read(cpu.regs.wz());
        Instruction::Void(load)
    }
    load (cpu: &mut CpuContext) {
        cpu.regs.a = cpu.regs.z;
        cpu.fetch_pc()
    }
}
