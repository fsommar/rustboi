use super::*;
use std::convert::{From, TryFrom};

pub(crate) fn execute(opcode: u8, gameboy: &mut GameBoy) -> Result<u8, String> {
    match opcode {
        0x00 => gameboy.nop(),
        0x01 => gameboy.load(R16::BC, Immediate16),
        0x02 => gameboy.load(AddrOf(R16::BC), R8::A),
        0x03 => gameboy.inc_16(R16::BC),
        0x04 => gameboy.inc(R8::B),
        0x05 => gameboy.dec(R8::B),
        0x06 => gameboy.load(R8::B, Immediate8),
        // Storing into a deref u16 pointer seems to be only this instruction,
        // so TBD if it should be special cased or if there are other similar instructions
        // but not necessarily LD -- e.g. PUSH or POP?
        // 0x08 => gameboy.load(AddrOf(Immediate16), R16::SP),
        0x0a => gameboy.load(R8::A, AddrOf(R16::BC)),
        0x0b => gameboy.dec_16(R16::BC),
        0x0c => gameboy.inc(R8::C),
        0x0d => gameboy.dec(R8::C),
        0x0e => gameboy.load(R8::C, Immediate8),
        0x10 => gameboy.stop(),
        0x11 => gameboy.load(R16::DE, Immediate16),
        0x12 => gameboy.load(AddrOf(R16::DE), R8::A),
        0x13 => gameboy.inc_16(R16::DE),
        0x14 => gameboy.inc(R8::D),
        0x15 => gameboy.dec(R8::D),
        0x16 => gameboy.load(R8::D, Immediate8),
        0x1a => gameboy.load(R8::A, AddrOf(R16::DE)),
        0x1b => gameboy.dec_16(R16::DE),
        0x1c => gameboy.inc(R8::C),
        0x1d => gameboy.dec(R8::C),
        0x1e => gameboy.load(R8::E, Immediate8),
        0x21 => gameboy.load(R16::HL, Immediate16),
        // If the `Read` argument is changed to allow mutation of the underlying data,
        // it's not particularly nice to special case load and increment/decrement HL.
        // so for now, this has to suffice.
        // then it could read HL with side effects. I'm not too fond of that idea either,
        0x22 => {
            let r = gameboy.load(AddrOf(R16::HL), R8::A);
            let _ = gameboy.inc_16(R16::HL)?;
            r
        }
        0x23 => gameboy.inc_16(R16::HL),
        0x24 => gameboy.inc(R8::H),
        0x25 => gameboy.dec(R8::H),
        0x26 => gameboy.load(R8::H, Immediate8),
        0x2a => {
            let r = gameboy.load(R8::A, AddrOf(R16::HL));
            let _ = gameboy.inc_16(R16::HL)?;
            r
        }
        0x2b => gameboy.dec_16(R16::HL),
        0x2c => gameboy.inc(R8::L),
        0x2d => gameboy.dec(R8::L),
        0x2e => gameboy.load(R8::L, Immediate8),
        0x31 => gameboy.load(R16::SP, Immediate16),
        0x32 => {
            let r = gameboy.load(AddrOf(R16::HL), R8::A);
            let _ = gameboy.dec_16(R16::HL)?;
            r
        }
        0x33 => gameboy.inc_16(R16::SP),
        0x34 => gameboy.inc(AddrOf(R16::HL)),
        0x35 => gameboy.dec(AddrOf(R16::HL)),
        0x36 => gameboy.load(AddrOf(R16::HL), Immediate8),
        0x3a => {
            let r = gameboy.load(R8::A, AddrOf(R16::HL));
            let _ = gameboy.dec_16(R16::HL)?;
            r
        }
        0x3b => gameboy.dec_16(R16::SP),
        0x3c => gameboy.inc(R8::A),
        0x3d => gameboy.dec(R8::A),
        0x3e => gameboy.load(R8::A, Immediate8),
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
        0x4a => gameboy.load(R8::C, R8::D),
        0x4b => gameboy.load(R8::C, R8::E),
        0x4c => gameboy.load(R8::C, R8::H),
        0x4d => gameboy.load(R8::C, R8::L),
        0x4e => gameboy.load(R8::C, AddrOf(R16::HL)),
        0x4f => gameboy.load(R8::C, R8::A),
        0x50 => gameboy.load(R8::D, R8::B),
        0x51 => gameboy.load(R8::D, R8::C),
        0x52 => gameboy.load(R8::D, R8::D),
        0x53 => gameboy.load(R8::D, R8::E),
        0x54 => gameboy.load(R8::D, R8::H),
        0x55 => gameboy.load(R8::D, R8::L),
        0x56 => gameboy.load(R8::D, AddrOf(R16::HL)),
        0x57 => gameboy.load(R8::D, R8::A),
        0x57 => gameboy.load(R8::E, R8::A),
        0x58 => gameboy.load(R8::E, R8::B),
        0x59 => gameboy.load(R8::E, R8::C),
        0x5a => gameboy.load(R8::E, R8::D),
        0x5b => gameboy.load(R8::E, R8::E),
        0x5c => gameboy.load(R8::E, R8::H),
        0x5d => gameboy.load(R8::E, R8::L),
        0x5e => gameboy.load(R8::E, AddrOf(R16::HL)),
        0x60 => gameboy.load(R8::H, R8::B),
        0x61 => gameboy.load(R8::H, R8::C),
        0x62 => gameboy.load(R8::H, R8::D),
        0x63 => gameboy.load(R8::H, R8::E),
        0x64 => gameboy.load(R8::H, R8::H),
        0x65 => gameboy.load(R8::H, R8::L),
        0x66 => gameboy.load(R8::H, AddrOf(R16::HL)),
        0x67 => gameboy.load(R8::H, R8::A),
        0x67 => gameboy.load(R8::L, R8::A),
        0x68 => gameboy.load(R8::L, R8::B),
        0x69 => gameboy.load(R8::L, R8::C),
        0x6a => gameboy.load(R8::L, R8::D),
        0x6b => gameboy.load(R8::L, R8::E),
        0x6c => gameboy.load(R8::L, R8::H),
        0x6d => gameboy.load(R8::L, R8::L),
        0x6e => gameboy.load(R8::L, AddrOf(R16::HL)),
        0x70 => gameboy.load(AddrOf(R16::HL), R8::B),
        0x71 => gameboy.load(AddrOf(R16::HL), R8::C),
        0x72 => gameboy.load(AddrOf(R16::HL), R8::D),
        0x73 => gameboy.load(AddrOf(R16::HL), R8::E),
        0x74 => gameboy.load(AddrOf(R16::HL), R8::H),
        0x75 => gameboy.load(AddrOf(R16::HL), R8::L),
        0x76 => gameboy.halt(),
        0x77 => gameboy.load(AddrOf(R16::HL), R8::A),
        0x77 => gameboy.load(R8::A, R8::A),
        0x78 => gameboy.load(R8::A, R8::B),
        0x79 => gameboy.load(R8::A, R8::C),
        0x7a => gameboy.load(R8::A, R8::D),
        0x7b => gameboy.load(R8::A, R8::E),
        0x7c => gameboy.load(R8::A, R8::H),
        0x7d => gameboy.load(R8::A, R8::L),
        0x7e => gameboy.load(R8::A, AddrOf(R16::HL)),
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
        0x8a => gameboy.add(R8::A, R8::D, Carry::With),
        0x8b => gameboy.add(R8::A, R8::E, Carry::With),
        0x8c => gameboy.add(R8::A, R8::H, Carry::With),
        0x8d => gameboy.add(R8::A, R8::L, Carry::With),
        0x8e => gameboy.add(R8::A, AddrOf(R16::HL), Carry::With),
        0x8f => gameboy.add(R8::A, R8::A, Carry::With),
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
        0x9a => gameboy.sub(R8::A, R8::D, Carry::With),
        0x9b => gameboy.sub(R8::A, R8::E, Carry::With),
        0x9c => gameboy.sub(R8::A, R8::H, Carry::With),
        0x9d => gameboy.sub(R8::A, R8::L, Carry::With),
        0x9e => gameboy.sub(R8::A, AddrOf(R16::HL), Carry::With),
        0x9f => gameboy.sub(R8::A, R8::A, Carry::With),
        0xa0 => gameboy.and(R8::A, R8::B),
        0xa1 => gameboy.and(R8::A, R8::C),
        0xa2 => gameboy.and(R8::A, R8::D),
        0xa3 => gameboy.and(R8::A, R8::E),
        0xa4 => gameboy.and(R8::A, R8::H),
        0xa5 => gameboy.and(R8::A, R8::L),
        0xa6 => gameboy.and(R8::A, AddrOf(R16::HL)),
        0xa7 => gameboy.and(R8::A, R8::A),
        0xa8 => gameboy.xor(R8::A, R8::B),
        0xa9 => gameboy.xor(R8::A, R8::C),
        0xaa => gameboy.xor(R8::A, R8::D),
        0xab => gameboy.xor(R8::A, R8::E),
        0xac => gameboy.xor(R8::A, R8::H),
        0xad => gameboy.xor(R8::A, R8::L),
        0xae => gameboy.xor(R8::A, AddrOf(R16::HL)),
        0xaf => gameboy.xor(R8::A, R8::A),
        0xb0 => gameboy.or(R8::A, R8::B),
        0xb1 => gameboy.or(R8::A, R8::C),
        0xb2 => gameboy.or(R8::A, R8::D),
        0xb3 => gameboy.or(R8::A, R8::E),
        0xb4 => gameboy.or(R8::A, R8::H),
        0xb5 => gameboy.or(R8::A, R8::L),
        0xb6 => gameboy.or(R8::A, AddrOf(R16::HL)),
        0xb7 => gameboy.or(R8::A, R8::A),
        0xb8 => gameboy.cmp(R8::A, R8::B),
        0xb9 => gameboy.cmp(R8::A, R8::C),
        0xba => gameboy.cmp(R8::A, R8::D),
        0xbb => gameboy.cmp(R8::A, R8::E),
        0xbc => gameboy.cmp(R8::A, R8::H),
        0xbd => gameboy.cmp(R8::A, R8::L),
        0xbe => gameboy.cmp(R8::A, AddrOf(R16::HL)),
        0xbf => gameboy.cmp(R8::A, R8::A),
        0xc6 => gameboy.add(R8::A, Immediate8, Carry::Without),
        0xce => gameboy.add(R8::A, Immediate8, Carry::With),
        0xd6 => gameboy.sub(R8::A, Immediate8, Carry::Without),
        0xde => gameboy.sub(R8::A, Immediate8, Carry::With),
        0xe0 => gameboy.load(AddrOf(Immediate8), R8::A),
        0xe2 => gameboy.load(AddrOf(R8::C), R8::A),
        0xe6 => gameboy.and(R8::A, Immediate8),
        0xee => gameboy.xor(R8::A, Immediate8),
        0xea => gameboy.load(AddrOf(Immediate16), R8::A),
        0xf0 => gameboy.load(R8::A, AddrOf(Immediate8)),
        0xf2 => gameboy.load(R8::A, AddrOf(R8::C)),
        0xf6 => gameboy.or(R8::A, Immediate8),
        0xfa => gameboy.load(R8::A, AddrOf(Immediate16)),
        0xfe => gameboy.cmp(R8::A, Immediate8),
        _ => Err(format!("Unable to match opcode {:x}", opcode)),
    }
}

