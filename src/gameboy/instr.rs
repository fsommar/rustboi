use failure::Error;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use crate::gameboy::cpu::flag;
use crate::gameboy::cpu::register::*;
use crate::gameboy::mem;
use crate::GameBoy;
use std::ops::DerefMut;

pub(crate) fn execute(opcode: u8, gameboy: &mut GameBoy) -> Result<u8, Error> {
    match opcode {
        0x00 => gameboy.nop(),
        0x01 => gameboy.load(R16::BC, Immediate16),
        0x02 => gameboy.load(AddrOf(R16::BC), R8::A),
        0x03 => gameboy.inc16(R16::BC),
        0x04 => gameboy.inc(R8::B),
        0x05 => gameboy.dec(R8::B),
        0x06 => gameboy.load(R8::B, Immediate8),
        0x07 => unimplemented!(),
        0x08 => unimplemented!(),
        // Storing into a deref u16 pointer seems to be only this instruction,
        // so TBD if it should be special cased or if there are other similar instructions
        // but not necessarily LD -- e.g. PUSH or POP?
        // 0x08 => gameboy.load(AddrOf(Immediate16), R16::SP),
        0x09 => gameboy.add(R16::HL, R16::BC, Carry::Without),
        0x0A => gameboy.load(R8::A, AddrOf(R16::BC)),
        0x0B => gameboy.dec16(R16::BC),
        0x0C => gameboy.inc(R8::C),
        0x0D => gameboy.dec(R8::C),
        0x0E => gameboy.load(R8::C, Immediate8),
        0x0F => unimplemented!(),
        0x10 => gameboy.stop(),
        0x11 => gameboy.load(R16::DE, Immediate16),
        0x12 => gameboy.load(AddrOf(R16::DE), R8::A),
        0x13 => gameboy.inc16(R16::DE),
        0x14 => gameboy.inc(R8::D),
        0x15 => gameboy.dec(R8::D),
        0x16 => gameboy.load(R8::D, Immediate8),
        0x17 => unimplemented!(),
        0x18 => gameboy.jump(Flags::Always, Immediate8),
        0x19 => gameboy.add(R16::HL, R16::DE, Carry::Without),
        0x1A => gameboy.load(R8::A, AddrOf(R16::DE)),
        0x1B => gameboy.dec16(R16::DE),
        0x1C => gameboy.inc(R8::C),
        0x1D => gameboy.dec(R8::C),
        0x1E => gameboy.load(R8::E, Immediate8),
        0x1F => unimplemented!(),
        0x20 => gameboy.jump(Flags::NZ, Immediate8),
        0x21 => gameboy.load(R16::HL, Immediate16),
        0x22 => gameboy.load(AddrOf(PostInc(R16::HL)), R8::A),
        0x23 => gameboy.inc16(R16::HL),
        0x24 => gameboy.inc(R8::H),
        0x25 => gameboy.dec(R8::H),
        0x26 => gameboy.load(R8::H, Immediate8),
        0x27 => unimplemented!(),
        0x28 => gameboy.jump(Flags::Z, Immediate8),
        0x29 => gameboy.add(R16::HL, R16::HL, Carry::Without),
        0x2A => gameboy.load(R8::A, AddrOf(PostInc(R16::HL))),
        0x2B => gameboy.dec16(R16::HL),
        0x2C => gameboy.inc(R8::L),
        0x2D => gameboy.dec(R8::L),
        0x2E => gameboy.load(R8::L, Immediate8),
        0x2F => unimplemented!(),
        0x30 => gameboy.jump(Flags::NC, Immediate8),
        0x31 => gameboy.load(R16::SP, Immediate16),
        0x32 => gameboy.load(AddrOf(PostDec(R16::HL)), R8::A),
        0x33 => gameboy.inc16(R16::SP),
        0x34 => gameboy.inc(AddrOf(R16::HL)),
        0x35 => gameboy.dec(AddrOf(R16::HL)),
        0x36 => gameboy.load(AddrOf(R16::HL), Immediate8),
        0x37 => unimplemented!(),
        0x38 => gameboy.jump(Flags::C, Immediate8),
        0x39 => gameboy.add(R16::HL, R16::SP, Carry::Without),
        0x3A => gameboy.load(R8::A, AddrOf(PostDec(R16::HL))),
        0x3B => gameboy.dec16(R16::SP),
        0x3C => gameboy.inc(R8::A),
        0x3D => gameboy.dec(R8::A),
        0x3E => gameboy.load(R8::A, Immediate8),
        0x3F => unimplemented!(),
        0x40 => gameboy.load(R8::B, R8::B),
        0x41 => gameboy.load(R8::B, R8::C),
        0x42 => gameboy.load(R8::B, R8::D),
        0x43 => gameboy.load(R8::B, R8::E),
        0x44 => gameboy.load(R8::B, R8::H),
        0x45 => gameboy.load(R8::B, R8::L),
        0x46 => gameboy.load(R8::B, AddrOf(R16::HL)),
        0x47 => gameboy.load(R8::B, R8::A),
        0x48 => gameboy.load(R8::C, R8::B),
        0x49 => gameboy.load(R8::C, R8::C),
        0x4A => gameboy.load(R8::C, R8::D),
        0x4B => gameboy.load(R8::C, R8::E),
        0x4C => gameboy.load(R8::C, R8::H),
        0x4D => gameboy.load(R8::C, R8::L),
        0x4E => gameboy.load(R8::C, AddrOf(R16::HL)),
        0x4F => gameboy.load(R8::C, R8::A),
        0x50 => gameboy.load(R8::D, R8::B),
        0x51 => gameboy.load(R8::D, R8::C),
        0x52 => gameboy.load(R8::D, R8::D),
        0x53 => gameboy.load(R8::D, R8::E),
        0x54 => gameboy.load(R8::D, R8::H),
        0x55 => gameboy.load(R8::D, R8::L),
        0x56 => gameboy.load(R8::D, AddrOf(R16::HL)),
        0x57 => gameboy.load(R8::D, R8::A),
        0x58 => gameboy.load(R8::E, R8::B),
        0x59 => gameboy.load(R8::E, R8::C),
        0x5A => gameboy.load(R8::E, R8::D),
        0x5B => gameboy.load(R8::E, R8::E),
        0x5C => gameboy.load(R8::E, R8::H),
        0x5D => gameboy.load(R8::E, R8::L),
        0x5E => gameboy.load(R8::E, AddrOf(R16::HL)),
        0x5F => gameboy.load(R8::E, R8::A),
        0x60 => gameboy.load(R8::H, R8::B),
        0x61 => gameboy.load(R8::H, R8::C),
        0x62 => gameboy.load(R8::H, R8::D),
        0x63 => gameboy.load(R8::H, R8::E),
        0x64 => gameboy.load(R8::H, R8::H),
        0x65 => gameboy.load(R8::H, R8::L),
        0x66 => gameboy.load(R8::H, AddrOf(R16::HL)),
        0x67 => gameboy.load(R8::H, R8::A),
        0x68 => gameboy.load(R8::L, R8::B),
        0x69 => gameboy.load(R8::L, R8::C),
        0x6A => gameboy.load(R8::L, R8::D),
        0x6B => gameboy.load(R8::L, R8::E),
        0x6C => gameboy.load(R8::L, R8::H),
        0x6D => gameboy.load(R8::L, R8::L),
        0x6E => gameboy.load(R8::L, AddrOf(R16::HL)),
        0x6F => gameboy.load(R8::L, R8::A),
        0x70 => gameboy.load(AddrOf(R16::HL), R8::B),
        0x71 => gameboy.load(AddrOf(R16::HL), R8::C),
        0x72 => gameboy.load(AddrOf(R16::HL), R8::D),
        0x73 => gameboy.load(AddrOf(R16::HL), R8::E),
        0x74 => gameboy.load(AddrOf(R16::HL), R8::H),
        0x75 => gameboy.load(AddrOf(R16::HL), R8::L),
        0x76 => gameboy.halt(),
        0x77 => gameboy.load(AddrOf(R16::HL), R8::A),
        0x78 => gameboy.load(R8::A, R8::B),
        0x79 => gameboy.load(R8::A, R8::C),
        0x7A => gameboy.load(R8::A, R8::D),
        0x7B => gameboy.load(R8::A, R8::E),
        0x7C => gameboy.load(R8::A, R8::H),
        0x7D => gameboy.load(R8::A, R8::L),
        0x7E => gameboy.load(R8::A, AddrOf(R16::HL)),
        0x7F => gameboy.load(R8::A, R8::A),
        0x80 => gameboy.add(R8::A, R8::B, Carry::Without),
        0x81 => gameboy.add(R8::A, R8::C, Carry::Without),
        0x82 => gameboy.add(R8::A, R8::D, Carry::Without),
        0x83 => gameboy.add(R8::A, R8::E, Carry::Without),
        0x84 => gameboy.add(R8::A, R8::H, Carry::Without),
        0x85 => gameboy.add(R8::A, R8::L, Carry::Without),
        0x86 => gameboy.add(R8::A, AddrOf(R16::HL), Carry::Without),
        0x87 => gameboy.add(R8::A, R8::A, Carry::Without),
        0x88 => gameboy.add(R8::A, R8::B, Carry::With),
        0x89 => gameboy.add(R8::A, R8::C, Carry::With),
        0x8A => gameboy.add(R8::A, R8::D, Carry::With),
        0x8B => gameboy.add(R8::A, R8::E, Carry::With),
        0x8C => gameboy.add(R8::A, R8::H, Carry::With),
        0x8D => gameboy.add(R8::A, R8::L, Carry::With),
        0x8E => gameboy.add(R8::A, AddrOf(R16::HL), Carry::With),
        0x8F => gameboy.add(R8::A, R8::A, Carry::With),
        0x90 => gameboy.sub(R8::A, R8::B, Carry::Without),
        0x91 => gameboy.sub(R8::A, R8::C, Carry::Without),
        0x92 => gameboy.sub(R8::A, R8::D, Carry::Without),
        0x93 => gameboy.sub(R8::A, R8::E, Carry::Without),
        0x94 => gameboy.sub(R8::A, R8::H, Carry::Without),
        0x95 => gameboy.sub(R8::A, R8::L, Carry::Without),
        0x96 => gameboy.sub(R8::A, AddrOf(R16::HL), Carry::Without),
        0x97 => gameboy.sub(R8::A, R8::A, Carry::Without),
        0x98 => gameboy.sub(R8::A, R8::B, Carry::With),
        0x99 => gameboy.sub(R8::A, R8::C, Carry::With),
        0x9A => gameboy.sub(R8::A, R8::D, Carry::With),
        0x9B => gameboy.sub(R8::A, R8::E, Carry::With),
        0x9C => gameboy.sub(R8::A, R8::H, Carry::With),
        0x9D => gameboy.sub(R8::A, R8::L, Carry::With),
        0x9E => gameboy.sub(R8::A, AddrOf(R16::HL), Carry::With),
        0x9F => gameboy.sub(R8::A, R8::A, Carry::With),
        0xA0 => gameboy.and(R8::A, R8::B),
        0xA1 => gameboy.and(R8::A, R8::C),
        0xA2 => gameboy.and(R8::A, R8::D),
        0xA3 => gameboy.and(R8::A, R8::E),
        0xA4 => gameboy.and(R8::A, R8::H),
        0xA5 => gameboy.and(R8::A, R8::L),
        0xA6 => gameboy.and(R8::A, AddrOf(R16::HL)),
        0xA7 => gameboy.and(R8::A, R8::A),
        0xA8 => gameboy.xor(R8::A, R8::B),
        0xA9 => gameboy.xor(R8::A, R8::C),
        0xAA => gameboy.xor(R8::A, R8::D),
        0xAB => gameboy.xor(R8::A, R8::E),
        0xAC => gameboy.xor(R8::A, R8::H),
        0xAD => gameboy.xor(R8::A, R8::L),
        0xAE => gameboy.xor(R8::A, AddrOf(R16::HL)),
        0xAF => gameboy.xor(R8::A, R8::A),
        0xB0 => gameboy.or(R8::A, R8::B),
        0xB1 => gameboy.or(R8::A, R8::C),
        0xB2 => gameboy.or(R8::A, R8::D),
        0xB3 => gameboy.or(R8::A, R8::E),
        0xB4 => gameboy.or(R8::A, R8::H),
        0xB5 => gameboy.or(R8::A, R8::L),
        0xB6 => gameboy.or(R8::A, AddrOf(R16::HL)),
        0xB7 => gameboy.or(R8::A, R8::A),
        0xB8 => gameboy.cmp(R8::A, R8::B),
        0xB9 => gameboy.cmp(R8::A, R8::C),
        0xBA => gameboy.cmp(R8::A, R8::D),
        0xBB => gameboy.cmp(R8::A, R8::E),
        0xBC => gameboy.cmp(R8::A, R8::H),
        0xBD => gameboy.cmp(R8::A, R8::L),
        0xBE => gameboy.cmp(R8::A, AddrOf(R16::HL)),
        0xBF => gameboy.cmp(R8::A, R8::A),
        0xC0 => unimplemented!(),
        0xC1 => gameboy.pop(R16::BC),
        0xC2 => gameboy.jump16(Flags::NZ, Immediate16),
        0xC3 => gameboy.jump16(Flags::Always, Immediate16),
        0xC4 => unimplemented!(),
        0xC5 => gameboy.push(R16::BC),
        0xC6 => gameboy.add(R8::A, Immediate8, Carry::Without),
        0xC7 => unimplemented!(),
        0xC8 => unimplemented!(),
        0xC9 => unimplemented!(),
        0xCA => gameboy.jump16(Flags::Z, Immediate16),
        0xCB => callback(gameboy),
        0xCC => unimplemented!(),
        0xCD => gameboy.call(Immediate16),
        0xCE => gameboy.add(R8::A, Immediate8, Carry::With),
        0xCF => unimplemented!(),
        0xD0 => unimplemented!(),
        0xD1 => unimplemented!(),
        0xD2 => gameboy.jump16(Flags::NC, Immediate16),
        0xD3 => unimplemented!(),
        0xD4 => unimplemented!(),
        0xD5 => gameboy.push(R16::DE),
        0xD6 => gameboy.sub(R8::A, Immediate8, Carry::Without),
        0xD7 => unimplemented!(),
        0xD8 => unimplemented!(),
        0xD9 => unimplemented!(),
        0xDA => gameboy.jump16(Flags::C, Immediate16),
        0xDB => unimplemented!(),
        0xDC => unimplemented!(),
        0xDD => unimplemented!(),
        0xDE => gameboy.sub(R8::A, Immediate8, Carry::With),
        0xDF => unimplemented!(),
        0xE0 => gameboy.load(AddrOf(Immediate8), R8::A),
        0xE1 => unimplemented!(),
        0xE2 => gameboy.load(AddrOf(R8::C), R8::A),
        0xE3 => unimplemented!(),
        0xE4 => unimplemented!(),
        0xE5 => gameboy.push(R16::HL),
        0xE6 => gameboy.and(R8::A, Immediate8),
        0xE7 => unimplemented!(),
        0xE8 => unimplemented!(),
        // ^TODO: Create separate method for this, since its behavior is unique. I.e., not:
        // gameboy.add(R16::SP, Immediate8, Carry::Without),
        0xE9 => unimplemented!(),
        0xEA => gameboy.load(AddrOf(Immediate16), R8::A),
        0xEB => unimplemented!(),
        0xEC => unimplemented!(),
        0xED => unimplemented!(),
        0xEE => gameboy.xor(R8::A, Immediate8),
        0xEF => unimplemented!(),
        0xF0 => gameboy.load(R8::A, AddrOf(Immediate8)),
        0xF1 => unimplemented!(),
        0xF2 => gameboy.load(R8::A, AddrOf(R8::C)),
        0xF3 => gameboy.set_interrupt(Interrupt::Disable),
        0xF4 => unimplemented!(),
        0xF5 => gameboy.push(R16::AF),
        0xF6 => gameboy.or(R8::A, Immediate8),
        0xF7 => unimplemented!(),
        0xF8 => gameboy.load(R16::HL, Plus(R16::SP, Immediate8)),
        0xF9 => gameboy.load(R16::SP, R16::HL),
        0xFA => gameboy.load(R8::A, AddrOf(Immediate16)),
        0xFB => gameboy.set_interrupt(Interrupt::Enable),
        0xFC => unimplemented!(),
        0xFD => unimplemented!(),
        0xFE => gameboy.cmp(R8::A, Immediate8),
        0xFF => unimplemented!(),
    }
}

