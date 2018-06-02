use super::*;
use std::convert::{From, TryFrom};

pub(crate) fn execute(opcode: u8, gameboy: &mut GameBoy) -> Result<u8, String> {
    match opcode {
        0x00 => gameboy.nop(),
        0x01 => gameboy.load(R16::BC, Immediate16),
        0x02 => gameboy.load(AddrOf(R16::BC), R8::A),
        0x06 => gameboy.load(R8::B, Immediate8),
        // Storing into a deref u16 pointer seems to be only this instruction,
        // so TBD if it should be special cased or if there are other similar instructions
        // but not necessarily LD -- e.g. PUSH or POP?
        // 0x08 => gameboy.load(AddrOf(Immediate16), R16::SP),
        0x0a => gameboy.load(R8::A, AddrOf(R16::BC)),
        0x0e => gameboy.load(R8::C, Immediate8),
        0x11 => gameboy.load(R16::DE, Immediate16),
        0x12 => gameboy.load(AddrOf(R16::DE), R8::A),
        0x16 => gameboy.load(R8::D, Immediate8),
        0x1a => gameboy.load(R8::A, AddrOf(R16::DE)),
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
        0x26 => gameboy.load(R8::H, Immediate8),
        0x2a => {
            let r = gameboy.load(R8::A, AddrOf(R16::HL));
            let _ = gameboy.inc_16(R16::HL)?;
            r
        }
        0x2e => gameboy.load(R8::L, Immediate8),
        0x31 => gameboy.load(R16::SP, Immediate16),
        0x32 => {
            let r = gameboy.load(AddrOf(R16::HL), R8::A);
            let _ = gameboy.dec_16(R16::HL)?;
            r
        }
        0x36 => gameboy.load(AddrOf(R16::HL), Immediate8),
        0x3a => {
            let r = gameboy.load(R8::A, AddrOf(R16::HL));
            let _ = gameboy.dec_16(R16::HL)?;
            r
        }
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
        0xe0 => gameboy.load(AddrOf(Immediate8), R8::A),
        0xe2 => gameboy.load(AddrOf(R8::C), R8::A),
        0xea => gameboy.load(AddrOf(Immediate16), R8::A),
        0xf0 => gameboy.load(R8::A, AddrOf(Immediate8)),
        0xf2 => gameboy.load(R8::A, AddrOf(R8::C)),
        0xfa => gameboy.load(R8::A, AddrOf(Immediate16)),
        _ => Err(format!("Unable to match opcode {:x}", opcode)),
    }
}

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

    fn unary_op<Operand, F>(&mut self, Operand, F) -> Self::Output
    where
        Operand: mem::Read<Out = u8> + mem::Write<In = u8>,
        F: FnOnce(u8, RegisterF) -> (u8, RegisterF);

    fn unary_op_16<Operand, F>(&mut self, Operand, F) -> Self::Output
    where
        Operand: mem::Read<Out = u16> + mem::Write<In = u16>,
        F: FnOnce(u16) -> u16;

    fn add<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        self.binary_op(lhs, rhs, |x, y, rf| (x + y, rf))
    }

    fn sub<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        self.binary_op(lhs, rhs, |x, y, rf| (x - y, rf))
    }

    fn inc<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u8> + mem::Write<In = u8>,
    {
        self.unary_op(operand, |x, rf| (x + 1, rf))
    }

    fn inc_16<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u16> + mem::Write<In = u16>,
    {
        self.unary_op_16(operand, |x| x + 1)
    }

    fn dec<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u8> + mem::Write<In = u8>,
    {
        self.unary_op(operand, |x, rf| (x - 1, rf))
    }

    fn dec_16<T>(&mut self, operand: T) -> Self::Output
    where
        T: mem::Read<Out = u16> + mem::Write<In = u16>,
    {
        self.unary_op_16(operand, |x| x - 1)
    }

    fn nop(&mut self) -> Self::Output;
    fn halt(&mut self) -> Self::Output;
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

    fn unary_op<Operand, F>(&mut self, operand: Operand, func: F) -> Self::Output
    where
        Operand: mem::Read<Out = u8> + mem::Write<In = u8>,
        F: FnOnce(u8, RegisterF) -> (u8, RegisterF),
    {
        let value = operand.read(self);
        let (result, f) = func(value, *(self.cpu.register.f()));
        *(self.cpu.register.f()) = f;
        operand.write(self, result)?;
        Ok(1)
    }

    fn unary_op_16<Operand, F>(&mut self, operand: Operand, func: F) -> Self::Output
    where
        Operand: mem::Read<Out = u16> + mem::Write<In = u16>,
        F: FnOnce(u16) -> u16,
    {
        let value = operand.read(self);
        let result = func(value);
        operand.write(self, result)?;
        Ok(1)
    }

    fn nop(&mut self) -> Self::Output {
        Ok(1)
    }

    fn halt(&mut self) -> Self::Output {
        unimplemented!()
    }
}
