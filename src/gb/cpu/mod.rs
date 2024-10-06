mod structs;
mod enums;
mod instructions;

use enums::Instruction;
use structs::{CpuContext, Registres};

use super::Hardware;

pub struct Cpu {
    registres: Registres,
    instruction: Instruction
}

impl Default for Cpu {
    fn default() -> Self {
        Self { registres: Default::default(), instruction: Default::default() }
    }
}

impl Cpu {
    pub fn simulate(&mut self, hw: &mut Hardware) {
        let mut cpu = CpuContext { regs: &mut self.registres, hw };
        self.instruction = self.instruction.exec(&mut cpu);
    }
}