fn callback(gameboy: &mut GameBoy) -> Result<u8, Error> {
    gameboy.advance_pc(1);
    let opcode = gameboy.fetch();
    match opcode {
        0x00 => gameboy.rotate_left(R8::B, Carry::Without),
        0x01 => gameboy.rotate_left(R8::C, Carry::Without),
        0x02 => gameboy.rotate_left(R8::D, Carry::Without),
        0x03 => gameboy.rotate_left(R8::E, Carry::Without),
        0x04 => gameboy.rotate_left(R8::H, Carry::Without),
        0x05 => gameboy.rotate_left(R8::L, Carry::Without),
        0x06 => gameboy.rotate_left(AddrOf(R16::HL), Carry::Without),
        0x07 => gameboy.rotate_left(R8::A, Carry::Without),
        0x08 => gameboy.rotate_right(R8::B, Carry::Without),
        0x09 => gameboy.rotate_right(R8::C, Carry::Without),
        0x0A => gameboy.rotate_right(R8::D, Carry::Without),
        0x0B => gameboy.rotate_right(R8::E, Carry::Without),
        0x0C => gameboy.rotate_right(R8::H, Carry::Without),
        0x0D => gameboy.rotate_right(R8::L, Carry::Without),
        0x0E => gameboy.rotate_right(AddrOf(R16::HL), Carry::Without),
        0x0F => gameboy.rotate_right(R8::A, Carry::Without),
        0x10 => gameboy.rotate_left(R8::B, Carry::With),
        0x11 => gameboy.rotate_left(R8::C, Carry::With),
        0x12 => gameboy.rotate_left(R8::D, Carry::With),
        0x13 => gameboy.rotate_left(R8::E, Carry::With),
        0x14 => gameboy.rotate_left(R8::H, Carry::With),
        0x15 => gameboy.rotate_left(R8::L, Carry::With),
        0x16 => gameboy.rotate_left(AddrOf(R16::HL), Carry::With),
        0x17 => gameboy.rotate_left(R8::A, Carry::With),
        0x18 => gameboy.rotate_right(R8::B, Carry::With),
        0x19 => gameboy.rotate_right(R8::C, Carry::With),
        0x1A => gameboy.rotate_right(R8::D, Carry::With),
        0x1B => gameboy.rotate_right(R8::E, Carry::With),
        0x1C => gameboy.rotate_right(R8::H, Carry::With),
        0x1D => gameboy.rotate_right(R8::L, Carry::With),
        0x1E => gameboy.rotate_right(AddrOf(R16::HL), Carry::With),
        0x1F => gameboy.rotate_right(R8::A, Carry::With),
        0x20 => gameboy.shift_left(R8::B),
        0x21 => gameboy.shift_left(R8::C),
        0x22 => gameboy.shift_left(R8::D),
        0x23 => gameboy.shift_left(R8::E),
        0x24 => gameboy.shift_left(R8::H),
        0x25 => gameboy.shift_left(R8::L),
        0x26 => gameboy.shift_left(AddrOf(R16::HL)),
        0x27 => gameboy.shift_left(R8::A),
        0x28 => gameboy.shift_right(R8::B),
        0x29 => gameboy.shift_right(R8::C),
        0x2A => gameboy.shift_right(R8::D),
        0x2B => gameboy.shift_right(R8::E),
        0x2C => gameboy.shift_right(R8::H),
        0x2D => gameboy.shift_right(R8::L),
        0x2E => gameboy.shift_right(AddrOf(R16::HL)),
        0x2F => gameboy.shift_right(R8::A),
        0x30 => gameboy.swap(R8::B),
        0x31 => gameboy.swap(R8::C),
        0x32 => gameboy.swap(R8::D),
        0x33 => gameboy.swap(R8::E),
        0x34 => gameboy.swap(R8::H),
        0x35 => gameboy.swap(R8::L),
        0x36 => gameboy.swap(AddrOf(R16::HL)),
        0x37 => gameboy.swap(R8::A),
        0x38 => gameboy.shift_right_logical(R8::B),
        0x39 => gameboy.shift_right_logical(R8::C),
        0x3A => gameboy.shift_right_logical(R8::D),
        0x3B => gameboy.shift_right_logical(R8::E),
        0x3C => gameboy.shift_right_logical(R8::H),
        0x3D => gameboy.shift_right_logical(R8::L),
        0x3E => gameboy.shift_right_logical(AddrOf(R16::HL)),
        0x3F => gameboy.shift_right_logical(R8::A),
        0x40 => gameboy.bit(0, R8::B),
        0x41 => gameboy.bit(0, R8::C),
        0x42 => gameboy.bit(0, R8::D),
        0x43 => gameboy.bit(0, R8::E),
        0x44 => gameboy.bit(0, R8::H),
        0x45 => gameboy.bit(0, R8::L),
        0x46 => gameboy.bit(0, AddrOf(R16::HL)),
        0x47 => gameboy.bit(0, R8::A),
        0x48 => gameboy.bit(1, R8::B),
        0x49 => gameboy.bit(1, R8::C),
        0x4A => gameboy.bit(1, R8::D),
        0x4B => gameboy.bit(1, R8::E),
        0x4C => gameboy.bit(1, R8::H),
        0x4D => gameboy.bit(1, R8::L),
        0x4E => gameboy.bit(1, AddrOf(R16::HL)),
        0x4F => gameboy.bit(1, R8::A),
        0x50 => gameboy.bit(2, R8::B),
        0x51 => gameboy.bit(2, R8::C),
        0x52 => gameboy.bit(2, R8::D),
        0x53 => gameboy.bit(2, R8::E),
        0x54 => gameboy.bit(2, R8::H),
        0x55 => gameboy.bit(2, R8::L),
        0x56 => gameboy.bit(2, AddrOf(R16::HL)),
        0x57 => gameboy.bit(2, R8::A),
        0x58 => gameboy.bit(3, R8::B),
        0x59 => gameboy.bit(3, R8::C),
        0x5A => gameboy.bit(3, R8::D),
        0x5B => gameboy.bit(3, R8::E),
        0x5C => gameboy.bit(3, R8::H),
        0x5D => gameboy.bit(3, R8::L),
        0x5E => gameboy.bit(3, AddrOf(R16::HL)),
        0x5F => gameboy.bit(3, R8::A),
        0x60 => gameboy.bit(4, R8::B),
        0x61 => gameboy.bit(4, R8::C),
        0x62 => gameboy.bit(4, R8::D),
        0x63 => gameboy.bit(4, R8::E),
        0x64 => gameboy.bit(4, R8::H),
        0x65 => gameboy.bit(4, R8::L),
        0x66 => gameboy.bit(4, AddrOf(R16::HL)),
        0x67 => gameboy.bit(4, R8::A),
        0x68 => gameboy.bit(5, R8::B),
        0x69 => gameboy.bit(5, R8::C),
        0x6A => gameboy.bit(5, R8::D),
        0x6B => gameboy.bit(5, R8::E),
        0x6C => gameboy.bit(5, R8::H),
        0x6D => gameboy.bit(5, R8::L),
        0x6E => gameboy.bit(5, AddrOf(R16::HL)),
        0x6F => gameboy.bit(5, R8::A),
        0x70 => gameboy.bit(6, R8::B),
        0x71 => gameboy.bit(6, R8::C),
        0x72 => gameboy.bit(6, R8::D),
        0x73 => gameboy.bit(6, R8::E),
        0x74 => gameboy.bit(6, R8::H),
        0x75 => gameboy.bit(6, R8::L),
        0x76 => gameboy.bit(6, AddrOf(R16::HL)),
        0x77 => gameboy.bit(6, R8::A),
        0x78 => gameboy.bit(7, R8::B),
        0x79 => gameboy.bit(7, R8::C),
        0x7A => gameboy.bit(7, R8::D),
        0x7B => gameboy.bit(7, R8::E),
        0x7C => gameboy.bit(7, R8::H),
        0x7D => gameboy.bit(7, R8::L),
        0x7E => gameboy.bit(7, AddrOf(R16::HL)),
        0x7F => gameboy.bit(7, R8::A),
        0x80 => gameboy.res(0, R8::B),
        0x81 => gameboy.res(0, R8::C),
        0x82 => gameboy.res(0, R8::D),
        0x83 => gameboy.res(0, R8::E),
        0x84 => gameboy.res(0, R8::H),
        0x85 => gameboy.res(0, R8::L),
        0x86 => gameboy.res(0, AddrOf(R16::HL)),
        0x87 => gameboy.res(0, R8::A),
        0x88 => gameboy.res(1, R8::B),
        0x89 => gameboy.res(1, R8::C),
        0x8A => gameboy.res(1, R8::D),
        0x8B => gameboy.res(1, R8::E),
        0x8C => gameboy.res(1, R8::H),
        0x8D => gameboy.res(1, R8::L),
        0x8E => gameboy.res(1, AddrOf(R16::HL)),
        0x8F => gameboy.res(1, R8::A),
        0x90 => gameboy.res(2, R8::B),
        0x91 => gameboy.res(2, R8::C),
        0x92 => gameboy.res(2, R8::D),
        0x93 => gameboy.res(2, R8::E),
        0x94 => gameboy.res(2, R8::H),
        0x95 => gameboy.res(2, R8::L),
        0x96 => gameboy.res(2, AddrOf(R16::HL)),
        0x97 => gameboy.res(2, R8::A),
        0x98 => gameboy.res(3, R8::B),
        0x99 => gameboy.res(3, R8::C),
        0x9A => gameboy.res(3, R8::D),
        0x9B => gameboy.res(3, R8::E),
        0x9C => gameboy.res(3, R8::H),
        0x9D => gameboy.res(3, R8::L),
        0x9E => gameboy.res(3, AddrOf(R16::HL)),
        0x9F => gameboy.res(3, R8::A),
        0xA0 => gameboy.res(4, R8::B),
        0xA1 => gameboy.res(4, R8::C),
        0xA2 => gameboy.res(4, R8::D),
        0xA3 => gameboy.res(4, R8::E),
        0xA4 => gameboy.res(4, R8::H),
        0xA5 => gameboy.res(4, R8::L),
        0xA6 => gameboy.res(4, AddrOf(R16::HL)),
        0xA7 => gameboy.res(4, R8::A),
        0xA8 => gameboy.res(5, R8::B),
        0xA9 => gameboy.res(5, R8::C),
        0xAA => gameboy.res(5, R8::D),
        0xAB => gameboy.res(5, R8::E),
        0xAC => gameboy.res(5, R8::H),
        0xAD => gameboy.res(5, R8::L),
        0xAE => gameboy.res(5, AddrOf(R16::HL)),
        0xAF => gameboy.res(5, R8::A),
        0xB0 => gameboy.res(6, R8::B),
        0xB1 => gameboy.res(6, R8::C),
        0xB2 => gameboy.res(6, R8::D),
        0xB3 => gameboy.res(6, R8::E),
        0xB4 => gameboy.res(6, R8::H),
        0xB5 => gameboy.res(6, R8::L),
        0xB6 => gameboy.res(6, AddrOf(R16::HL)),
        0xB7 => gameboy.res(6, R8::A),
        0xB8 => gameboy.res(7, R8::B),
        0xB9 => gameboy.res(7, R8::C),
        0xBA => gameboy.res(7, R8::D),
        0xBB => gameboy.res(7, R8::E),
        0xBC => gameboy.res(7, R8::H),
        0xBD => gameboy.res(7, R8::L),
        0xBE => gameboy.res(7, AddrOf(R16::HL)),
        0xBF => gameboy.res(7, R8::A),
        0xC0 => gameboy.set(0, R8::B),
        0xC1 => gameboy.set(0, R8::C),
        0xC2 => gameboy.set(0, R8::D),
        0xC3 => gameboy.set(0, R8::E),
        0xC4 => gameboy.set(0, R8::H),
        0xC5 => gameboy.set(0, R8::L),
        0xC6 => gameboy.set(0, AddrOf(R16::HL)),
        0xC7 => gameboy.set(0, R8::A),
        0xC8 => gameboy.set(1, R8::B),
        0xC9 => gameboy.set(1, R8::C),
        0xCA => gameboy.set(1, R8::D),
        0xCB => gameboy.set(1, R8::E),
        0xCC => gameboy.set(1, R8::H),
        0xCD => gameboy.set(1, R8::L),
        0xCE => gameboy.set(1, AddrOf(R16::HL)),
        0xCF => gameboy.set(1, R8::A),
        0xD0 => gameboy.set(2, R8::B),
        0xD1 => gameboy.set(2, R8::C),
        0xD2 => gameboy.set(2, R8::D),
        0xD3 => gameboy.set(2, R8::E),
        0xD4 => gameboy.set(2, R8::H),
        0xD5 => gameboy.set(2, R8::L),
        0xD6 => gameboy.set(2, AddrOf(R16::HL)),
        0xD7 => gameboy.set(2, R8::A),
        0xD8 => gameboy.set(3, R8::B),
        0xD9 => gameboy.set(3, R8::C),
        0xDA => gameboy.set(3, R8::D),
        0xDB => gameboy.set(3, R8::E),
        0xDC => gameboy.set(3, R8::H),
        0xDD => gameboy.set(3, R8::L),
        0xDE => gameboy.set(3, AddrOf(R16::HL)),
        0xDF => gameboy.set(3, R8::A),
        0xE0 => gameboy.set(4, R8::B),
        0xE1 => gameboy.set(4, R8::C),
        0xE2 => gameboy.set(4, R8::D),
        0xE3 => gameboy.set(4, R8::E),
        0xE4 => gameboy.set(4, R8::H),
        0xE5 => gameboy.set(4, R8::L),
        0xE6 => gameboy.set(4, AddrOf(R16::HL)),
        0xE7 => gameboy.set(4, R8::A),
        0xE8 => gameboy.set(5, R8::B),
        0xE9 => gameboy.set(5, R8::C),
        0xEA => gameboy.set(5, R8::D),
        0xEB => gameboy.set(5, R8::E),
        0xEC => gameboy.set(5, R8::H),
        0xED => gameboy.set(5, R8::L),
        0xEE => gameboy.set(5, AddrOf(R16::HL)),
        0xEF => gameboy.set(5, R8::A),
        0xF0 => gameboy.set(6, R8::B),
        0xF1 => gameboy.set(6, R8::C),
        0xF2 => gameboy.set(6, R8::D),
        0xF3 => gameboy.set(6, R8::E),
        0xF4 => gameboy.set(6, R8::H),
        0xF5 => gameboy.set(6, R8::L),
        0xF6 => gameboy.set(6, AddrOf(R16::HL)),
        0xF7 => gameboy.set(6, R8::A),
        0xF8 => gameboy.set(7, R8::B),
        0xF9 => gameboy.set(7, R8::C),
        0xFA => gameboy.set(7, R8::D),
        0xFB => gameboy.set(7, R8::E),
        0xFC => gameboy.set(7, R8::H),
        0xFD => gameboy.set(7, R8::L),
        0xFE => gameboy.set(7, AddrOf(R16::HL)),
        0xFF => gameboy.set(7, R8::A),
    }
}

