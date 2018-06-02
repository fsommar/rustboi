use super::*;
use std::convert::{From, TryFrom};

pub(crate) fn execute(opcode: u8, gameboy: &mut GameBoy) -> Result<u8, String> {
    match opcode {
        0x47 => gameboy.load(R8::C, R8::A),
        0x57 => gameboy.load(R8::E, R8::A),
        0x67 => gameboy.load(R8::L, R8::A),
        0x77 => gameboy.load(R8::A, R8::A),
        0x0e => gameboy.load(R8::C, Immediate8),
        0x1e => gameboy.load(R8::E, Immediate8),
        0x2e => gameboy.load(R8::L, Immediate8),
        0x3e => gameboy.load(R8::A, Immediate8),
        0x4e => gameboy.load(R8::C, AddrOf(R16::HL)),
        0x5e => gameboy.load(R8::E, AddrOf(R16::HL)),
        0x6e => gameboy.load(R8::L, AddrOf(R16::HL)),
        0x7e => gameboy.load(R8::A, AddrOf(R16::HL)),
        0x4d => gameboy.load(R8::C, R8::L),
        0x5d => gameboy.load(R8::E, R8::L),
        0x6d => gameboy.load(R8::L, R8::L),
        0x7d => gameboy.load(R8::A, R8::L),
        0x4c => gameboy.load(R8::C, R8::H),
        0x5c => gameboy.load(R8::E, R8::H),
        0x6c => gameboy.load(R8::L, R8::H),
        0x7c => gameboy.load(R8::A, R8::H),
        0x4b => gameboy.load(R8::C, R8::E),
        0x5b => gameboy.load(R8::E, R8::E),
        0x6b => gameboy.load(R8::L, R8::E),
        0x7b => gameboy.load(R8::A, R8::E),
        0x4a => gameboy.load(R8::C, R8::D),
        0x5a => gameboy.load(R8::E, R8::D),
        0x6a => gameboy.load(R8::L, R8::D),
        0x7a => gameboy.load(R8::A, R8::D),
        0x49 => gameboy.load(R8::C, R8::C),
        0x59 => gameboy.load(R8::E, R8::C),
        0x69 => gameboy.load(R8::L, R8::C),
        0x79 => gameboy.load(R8::A, R8::C),
        0x48 => gameboy.load(R8::C, R8::B),
        0x58 => gameboy.load(R8::E, R8::B),
        0x68 => gameboy.load(R8::L, R8::B),
        0x78 => gameboy.load(R8::A, R8::B),
        0x47 => gameboy.load(R8::B,           R8::A),
        0x57 => gameboy.load(R8::D,           R8::A),
        0x67 => gameboy.load(R8::H,           R8::A),
        0x77 => gameboy.load(AddrOf(R16::HL), R8::A),
        0x46 => gameboy.load(R8::B, AddrOf(R16::HL)),
        0x56 => gameboy.load(R8::D, AddrOf(R16::HL)),
        0x66 => gameboy.load(R8::H, AddrOf(R16::HL)),
        0x76 => gameboy.halt(),
        0x45 => gameboy.load(R8::B,           R8::L),
        0x55 => gameboy.load(R8::D,           R8::L),
        0x65 => gameboy.load(R8::H,           R8::L),
        0x75 => gameboy.load(AddrOf(R16::HL), R8::L),
        0x44 => gameboy.load(R8::B,           R8::H),
        0x54 => gameboy.load(R8::D,           R8::H),
        0x64 => gameboy.load(R8::H,           R8::H),
        0x74 => gameboy.load(AddrOf(R16::HL), R8::H),
        0x43 => gameboy.load(R8::B,           R8::E),
        0x53 => gameboy.load(R8::D,           R8::E),
        0x63 => gameboy.load(R8::H,           R8::E),
        0x73 => gameboy.load(AddrOf(R16::HL), R8::E),
        0x42 => gameboy.load(R8::B,           R8::D),
        0x52 => gameboy.load(R8::D,           R8::D),
        0x62 => gameboy.load(R8::H,           R8::D),
        0x72 => gameboy.load(AddrOf(R16::HL), R8::D),
        0x41 => gameboy.load(R8::B,           R8::C),
        0x51 => gameboy.load(R8::D,           R8::C),
        0x61 => gameboy.load(R8::H,           R8::C),
        0x71 => gameboy.load(AddrOf(R16::HL), R8::C),
        0x40 => gameboy.load(R8::B,           R8::B),
        0x50 => gameboy.load(R8::D,           R8::B),
        0x60 => gameboy.load(R8::H,           R8::B),
        0x70 => gameboy.load(AddrOf(R16::HL), R8::B),
        _ => Err(format!("Unable to match opcode {:x}", opcode)),
    }
}

struct AddrOf<T>(T);
struct Immediate8;

impl<R: mem::Read<Out=u16>> mem::Read for AddrOf<R> {
    type Out = u8;
    fn read(&self, gb: &GameBoy) -> Self::Out {
        gb.mmu.read_u8(self.0.read(gb))
    }
}

impl<R: mem::Read<Out=u16>> mem::Write for AddrOf<R> {
    type In = u8;
    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), String> {
        let addr = self.0.read(gb);
        gb.mmu.write_u8(addr, value);
        Ok(())
    }
}

impl mem::Read for Immediate8 {
    type Out = u8;
    fn read(&self, gb: &GameBoy) -> Self::Out {
        gb.mmu.read_u8(gb.cpu.register.pc())
    }
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
    fn read(&self, gb: &GameBoy) -> Self::Out {
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
    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), String> {
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
}

impl mem::Read for R16 {
    type Out = u16;
    fn read(&self, gb: &GameBoy) -> Self::Out {
        match self {
            AF => gb.cpu.register.af(),
            BC => gb.cpu.register.bc(),
            DE => gb.cpu.register.de(),
            HL => gb.cpu.register.hl(),
        }
    }
}

impl mem::Write for R16 {
    type In = u16;
    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), String> {
        let reg: &mut u16 = &mut match self {
            AF => gb.cpu.register.af(),
            BC => gb.cpu.register.bc(),
            DE => gb.cpu.register.de(),
            HL => gb.cpu.register.hl(),
        };
        *reg = value;
        Ok(())
    }
}

trait Instructions {
    type Output;
    fn load<R: mem::Read<Out = V>, W: mem::Write<In = V>, V>(&mut self, W, R) -> Self::Output;
    fn halt(&mut self) -> Self::Output;
}

impl Instructions for GameBoy {
    type Output = Result<u8, String>;

    fn load<R: mem::Read<Out = V>, W: mem::Write<In = V>, V>(
        &mut self,
        to: W,
        from: R,
    ) -> Self::Output {
        let value: V = from.read(self);
        to.write(self, value)?;
        Ok(1)
    }

    fn halt(&mut self) -> Self::Output {
        unimplemented!()
    }
}
