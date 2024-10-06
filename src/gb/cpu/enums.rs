use std::mem::transmute;
use super::{instructions::block_0::nop, structs::CpuContext};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Instruction {
    Void(fn (&mut CpuContext) -> Instruction),
    Reg8(fn (&mut CpuContext, Reg8) -> Instruction, Reg8),
    DoubleReg8(fn (&mut CpuContext, Reg8, Reg8) -> Instruction, Reg8, Reg8),
    Reg16(fn (&mut CpuContext, Reg16) -> Instruction, Reg16),
    Reg16Indirect(fn (&mut CpuContext, Reg16Indirect) -> Instruction, Reg16Indirect),
    Reg16Stack(fn (&mut CpuContext, Reg16Stack) -> Instruction, Reg16Stack),
    U8(fn (&mut CpuContext, u8) -> Instruction, u8),
    U8Reg8(fn (&mut CpuContext, u8, Reg8) -> Instruction, u8, Reg8),
}

impl Instruction {
    pub fn exec(self, cpu: &mut CpuContext) -> Instruction {
        match self {
            Instruction::Void(f) => f(cpu),
            Instruction::Reg8(f, reg8) => f(cpu, reg8),
            Instruction::DoubleReg8(f, reg8, reg9) => f(cpu, reg8, reg9),
            Instruction::Reg16(f, reg16) => f(cpu, reg16),
            Instruction::Reg16Indirect(f, reg16_indirect) => f(cpu, reg16_indirect),
            Instruction::Reg16Stack(f, reg16_stack) => f(cpu, reg16_stack),
            Instruction::U8(f, n) => f(cpu,n),
            Instruction::U8Reg8(f, nn, reg8) => f(cpu, nn, reg8),
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Void(nop)
    }
}

//----------------------------------------------------------

macro_rules! try_from_u8 {
    ($enum:ty, $max:literal) => {
        impl TryFrom<u8> for $enum {
            #[inline]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                if value < $max {
                    Ok(unsafe {transmute(value)})
                } else {
                    Err(concat!(stringify!($enum), " only allow integer strictely less than ", $max))
                }
            }
            
            type Error=&'static str;
        }
    };
}

//----------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Reg8 {
    B,C,D,E,H,L,IndirectHL,A
}

try_from_u8!(Reg8,8);

//----------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Reg16 {
    BC,DE,HL,SP
}

try_from_u8!(Reg16,4);

//----------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Reg16Stack {
    BC,DE,HL,AF
}

try_from_u8!(Reg16Stack,4);

//----------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Reg16Indirect {
    BC,DE,HLIncr,HLDecr
}

try_from_u8!(Reg16Indirect,4);

//----------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum OpCond {
    NZ,Z,NC,C
}

try_from_u8!(OpCond,4);