struct Plus<T, U>(T, U);

struct PostDec<T>(T);

struct PostInc<T>(T);

struct NoWrite<T>(T);

struct Constant<T>(T);

struct AddrOf<T>(T);

struct Immediate8;

struct Immediate16;

trait AsAddr {
    fn into_addr(self) -> u16;
}

impl AsAddr for u16 {
    fn into_addr(self) -> u16 {
        self
    }
}

impl AsAddr for u8 {
    fn into_addr(self) -> u16 {
        let le = u16::from(self) << 8;
        le + 0xff_u16.to_le()
    }
}

trait Integer: num_traits::PrimInt + num_traits::Unsigned + num_traits::WrappingSub {
    const CYCLES: u8;
    const HALF_CARRY_FLAG: Self;
    fn from_carry(carry: Carry) -> Self;
    fn set_zero_flag(f: &mut RegisterF, res: Self);
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
}

impl Integer for u16 {
    const CYCLES: u8 = 2;
    const HALF_CARRY_FLAG: Self = 0x100;

    fn from_carry(carry: Carry) -> Self {
        carry.to_u16().unwrap()
    }

    fn set_zero_flag(_f: &mut RegisterF, _res: Self) {
        // Do nothing
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        u16::overflowing_add(self, rhs)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        u16::overflowing_sub(self, rhs)
    }
}

