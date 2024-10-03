use crate::gb::cpu::{enums::{Instruction, Reg8}, structs::RW, Cpu};


opcode!{
    ld_r8_r8 (cpu: &mut Cpu , r1: Reg8, r2: Reg8) {
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
        let next = cpu.fetch_cycle();
        cpu.regs.pc += 1;
        cpu.write(r1, cpu.read(r2));
        next
    }

    next_r1_hl (cpu: &mut Cpu) {
        let next = cpu.fetch_cycle();
        cpu.regs.pc += 1;
        next
    }

    next_r2_hl (cpu: &mut Cpu, r1: Reg8) {
        let next = cpu.fetch_cycle();
        cpu.regs.pc += 1;
        cpu.write(r1, cpu.regs.z);
        next
    }
}

opcode!{
    halt(cpu: &mut Cpu) {
        !todo!()
    }
}