struct NoWrite<T>(T);
struct Constant<T>(T);
struct AddrOf<T>(T);
struct Immediate8;
struct Immediate16;

trait AsAddress {
    fn as_address(self) -> u16;
}

impl AsAddress for u16 {
    fn as_address(self) -> u16 {
        self
    }
}

impl AsAddress for u8 {
    fn as_address(self) -> u16 {
        self as u16 + 0xff
    }
}

impl<A: AsAddress, R: mem::Read<Out = A>> mem::Read for AddrOf<R> {
    type Out = u8;
    fn read(&self, gb: &GameBoy) -> Self::Out {
        gb.mmu.read_u8(self.0.read(gb).as_address())
    }
}

impl<V, T: mem::Write<In=V>> mem::Write for NoWrite<T> {
    type In = V;
    fn write(&self, _: &mut GameBoy, _: Self::In) -> Result<(), String> {
        // NoWrite causes write to be a NOOP.
        Ok(())
    }
}

impl <V, T: mem::Read<Out=V>> mem::Read for NoWrite<T> {
    type Out = V;
    fn read(&self, gb: &GameBoy) -> Self::Out {
        self.0.read(gb)
    }
}

impl<T: Copy> mem::Read for Constant<T> {
    type Out = T;
    fn read(&self, _: &GameBoy) -> Self::Out {
        self.0
    }
}