impl Integer for u8 {
    const CYCLES: u8 = 1;
    const HALF_CARRY_FLAG: Self = 0x10;

    fn from_carry(carry: Carry) -> Self {
        carry.to_u8().unwrap()
    }

    fn set_zero_flag(f: &mut RegisterF, res: Self) {
        f[flag::Z].set_bool(res == 0);
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        u8::overflowing_add(self, rhs)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        u8::overflowing_sub(self, rhs)
    }
}

impl<T, U, UNum, Num> mem::Read for Plus<T, U>
where
    T: mem::Read<Out = Num>,
    U: mem::Read<Out = UNum>,
    UNum: Into<Num>,
    Num: Integer,
{
    type Out = Num;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        let lhs = self.0.read(gb);
        let rhs = self.1.read(gb);
        lhs + rhs.into()
    }
}

impl<W, T> mem::Read for PostInc<W>
where
    T: Integer,
    W: mem::Write<In = T> + mem::Read<Out = T>,
{
    type Out = T;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        let value = self.0.read(gb);
        self.0
            .write(gb, value + T::one())
            .expect("post inc write must not fail");
        value
    }
}

impl<W, T> mem::Read for PostDec<W>
where
    T: Integer,
    W: mem::Write<In = T> + mem::Read<Out = T>,
{
    type Out = T;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        let value = self.0.read(gb);
        self.0
            .write(gb, value.wrapping_sub(&T::one()))
            .expect("post dec write must not fail");
        value
    }
}

