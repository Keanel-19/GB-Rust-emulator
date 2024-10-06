use crate::gb::cpu::{enums::{Instruction, Reg8}, structs::RW, CpuContext};

opcode!{
    add_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let nib = cpu.regs.a & 0xf;
        let (result, overflow) = cpu.regs.a.overflowing_add(v);
        cpu.regs.a = result;
        cpu.regs.flag_z = result == 0;
        cpu.regs.flag_n = false; 
        cpu.regs.flag_h = result & 0xf < nib;
        cpu.regs.flag_c = overflow;
        cpu.fetch_pc()
    }
}

opcode!{
    adc_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let nib = cpu.regs.a & 0xf;

        let (res, ov) = cpu.regs.a.overflowing_add(v);

        let nib2 = res & 0xf;
        let nib_ov = nib2 < nib;

        let (res2, ov2) = res.overflowing_add(cpu.regs.flag_c as u8);

        cpu.regs.a = res2;
        cpu.regs.flag_z = res2 == 0;
        cpu.regs.flag_n = false; 
        cpu.regs.flag_h = res2 & 0xf < nib2 || nib_ov;
        cpu.regs.flag_c = ov || ov2;
        cpu.fetch_pc()
    }
}

opcode!{
    sub_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let nib = cpu.regs.a & 0xf;
        let (result, overflow) = cpu.regs.a.overflowing_sub(v);
        cpu.regs.a = result;
        cpu.regs.flag_z = result == 0;
        cpu.regs.flag_n = true; 
        cpu.regs.flag_h = result & 0xf > nib;
        cpu.regs.flag_c = overflow;
        cpu.fetch_pc()
    }
}

opcode!{
    sbc_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let nib = cpu.regs.a & 0xf;

        let (res, ov) = cpu.regs.a.overflowing_sub(v);

        let nib2 = res & 0xf;
        let nib_ov = nib2 > nib;

        let (res2, ov2) = res.overflowing_sub(cpu.regs.flag_c as u8);

        cpu.regs.a = res2;
        cpu.regs.flag_z = res2 == 0;
        cpu.regs.flag_n = true; 
        cpu.regs.flag_h = res2 & 0xf > nib2 || nib_ov;
        cpu.regs.flag_c = ov || ov2;
        cpu.fetch_pc()
    }
}

opcode!{
    and_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let result = cpu.regs.a & v;
        cpu.regs.a = result;
        cpu.regs.flag_z = result == 0;
        cpu.regs.flag_n = false; 
        cpu.regs.flag_h = true;
        cpu.regs.flag_c = false;
        cpu.fetch_pc()
    }
}

opcode!{
    xor_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let result = cpu.regs.a ^ v;
        cpu.regs.a = result;
        cpu.regs.flag_z = result == 0;
        cpu.regs.flag_n = false; 
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = false;
        cpu.fetch_pc()
    }
}

opcode!{
    or_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let result = cpu.regs.a | v;
        cpu.regs.a = result;
        cpu.regs.flag_z = result == 0;
        cpu.regs.flag_n = false; 
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = false;
        cpu.fetch_pc()
    }
}

opcode!{
    cp_a_r8 (cpu: &mut CpuContext , r: Reg8) {
        if r == Reg8::IndirectHL {
            cpu.regs.z = cpu.hw.read(cpu.regs.hl());
            return Instruction::U8(op, cpu.regs.z);
        }
        op(cpu, cpu.read(r))
    }

    op (cpu: &mut CpuContext , v: u8) {
        let nib = cpu.regs.a & 0xf;
        let (result, overflow) = cpu.regs.a.overflowing_sub(v);
        
        cpu.regs.flag_z = result == 0;
        cpu.regs.flag_n = true; 
        cpu.regs.flag_h = result & 0xf > nib;
        cpu.regs.flag_c = overflow;
        cpu.fetch_pc()
    }
}
