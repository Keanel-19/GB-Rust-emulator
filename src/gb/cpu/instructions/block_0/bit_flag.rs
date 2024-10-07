use crate::gb::cpu::{enums::Instruction, CpuContext};

opcode!{
    rlca (cpu: &mut CpuContext) {
        let res = cpu.regs.a.rotate_left(1);
        cpu.regs.a = res;
        cpu.regs.flag_z = false;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = res & 1 > 0;
        cpu.fetch_pc()
    }
}

opcode!{
    rrca (cpu: &mut CpuContext) {
        let res = cpu.regs.a.rotate_right(1);
        cpu.regs.a = res;
        cpu.regs.flag_z = false;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = res & 0x80 > 0;
        cpu.fetch_pc()
    }
}

opcode!{
    rla (cpu: &mut CpuContext) {
        let res = cpu.regs.a;
        let carry = res & 0x80 > 0;
        let res = res << 1 | cpu.regs.flag_c as u8;
        cpu.regs.a = res;
        cpu.regs.flag_z = false;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        cpu.fetch_pc()
    }
}

opcode!{
    rra (cpu: &mut CpuContext) {
        let res = cpu.regs.a;
        let carry = res & 0x1 > 0;
        let res = res >> 1 | (cpu.regs.flag_c as u8) << 8;
        cpu.regs.a = res;
        cpu.regs.flag_z = false;
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = carry;
        cpu.fetch_pc()
    }
}

opcode!{
    daa (cpu: &mut CpuContext) {
        let value = cpu.regs.a;
        let mut offset = 0;
        let neg = cpu.regs.flag_n;

        if (!neg && value & 0xf > 0x09) || cpu.regs.flag_h {
            offset += 0x06;
        }
        if (!neg && value > 0x99) || cpu.regs.flag_c {
            offset += 0x60;
        } 
        
        let (res, new_carry) = if neg {
            value.overflowing_sub(offset)
        } else {
            value.overflowing_add(offset)
        };

        cpu.regs.a = res;
        cpu.regs.flag_z = res == 0;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c |= new_carry ;
        cpu.fetch_pc()
    }
}

opcode!{
    cpl (cpu: &mut CpuContext) {
        let res = !cpu.regs.a;
        cpu.regs.a = res;
        cpu.regs.flag_n = true;
        cpu.regs.flag_h = true;
        cpu.fetch_pc()
    }
}

opcode!{
    scf (cpu: &mut CpuContext) {
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = true;
        cpu.fetch_pc()
    }
}

opcode!{
    ccf (cpu: &mut CpuContext) {
        cpu.regs.flag_n = false;
        cpu.regs.flag_h = false;
        cpu.regs.flag_c = !cpu.regs.flag_c;
        cpu.fetch_pc()
    }
}