impl<A: AsAddr, R: mem::Read<Out = A>> mem::Read for AddrOf<R> {
    type Out = u8;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        let addr = self.0.read(gb).into_addr();
        gb.mmu.read_u8(addr)
    }
}

impl<A: AsAddr, R: mem::Read<Out = A>> mem::Write for AddrOf<R> {
    type In = u8;
    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), Error> {
        let addr = self.0.read(gb).into_addr();
        gb.mmu.write_u8(addr, value);
        Ok(())
    }
}

impl<V, T: mem::Write<In = V>> mem::Write for NoWrite<T> {
    type In = V;
    fn write(&self, _: &mut GameBoy, _: Self::In) -> Result<(), Error> {
        // NoWrite causes write to be a NOOP.
        Ok(())
    }
}

impl<V, T: mem::Read<Out = V>> mem::Read for NoWrite<T> {
    type Out = V;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        self.0.read(gb)
    }
}

impl<T: Copy> mem::Read for Constant<T> {
    type Out = T;
    fn read(&self, _: &mut GameBoy) -> Self::Out {
        self.0
    }
}

impl mem::Read for Immediate8 {
    type Out = u8;
    const CYCLES: u8 = 1;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        gb.mmu.read_u8(*gb.cpu.register.pc + 1)
    }
}

