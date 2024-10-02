use std::{convert::Infallible, error::Error, fmt, mem::transmute};
use super::Cpu;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Instruction {
    Void(fn (&mut Cpu) -> Instruction),
    Reg8(fn (&mut Cpu, Reg8) -> Instruction, Reg8),
    DoubleReg8(fn (&mut Cpu, Reg8, Reg8) -> Instruction, Reg8, Reg8),
    Reg16(fn (&mut Cpu, Reg16) -> Instruction, Reg16),
    Reg16Indirect(fn (&mut Cpu, Reg16Indirect) -> Instruction, Reg16Indirect),
    Reg16Stack(fn (&mut Cpu, Reg16Stack) -> Instruction, Reg16Stack),
    U8(fn (&mut Cpu, u8) -> Instruction, u8),
    U8Reg8(fn (&mut Cpu, u8, Reg8) -> Instruction, u8, Reg8),
}

//-------------------------------------------------------------

/// The error type returned when a checked integral type conversion fails.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromIntError;

impl fmt::Display for TryFromIntError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        "out of range integral type conversion attempted".fmt(fmt)
    }
}

impl Error for TryFromIntError {}

impl From<Infallible> for TryFromIntError {
    fn from(x: Infallible) -> TryFromIntError {
        match x {}
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
                    Err(TryFromIntError)
                }
            }
            
            type Error=TryFromIntError;
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
    BC,DE,HL,SP
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
