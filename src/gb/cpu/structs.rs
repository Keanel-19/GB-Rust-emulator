use super::enums::*;

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    w: u8,
    z: u8,

    flag_z: bool,
    flag_n: bool,
    flag_h: bool,
    flag_c: bool,

    pc: u16,
    sp: u16,

    instruction: Instruction,
    interrupt_enable: bool,
}

impl Cpu {
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