impl mem::Read for Immediate16 {
    type Out = u16;
    const CYCLES: u8 = 2;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        gb.mmu.read_u16(*gb.cpu.register.pc + 1)
    }
}

enum Flags {
    Always,
    Z,
    N,
    H,
    C,
    NZ,
    NN,
    NH,
    NC,
}

enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl mem::Read for R8 {
    type Out = u8;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        use self::R8::*;
        match self {
            A => gb.cpu.register.a(),
            B => gb.cpu.register.b(),
            C => gb.cpu.register.c(),
            D => gb.cpu.register.d(),
            E => gb.cpu.register.e(),
            H => gb.cpu.register.h(),
            L => gb.cpu.register.l(),
        }
    }
}

impl mem::Write for R8 {
    type In = u8;
    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), Error> {
        use self::R8::*;
        let reg: &mut u8 = &mut match self {
            A => gb.cpu.register.a(),
            B => gb.cpu.register.b(),
            C => gb.cpu.register.c(),
            D => gb.cpu.register.d(),
            E => gb.cpu.register.e(),
            H => gb.cpu.register.h(),
            L => gb.cpu.register.l(),
        };
        *reg = value;
        Ok(())
    }
}

enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl mem::Read for R16 {
    type Out = u16;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        use self::R16::*;
        match self {
            AF => *gb.cpu.register.af,
            BC => *gb.cpu.register.bc,
            DE => *gb.cpu.register.de,
            HL => *gb.cpu.register.hl,
            SP => *gb.cpu.register.sp,
        }
    }
}