impl<A: AsAddress, R: mem::Read<Out = A>> mem::Write for AddrOf<R> {
    type In = u8;
    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), String> {
        let addr = self.0.read(gb).as_address();
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

impl mem::Read for Immediate16 {
    type Out = u16;
    fn read(&self, gb: &GameBoy) -> Self::Out {
        gb.mmu.read_u16(gb.cpu.register.pc())
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
    SP,
}

impl mem::Read for R16 {
    type Out = u16;
    fn read(&self, gb: &GameBoy) -> Self::Out {
        match self {
            AF => gb.cpu.register.af(),
            BC => gb.cpu.register.bc(),
            DE => gb.cpu.register.de(),
            HL => gb.cpu.register.hl(),
            SP => gb.cpu.register.sp(),
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
            SP => gb.cpu.register.sp(),
        };
        *reg = value;
        Ok(())
    }
}

enum Carry {
    Without,
    With,
}

trait Instructions {
    type Output;

    fn load<W, R, V>(&mut self, W, R) -> Self::Output
    where
        W: mem::Write<In = V>,
        R: mem::Read<Out = V>;

    fn binary_op<LHS, RHS, F>(&mut self, LHS, RHS, F) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
        F: FnOnce(u8, u8, RegisterF) -> (u8, RegisterF);

    fn binary_op_16<LHS, RHS, F>(&mut self, LHS, RHS, F) -> Self::Output
    where
        LHS: mem::Read<Out = u16> + mem::Write<In = u16>,
        RHS: mem::Read<Out = u16>,
        F: FnOnce(u16, u16, RegisterF) -> (u16, RegisterF);

    fn add<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS, carry: Carry) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        self.binary_op(lhs, rhs, |x, y, rf| (x + y, rf))
    }

    fn sub<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS, carry: Carry) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        self.binary_op(lhs, rhs, |x, y, rf| (x - y, rf))
    }

    fn xor<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        self.binary_op(lhs, rhs, |x, y, rf| (x ^ y, rf))
    }

    fn and<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        self.binary_op(lhs, rhs, |x, y, rf| (x & y, rf))
    }

    fn or<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        self.binary_op(lhs, rhs, |x, y, rf| (x | y, rf))
    }

    fn cmp<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        // Ensure that the result is not written to the LHS
        self.binary_op(NoWrite(lhs), rhs, |x, y, rf| (x - y, rf))
    }

    fn inc<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u8> + mem::Write<In = u8>,
    {
        self.binary_op(operand, Constant(1), |x, y, rf| (x + y, rf))
    }

    fn inc_16<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u16> + mem::Write<In = u16>,
    {
        self.binary_op_16(operand, Constant(1), |x, y, rf| (x + y, rf))
    }

    fn dec<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u8> + mem::Write<In = u8>,
    {
        self.binary_op(operand, Constant(1), |x, y, rf| (x - y, rf))
    }

    fn dec_16<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u16> + mem::Write<In = u16>,
    {
        self.binary_op_16(operand, Constant(1), |x, y, rf| (x - y, rf))
    }

    fn nop(&mut self) -> Self::Output;
    fn halt(&mut self) -> Self::Output;
    fn stop(&mut self) -> Self::Output;
}

