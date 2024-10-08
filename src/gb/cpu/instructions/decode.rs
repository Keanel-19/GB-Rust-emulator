use crate::gb::cpu::enums::{Instruction, OpCond, Reg16, Reg16Indirect, Reg16Stack, Reg8};

use super::{block_0, block_1, block_2, block_3, block_bc, special};

pub fn decode(opcode: u8) -> Instruction {
    let block  = (opcode & 0b1100_0000) >> 6;
    let flag_l = (opcode & 0b0011_1000) >> 3;
    let flag_c =  opcode & 0b0000_0111;
    match block {
        0 => match flag_c {
            0 => match flag_l {
                0 => Instruction::Void(block_0::nop),
                1 => Instruction::Void(block_0::ld_u16_sp),
                2 => Instruction::Void(block_0::stop),
                3 => Instruction::Void(block_0::jr_i8),
                _ => Instruction::OpCond(block_0::jr_cond_i8,OpCond::try_from(flag_l & 0b11).unwrap()),
            },
            1 => match flag_l & 1 {
                0 => Instruction::Reg16(block_0::ld_r16_u16, Reg16::try_from(flag_l >> 1).unwrap()),
                1 => Instruction::Reg16(block_0::add_hl_r16, Reg16::try_from(flag_l >> 1).unwrap()),
                _ => unreachable!()
            },
            2 => match flag_l & 1 {
                0 => Instruction::Reg16Indirect(block_0::ld_r16_a, Reg16Indirect::try_from(flag_l >> 1).unwrap()),
                1 => Instruction::Reg16Indirect(block_0::ld_a_r16, Reg16Indirect::try_from(flag_l >> 1).unwrap()),
                _ => unreachable!()
            },
            3 => match flag_l & 1 {
                0 => Instruction::Reg16(block_0::inc_r16, Reg16::try_from(flag_l >> 1).unwrap()),
                1 => Instruction::Reg16(block_0::dec_r16, Reg16::try_from(flag_l >> 1).unwrap()),
                _ => unreachable!()
            },
            4 => Instruction::Reg8(block_0::inc_r8, Reg8::try_from(flag_l).unwrap()),
            5 => Instruction::Reg8(block_0::dec_r8, Reg8::try_from(flag_l).unwrap()),
            6 => Instruction::Reg8(block_0::ld_r8_u8, Reg8::try_from(flag_l).unwrap()),
            7 => match flag_l {
                0 => Instruction::Void(block_0::rlca),
                1 => Instruction::Void(block_0::rrca),
                2 => Instruction::Void(block_0::rla),
                3 => Instruction::Void(block_0::rra),
                4 => Instruction::Void(block_0::daa),
                5 => Instruction::Void(block_0::cpl),
                6 => Instruction::Void(block_0::scf),
                7 => Instruction::Void(block_0::ccf),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        1 => if opcode == 0o166 {
                Instruction::Void(block_1::halt)
            } else {
                Instruction::DoubleReg8(block_1::ld_r8_r8, Reg8::try_from(flag_l).unwrap(), Reg8::try_from(flag_c).unwrap())
            },
        2 => match flag_l {
            0 => Instruction::Reg8(block_2::add_a_r8, Reg8::try_from(flag_c).unwrap()),
            1 => Instruction::Reg8(block_2::adc_a_r8, Reg8::try_from(flag_c).unwrap()),
            2 => Instruction::Reg8(block_2::sub_a_r8, Reg8::try_from(flag_c).unwrap()),
            3 => Instruction::Reg8(block_2::sbc_a_r8, Reg8::try_from(flag_c).unwrap()),
            4 => Instruction::Reg8(block_2::and_a_r8, Reg8::try_from(flag_c).unwrap()),
            5 => Instruction::Reg8(block_2::xor_a_r8, Reg8::try_from(flag_c).unwrap()),
            6 => Instruction::Reg8(block_2::or_a_r8 , Reg8::try_from(flag_c).unwrap()),
            7 => Instruction::Reg8(block_2::cp_a_r8 , Reg8::try_from(flag_c).unwrap()),
            _ => unreachable!()
        },
        3 => match flag_c {
            0 => if flag_l < 4 {
                Instruction::OpCond(block_3::ret_cond, OpCond::try_from(flag_l & 0b11).unwrap())
            } else {
                match flag_l & 0b11 {
                    0 => Instruction::Void(block_3::ldh_u8_a),
                    1 => Instruction::Void(block_3::add_sp_i8),
                    2 => Instruction::Void(block_3::ldh_a_u8),
                    3 => Instruction::Void(block_3::ld_hl_sp_i8),
                    _ => unreachable!()
                }
            },
            1 => match flag_l & 1 {
                0 => Instruction::Reg16Stack(block_3::pop_r16, Reg16Stack::try_from(flag_l >> 1).unwrap()),
                1 => match flag_l >> 1 {
                    0 => Instruction::Void(block_3::ret),
                    1 => Instruction::Void(block_3::reti),
                    2 => Instruction::Void(block_3::jp_hl),
                    3 => Instruction::Void(block_3::ld_sp_hl),
                    _ => unreachable!()
                },
                _ => unreachable!()
            },
            2 => if flag_l < 4 {
                Instruction::OpCond(block_3::jp_cond_u16, OpCond::try_from(flag_l & 0b11).unwrap())
            } else {
                match flag_l & 0b11 {
                    0 => Instruction::Void(block_3::ldh_c_a),
                    1 => Instruction::Void(block_3::ld_u16_a),
                    2 => Instruction::Void(block_3::ldh_a_c),
                    3 => Instruction::Void(block_3::ld_a_u16),
                    _ => unreachable!()
                }
            },
            3 => match flag_l {
                0 => Instruction::Void(block_3::jp_u16),
                1 => Instruction::Void(block_3::prefix),
                6 => Instruction::Void(block_3::di),
                7 => Instruction::Void(block_3::ei),
                _ => Instruction::Void(special::invalid),
            },
            4 => if flag_l < 4 {
                Instruction::OpCond(block_3::call_cond_u16, OpCond::try_from(flag_l & 0b11).unwrap())
            } else {
                Instruction::Void(special::invalid)
            },
            5 => match flag_l & 1 {
                0 => Instruction::Reg16Stack(block_3::push_r16, Reg16Stack::try_from(flag_l >> 1).unwrap()),
                1 => if flag_l == 1 {
                    Instruction::Void(block_3::call_u16)
                } else {
                    Instruction::Void(special::invalid)
                },
                _ => unreachable!()
            },
            6 => match flag_l {
                0 => Instruction::Void(block_3::add_a_u8),
                1 => Instruction::Void(block_3::adc_a_u8),
                2 => Instruction::Void(block_3::sub_a_u8),
                3 => Instruction::Void(block_3::sbc_a_u8),
                4 => Instruction::Void(block_3::and_a_u8),
                5 => Instruction::Void(block_3::xor_a_u8),
                6 => Instruction::Void(block_3::or_a_u8),
                7 => Instruction::Void(block_3::cp_a_u8),
                _ => unreachable!()
            },
            7 => Instruction::U8(block_3::rst_tgt, flag_l << 3),
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

pub fn decode_cb(opcode: u8) -> Instruction {
    let block  = (opcode & 0b1100_0000) >> 6;
    let flag_l = (opcode & 0b0011_1000) >> 3;
    let flag_c =  opcode & 0b0000_0111;
    match block {
        0 => match flag_l {
            0 => Instruction::Reg8(block_bc::rlc_r8, Reg8::try_from(flag_c).unwrap()),
            1 => Instruction::Reg8(block_bc::rrc_r8, Reg8::try_from(flag_c).unwrap()),
            2 => Instruction::Reg8(block_bc::rl_r8, Reg8::try_from(flag_c).unwrap()),
            3 => Instruction::Reg8(block_bc::rr_r8, Reg8::try_from(flag_c).unwrap()),
            4 => Instruction::Reg8(block_bc::sla_r8, Reg8::try_from(flag_c).unwrap()),
            5 => Instruction::Reg8(block_bc::sra_r8, Reg8::try_from(flag_c).unwrap()),
            6 => Instruction::Reg8(block_bc::swap_r8, Reg8::try_from(flag_c).unwrap()),
            7 => Instruction::Reg8(block_bc::srl_r8, Reg8::try_from(flag_c).unwrap()),
            _ => unreachable!()
        },
        1 => Instruction::U8Reg8(block_bc::bit_b3_r8, flag_l, Reg8::try_from(flag_c).unwrap()),
        2 => Instruction::U8Reg8(block_bc::res_b3_r8, flag_l, Reg8::try_from(flag_c).unwrap()),
        3 => Instruction::U8Reg8(block_bc::set_b3_r8, flag_l, Reg8::try_from(flag_c).unwrap()),
        _ => unreachable!()
    }
}