impl mem::Write for R16 {
    type In = u16;
    const CYCLES: u8 = 1;

    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), Error> {
        use self::R16::*;
        let reg: &mut u16 = match self {
            AF => gb.cpu.register.af.deref_mut(),
            BC => gb.cpu.register.bc.deref_mut(),
            DE => gb.cpu.register.de.deref_mut(),
            HL => gb.cpu.register.hl.deref_mut(),
            SP => gb.cpu.register.sp.deref_mut(),
        };
        *reg = value;
        Ok(())
    }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum Carry {
    Without,
    With,
}

#[derive(Debug)]
enum Interrupt {
    Enable,
    Disable,
}

trait Instructions {
    type Output;

    fn load<W, R, V>(&mut self, _: W, _: R) -> Self::Output
    where
        W: mem::Write<In = V>,
        R: mem::Read<Out = V>;

    fn binary_op<LHS, RHS, F, Num>(&mut self, _: LHS, _: RHS, _: F) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        F: FnOnce(Num, Num, RegisterF) -> (Num, RegisterF),
        Num: Integer;

    fn jump<Offset>(&mut self, _: Flags, _: Offset) -> Self::Output
    where
        Offset: mem::Read<Out = u8>;

    fn jump16<Offset>(&mut self, _: Flags, _: Offset) -> Self::Output
    where
        Offset: mem::Read<Out = u16>;

    fn call<R>(&mut self, address: R) -> Self::Output
    where
        R: mem::Read<Out = u16>;

    fn set_interrupt(&mut self, _: Interrupt) -> Self::Output;

    fn add<LHS, RHS, Num>(&mut self, lhs: LHS, rhs: RHS, carry: Carry) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        Num: Integer,
    {
        self.binary_op(lhs, rhs, |x, y, mut f| {
            let (res, overflow1) = x.overflowing_add(y);
            let (res, overflow2) = res.overflowing_add(Num::from_carry(carry));
            Num::set_zero_flag(&mut f, res);
            f[flag::H].set_bool((x ^ y ^ res) & Num::HALF_CARRY_FLAG != Num::zero());
            f[flag::C].set_bool(overflow1 || overflow2);
            (res, f)
        })
    }