impl Instructions for GameBoy {
    type Output = Result<u8, String>;

    fn load<W, R, V>(&mut self, to: W, from: R) -> Self::Output
    where
        W: mem::Write<In = V>,
        R: mem::Read<Out = V>,
    {
        let value: V = from.read(self);
        to.write(self, value)?;
        Ok(1)
    }

    fn binary_op<LHS, RHS, F>(&mut self, lhs: LHS, rhs: RHS, func: F) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
        F: FnOnce(u8, u8, RegisterF) -> (u8, RegisterF),
    {
        let lhs_ = lhs.read(self);
        let rhs_ = rhs.read(self);
        let (result, f) = func(lhs_, rhs_, *(self.cpu.register.f()));
        *(self.cpu.register.f()) = f;
        lhs.write(self, result)?;
        Ok(1)
    }

    fn binary_op_16<LHS, RHS, F>(&mut self, lhs: LHS, rhs: RHS, func: F) -> Self::Output
    where
        LHS: mem::Read<Out = u16> + mem::Write<In = u16>,
        RHS: mem::Read<Out = u16>,
        F: FnOnce(u16, u16, RegisterF) -> (u16, RegisterF),
    {
        let lhs_ = lhs.read(self);
        let rhs_ = rhs.read(self);
        let (result, f) = func(lhs_, rhs_, *(self.cpu.register.f()));
        *(self.cpu.register.f()) = f;
        lhs.write(self, result)?;
        Ok(2)
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
}
