use crate::gb::memory::Memory;

use super::enums::*;

pub struct Cpu {
    pub(super) regs: Registres,
    pub(super) instruct: Instruction,
    pub mem: Memory,
}

impl Cpu {
    pub(super) fn fetch_cycle(&self) -> Instruction {
        todo!()
    }
}

impl RW<Reg8> for Cpu {
    type Data = u8;

    fn read(&self, arg: Reg8) -> Self::Data {
        match arg {
            Reg8::A => self.regs.a,
            Reg8::B => self.regs.b,
            Reg8::C => self.regs.c,
            Reg8::D => self.regs.d,
            Reg8::E => self.regs.e,
            Reg8::H => self.regs.h,
            Reg8::L => self.regs.l,
            Reg8::IndirectHL => self.mem.read(self.regs.hl()),
        }
    }

    fn write(&mut self, arg: Reg8, value: Self::Data) {
        match arg {
            Reg8::A => self.regs.a = value,
            Reg8::B => self.regs.b = value,
            Reg8::C => self.regs.c = value,
            Reg8::D => self.regs.d = value,
            Reg8::E => self.regs.e = value,
            Reg8::H => self.regs.h = value,
            Reg8::L => self.regs.l = value,
            Reg8::IndirectHL => self.mem.write(self.regs.hl(), value),
        }
    }
}

impl RW<Reg16> for Cpu {
    type Data = u16;

    fn read(&self, arg: Reg16) -> Self::Data {
        match arg {
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        }
    }

    fn write(&mut self, arg: Reg16, value: Self::Data) {
        match arg {
            Reg16::BC => self.regs.set_bc(value),
            Reg16::DE => self.regs.set_de(value),
            Reg16::HL => self.regs.set_hl(value),
            Reg16::SP => self.regs.sp = value,
        }
    }
}

impl RW<Reg16Indirect> for Cpu {
    type Data = u8;

    fn read(&self, arg: Reg16Indirect) -> Self::Data {
        match arg {
            Reg16Indirect::BC => self.mem.read(self.regs.bc()),
            Reg16Indirect::DE => self.mem.read(self.regs.de()),
            _                 => self.mem.read(self.regs.hl()),
        }
    }

    fn write(&mut self, arg: Reg16Indirect, value: Self::Data) {
        match arg {
            Reg16Indirect::BC => self.mem.write(self.regs.bc(), value),
            Reg16Indirect::DE => self.mem.write(self.regs.de(), value),
            _                 => self.mem.write(self.regs.hl(), value),
        }
    }
}

impl RW<Reg16Stack> for Cpu {
    type Data = u16;

    fn read(&self, arg: Reg16Stack) -> Self::Data {
        match arg {
            Reg16Stack::BC => self.regs.bc(),
            Reg16Stack::DE => self.regs.de(),
            Reg16Stack::HL => self.regs.hl(),
            Reg16Stack::AF => self.regs.af(),
        }
    }

    fn write(&mut self, arg: Reg16Stack, value: Self::Data) {
        match arg {
            Reg16Stack::BC => self.regs.set_bc(value),
            Reg16Stack::DE => self.regs.set_de(value),
            Reg16Stack::HL => self.regs.set_hl(value),
            Reg16Stack::AF => self.regs.set_af(value),
        }
    }
}

pub(super) trait RW<T> {
    type Data;

    fn read(&self, arg: T) -> Self::Data;
    fn write(&mut self, arg: T, value: Self::Data);
}

pub(super) struct Registres {
    pub(super) a: u8,
    pub(super) b: u8,
    pub(super) c: u8,
    pub(super) d: u8,
    pub(super) e: u8,
    pub(super) h: u8,
    pub(super) l: u8,
    pub(super) w: u8,
    pub(super) z: u8,

    pub(super) flag_z: bool,
    pub(super) flag_n: bool,
    pub(super) flag_h: bool,
    pub(super) flag_c: bool,

    pub(super) pc: u16,
    pub(super) sp: u16,

    pub(super) interrupt_enable: bool,
}

impl Registres {
    const fn f(&self) -> u8 {
          (self.flag_z as u8) << 7
        + (self.flag_n as u8) << 6
        + (self.flag_h as u8) << 5
        + (self.flag_c as u8) << 4
    }

    fn set_f(&mut self, v:u8) {
        self.flag_z = 0b1000_0000 & v != 0;
        self.flag_n = 0b0100_0000 & v != 0;
        self.flag_h = 0b0010_0000 & v != 0;
        self.flag_c = 0b0001_0000 & v != 0;
    }

    const fn af(&self) -> u16 {
        (self.a as u16) << 8 + self.f() as u16
    }

    fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as _ ;
        self.set_f(v as u8);
    }

    const fn bc(&self) -> u16 {
        (self.b as u16) << 8 + self.c as u16
    }

    fn set_bc(&mut self, v: u16) {
        self.b = (v >> 8) as _ ;
        self.c = v as u8;
    }


    const fn de(&self) -> u16 {
        (self.d as u16) << 8 + self.e as u16
    }

    fn set_de(&mut self, v: u16) {
        self.d = (v >> 8) as _ ;
        self.e = v as u8;
    }


    const fn hl(&self) -> u16 {
        (self.h as u16) << 8 + self.l as u16
    }

    fn set_hl(&mut self, v: u16) {
        self.h = (v >> 8) as _ ;
        self.l = v as u8;
    }


    const fn wz(&self) -> u16 {
        (self.w as u16) << 8 + self.z as u16
    }

    fn set_wz(&mut self, v: u16) {
        self.w = (v >> 8) as _ ;
        self.z = v as u8;
    }

    const fn pc_high(&self) -> u8 {
        (self.pc >> 8) as _
    }

    const fn pc_low(&self) -> u8 {
        self.pc as _
    }
    
    const fn sp_high(&self) -> u8 {
        (self.sp >> 8) as _
    }

    const fn sp_low(&self) -> u8 {
        self.sp as _
    }
}