    fn sub<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS, carry: Carry) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        let carry = carry.to_u8().unwrap();
        self.binary_op(lhs, rhs, |x, y, mut f| {
            let (res, overflow1) = x.overflowing_sub(y);
            let (res, overflow2) = res.overflowing_sub(carry);
            f[flag::Z].set_bool(res == 0);
            f[flag::N].set();
            f[flag::H].set_bool((x ^ y ^ res) & u8::HALF_CARRY_FLAG != 0);
            f[flag::C].set_bool(overflow1 || overflow2);
            (res, f)
        })
    }

    fn xor<LHS, RHS, Num>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        Num: Integer,
    {
        self.binary_op(lhs, rhs, |x, y, mut f| {
            let res = x ^ y;
            f[flag::Z].set_bool(res == Num::zero());
            (res, f)
        })
    }

    fn and<LHS, RHS, Num>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        Num: Integer,
    {
        self.binary_op(lhs, rhs, |x, y, mut f| {
            let res = x & y;
            f[flag::Z].set_bool(res == Num::zero());
            f[flag::N].reset();
            f[flag::H].set();
            f[flag::C].reset();
            (res, f)
        })
    }

    fn or<LHS, RHS, Num>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        Num: Integer,
    {
        self.binary_op(lhs, rhs, |x, y, mut f| {
            let res = x | y;
            f[flag::Z].set_bool(res == Num::zero());
            f[flag::N].reset();
            f[flag::H].reset();
            f[flag::C].reset();
            (res, f)
        })
    }

    fn cmp<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        // Ensure that the result is not written to the LHS
        self.sub(NoWrite(lhs), rhs, Carry::Without)
    }

    fn inc<T>(&mut self, lhs: T) -> Self::Output
    where
        T: mem::Read<Out = u8> + mem::Write<In = u8>,
    {
        self.binary_op(lhs, Constant(1), |x, y, mut f| {
            let res = x.wrapping_add(y);
            f[flag::Z].set_bool(res == 0);
            f[flag::N].reset();
            f[flag::H].set_bool((x ^ y ^ res) & u8::HALF_CARRY_FLAG != 0);
            (res, f)
        })
    }

    fn inc16<T>(&mut self, lhs: T) -> Self::Output
    where
        T: mem::Read<Out = u16> + mem::Write<In = u16>,
    {
        self.binary_op(lhs, Constant(1), |x, y, f| (x + y, f))
    }

    fn dec<T>(&mut self, lhs: T) -> Self::Output
    where
        T: mem::Read<Out = u8> + mem::Write<In = u8>,
    {
        self.binary_op(lhs, Constant(1), |x, y, mut f| {
            let res = x.wrapping_sub(y);
            f[flag::Z].set_bool(res == 0);
            f[flag::N].set();
            f[flag::H].set_bool((x ^ y ^ res) & u8::HALF_CARRY_FLAG != 0);
            (res, f)
        })
    }

    fn dec16<T>(&mut self, lhs: T) -> Self::Output
    where
        T: mem::Read<Out = u16> + mem::Write<In = u16>,
    {
        self.binary_op(lhs, Constant(1), |x, y, f| (x - y, f))
    }

    fn push<R>(&mut self, from: R) -> Self::Output
    where
        R: mem::Read<Out = u16>;
    fn pop<W>(&mut self, to: W) -> Self::Output
    where
        W: mem::Write<In = u16>;
    fn nop(&mut self) -> Self::Output;
    fn halt(&mut self) -> Self::Output;
    fn stop(&mut self) -> Self::Output;
    fn shift_left<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn shift_right<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn shift_right_logical<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn rotate_left<R>(&mut self, register: R, carry: Carry) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn rotate_right<R>(&mut self, register: R, carry: Carry) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn swap<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn bit<R>(&mut self, pos: u8, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn res<R>(&mut self, pos: u8, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>;
    fn set<R>(&mut self, pos: u8, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>;
}

impl Instructions for GameBoy {
    type Output = Result<u8, Error>;

    fn load<W, R, V>(&mut self, to: W, from: R) -> Self::Output
    where
        W: mem::Write<In = V>,
        R: mem::Read<Out = V>,
    {
        let value: V = from.read(self);
        to.write(self, value)?;
        // This logic might be a bit contrived, but if I can prove it by
        // e.g. testing all combinations, then it can stay.
        Ok(W::CYCLES + R::CYCLES)
    }

    fn binary_op<LHS, RHS, F, Num>(&mut self, lhs: LHS, rhs: RHS, op: F) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        F: FnOnce(Num, Num, RegisterF) -> (Num, RegisterF),
        Num: Integer,
    {
        let lhs_ = lhs.read(self);
        let rhs_ = rhs.read(self);
        let (result, f) = op(lhs_, rhs_, *(self.cpu.register.f()));
        *(self.cpu.register.f()) = f;
        lhs.write(self, result)?;
        Ok(Num::CYCLES)
    }

    fn jump<Offset>(&mut self, flags: Flags, offset: Offset) -> Self::Output
    where
        Offset: mem::Read<Out = u8>,
    {
        use self::Flags::*;
        let offset = offset.read(self);
        let new_addr = u16::from(offset) | 0xFF00;
        println!(
            "Read offset as ${:02X?}, converted to ${:04X?}",
            offset, new_addr,
        );
        let f = *(self.cpu.register.f());
        if match flags {
            Always => true,
            Z => f[flag::Z].as_bool(),
            N => f[flag::N].as_bool(),
            H => f[flag::H].as_bool(),
            C => f[flag::C].as_bool(),
            NZ => !f[flag::Z].as_bool(),
            NN => !f[flag::N].as_bool(),
            NH => !f[flag::H].as_bool(),
            NC => !f[flag::C].as_bool(),
        } {
            *self.cpu.register.pc = self.cpu.register.pc.wrapping_add(new_addr);
        }
        Ok(0)
    }

    fn jump16<Offset>(&mut self, flags: Flags, offset: Offset) -> Self::Output
    where
        Offset: mem::Read<Out = u16>,
    {
        use self::Flags::*;
        let offset = offset.read(self);
        let f = *(self.cpu.register.f());
        if match flags {
            Always => true,
            Z => f[flag::Z].as_bool(),
            N => f[flag::N].as_bool(),
            H => f[flag::H].as_bool(),
            C => f[flag::C].as_bool(),
            NZ => !f[flag::Z].as_bool(),
            NN => !f[flag::N].as_bool(),
            NH => !f[flag::H].as_bool(),
            NC => !f[flag::C].as_bool(),
        } {
            *self.cpu.register.pc += offset;
        }
        Ok(0)
    }

    fn call<R>(&mut self, address: R) -> Self::Output
    where
        R: mem::Read<Out = u16>,
    {
        let address = address.read(self);
        println!("Read address 0x{:04X?} for call", address);
        self.push(Constant(*self.cpu.register.pc))?;
        *self.cpu.register.pc = address;
        Ok(0)
    }

    fn set_interrupt(&mut self, _: Interrupt) -> Self::Output {
        // TODO: Handle interrupts
        Ok(1)
    }

    fn push<R>(&mut self, from: R) -> Self::Output
    where
        R: mem::Read<Out = u16>,
    {
        let from = from.read(self);
        let sp: &mut u16 = &mut self.cpu.register.sp;
        self.mmu.write_u16(*sp, from);
        *sp = sp.wrapping_add(2);
        Ok(1)
    }

    fn pop<W>(&mut self, to: W) -> Self::Output
    where
        W: mem::Write<In = u16>,
    {
        let value = self.mmu.read_u16(*self.cpu.register.sp);
        to.write(self, value)?;
        let sp: &mut u16 = &mut self.cpu.register.sp;
        *sp = sp.wrapping_sub(2);
        Ok(1)
    }

    fn nop(&mut self) -> Self::Output {
        Ok(1)
    }

    fn halt(&mut self) -> Self::Output {
        unimplemented!()
    }

    fn stop(&mut self) -> Self::Output {
        unimplemented!()
    }

    fn shift_left<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }

    fn shift_right<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }

    fn shift_right_logical<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }

    fn rotate_left<R>(&mut self, register: R, carry: Carry) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }

    fn rotate_right<R>(&mut self, register: R, carry: Carry) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }

    fn swap<R>(&mut self, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }

    fn bit<R>(&mut self, pos: u8, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        let register_ = register.read(self);
        let f = self.cpu.register.f();
        if register_ & (1 << pos) == 0 {
            f[flag::Z].set();
        }
        f[flag::N].reset();
        f[flag::H].set();
        Ok(2)
    }

    fn res<R>(&mut self, pos: u8, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }

    fn set<R>(&mut self, pos: u8, register: R) -> Self::Output
    where
        R: mem::Read<Out = u8>,
    {
        unimplemented!()
    }
}
