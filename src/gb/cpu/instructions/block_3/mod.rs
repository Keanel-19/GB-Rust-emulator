mod math;
mod ctrl_flow;
mod ld;
mod stack;
mod interrupt;

mod special {
    use crate::gb::cpu::{enums::Instruction, instructions::decode::decode_cb, CpuContext};

    opcode!{
        prefix (cpu: &mut CpuContext) {
            let next = decode_cb(cpu.hw.read(cpu.regs.pc));
            cpu.regs.pc += 1;
            next
        }
    }
}

pub use math::*;
pub use ctrl_flow::*;
pub use ld::*;
pub use stack::*;
pub use interrupt::*;
pub use special::*;
