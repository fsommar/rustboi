extern crate num_traits;

use self::num_traits::{FromPrimitive, ToPrimitive};

use self::cpu::*;
use super::*;

pub(crate) fn execute(opcode: u8, gameboy: &mut GameBoy) -> Result<u8, String> {
    match opcode {
        0x00 => gameboy.nop(),
        0x01 => gameboy.load(R16::BC, Immediate16),
        0x02 => gameboy.load(AddrOf(R16::BC), R8::A),
        0x03 => gameboy.inc16(R16::BC),
        0x04 => gameboy.inc(R8::B),
        0x05 => gameboy.dec(R8::B),
        0x06 => gameboy.load(R8::B, Immediate8),
        // Storing into a deref u16 pointer seems to be only this instruction,
        // so TBD if it should be special cased or if there are other similar instructions
        // but not necessarily LD -- e.g. PUSH or POP?
        // 0x08 => gameboy.load(AddrOf(Immediate16), R16::SP),
        0x09 => gameboy.add16(R16::HL, R16::BC, Carry::Without),
        0x0A => gameboy.load(R8::A, AddrOf(R16::BC)),
        0x0B => gameboy.dec16(R16::BC),
        0x0C => gameboy.inc(R8::C),
        0x0D => gameboy.dec(R8::C),
        0x0E => gameboy.load(R8::C, Immediate8),
        0x10 => gameboy.stop(),
        0x11 => gameboy.load(R16::DE, Immediate16),
        0x12 => gameboy.load(AddrOf(R16::DE), R8::A),
        0x13 => gameboy.inc16(R16::DE),
        0x14 => gameboy.inc(R8::D),
        0x15 => gameboy.dec(R8::D),
        0x16 => gameboy.load(R8::D, Immediate8),
        0x18 => gameboy.jump(Flags::Always, Immediate8),
        0x19 => gameboy.add16(R16::HL, R16::DE, Carry::Without),
        0x1A => gameboy.load(R8::A, AddrOf(R16::DE)),
        0x1B => gameboy.dec16(R16::DE),
        0x1C => gameboy.inc(R8::C),
        0x1D => gameboy.dec(R8::C),
        0x1E => gameboy.load(R8::E, Immediate8),
        0x20 => gameboy.jump(Flags::NZ, Immediate8),
        0x21 => gameboy.load(R16::HL, Immediate16),
        0x22 => gameboy.load(AddrOf(PostInc(R16::HL)), R8::A),
        0x23 => gameboy.inc16(R16::HL),
        0x24 => gameboy.inc(R8::H),
        0x25 => gameboy.dec(R8::H),
        0x26 => gameboy.load(R8::H, Immediate8),
        0x28 => gameboy.jump(Flags::Z, Immediate8),
        0x29 => gameboy.add16(R16::HL, R16::HL, Carry::Without),
        0x2A => gameboy.load(R8::A, AddrOf(PostInc(R16::HL))),
        0x2B => gameboy.dec16(R16::HL),
        0x2C => gameboy.inc(R8::L),
        0x2D => gameboy.dec(R8::L),
        0x2E => gameboy.load(R8::L, Immediate8),
        0x30 => gameboy.jump(Flags::NC, Immediate8),
        0x31 => gameboy.load(R16::SP, Immediate16),
        0x32 => gameboy.load(AddrOf(PostDec(R16::HL)), R8::A),
        0x33 => gameboy.inc16(R16::SP),
        0x34 => gameboy.inc(AddrOf(R16::HL)),
        0x35 => gameboy.dec(AddrOf(R16::HL)),
        0x36 => gameboy.load(AddrOf(R16::HL), Immediate8),
        0x38 => gameboy.jump(Flags::C, Immediate8),
        0x39 => gameboy.add16(R16::HL, R16::SP, Carry::Without),
        0x3A => gameboy.load(R8::A, AddrOf(PostDec(R16::HL))),
        0x3B => gameboy.dec16(R16::SP),
        0x3C => gameboy.inc(R8::A),
        0x3D => gameboy.dec(R8::A),
        0x3E => gameboy.load(R8::A, Immediate8),
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
        0xC2 => gameboy.jump(Flags::NZ, Immediate16),
        0xC3 => gameboy.jump(Flags::Always, Immediate16),
        0xC6 => gameboy.add(R8::A, Immediate8, Carry::Without),
        0xCA => gameboy.jump(Flags::Z, Immediate16),
        0xCE => gameboy.add(R8::A, Immediate8, Carry::With),
        0xD2 => gameboy.jump(Flags::NC, Immediate16),
        0xD6 => gameboy.sub(R8::A, Immediate8, Carry::Without),
        0xDA => gameboy.jump(Flags::C, Immediate16),
        0xDE => gameboy.sub(R8::A, Immediate8, Carry::With),
        0xE0 => gameboy.load(AddrOf(Immediate8), R8::A),
        0xE2 => gameboy.load(AddrOf(R8::C), R8::A),
        0xE6 => gameboy.and(R8::A, Immediate8),
        // TODO: Create separate method for this, since its behavior is unique
        // 0xE8 => gameboy.add(R16::SP, Immediate8, Carry::Without),
        0xEE => gameboy.xor(R8::A, Immediate8),
        0xEA => gameboy.load(AddrOf(Immediate16), R8::A),
        0xF0 => gameboy.load(R8::A, AddrOf(Immediate8)),
        0xF2 => gameboy.load(R8::A, AddrOf(R8::C)),
        0xF3 => gameboy.set_interrupt(Interrupt::Disable),
        0xF6 => gameboy.or(R8::A, Immediate8),
        0xF8 => gameboy.load(R16::HL, Plus(R16::SP, Immediate8)),
        0xF9 => gameboy.load(R16::SP, R16::HL),
        0xFA => gameboy.load(R8::A, AddrOf(Immediate16)),
        0xFB => gameboy.set_interrupt(Interrupt::Enable),
        0xFE => gameboy.cmp(R8::A, Immediate8),
        _ => Err(format!("Unable to match opcode {:x}", opcode)),
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

impl<T, U, UNum, Num> mem::Read for Plus<T, U>
where
    T: mem::Read<Out = Num>,
    U: mem::Read<Out = UNum>,
    UNum: Into<Num>,
    Num: num_traits::PrimInt + num_traits::Unsigned,
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
    T: num_traits::PrimInt + num_traits::Unsigned,
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
    T: num_traits::PrimInt + num_traits::Unsigned,
    W: mem::Write<In = T> + mem::Read<Out = T>,
{
    type Out = T;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        let value = self.0.read(gb);
        self.0
            .write(gb, value - T::one())
            .expect("post dec write must not fail");
        value
    }
}

impl<A: AsAddress, R: mem::Read<Out = A>> mem::Read for AddrOf<R> {
    type Out = u8;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        let addr = self.0.read(gb).as_address();
        gb.mmu.read_u8(addr)
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

impl<V, T: mem::Write<In = V>> mem::Write for NoWrite<T> {
    type In = V;
    fn write(&self, _: &mut GameBoy, _: Self::In) -> Result<(), String> {
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
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        gb.mmu.read_u8(gb.cpu.register.pc())
    }
}

impl mem::Read for Immediate16 {
    type Out = u16;
    fn read(&self, gb: &mut GameBoy) -> Self::Out {
        gb.mmu.read_u16(gb.cpu.register.pc())
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
    fn write(&self, gb: &mut GameBoy, value: Self::In) -> Result<(), String> {
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
        use self::R16::*;
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

#[derive(FromPrimitive, ToPrimitive)]
enum Carry {
    Without,
    With,
}

enum Interrupt {
    Enable,
    Disable,
}

trait Instructions {
    type Output;

    fn load<W, R, V>(&mut self, W, R) -> Self::Output
    where
        W: mem::Write<In = V>,
        R: mem::Read<Out = V>;

    fn binary_op<LHS, RHS, F, Num>(&mut self, LHS, RHS, F) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        F: FnOnce(Num, Num, RegisterF) -> (Num, RegisterF),
        Num: num_traits::PrimInt + num_traits::Unsigned;

    fn jump<Offset, V>(&mut self, Flags, Offset) -> Self::Output
    where
        V: Into<u16>,
        Offset: mem::Read<Out = V>;

    fn set_interrupt(&mut self, Interrupt) -> Self::Output;

    // TODO: Combine add and add16 (?)
    fn add<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS, carry: Carry) -> Self::Output
    where
        LHS: mem::Read<Out = u8> + mem::Write<In = u8>,
        RHS: mem::Read<Out = u8>,
    {
        let carry = carry.to_u8().unwrap();
        self.binary_op(lhs, rhs, |x, y, mut f| {
            // TODO: Figure out overflow in a nicer way.
            let (res, overflow1) = x.overflowing_add(y);
            // FIXME: Is carry included in the overflow calculation?
            let (res, overflow2) = res.overflowing_add(carry);
            f[flag::Z].set_bool(res == 0);
            f[flag::N].reset();
            f[flag::H].set_bool((x ^ y ^ res) & 0x10 != 0);
            f[flag::C].set_bool(overflow1 || overflow2);
            (res, f)
        })
    }

    fn add16<LHS, RHS>(&mut self, lhs: LHS, rhs: RHS, carry: Carry) -> Self::Output
    where
        LHS: mem::Read<Out = u16> + mem::Write<In = u16>,
        RHS: mem::Read<Out = u16>,
    {
        let carry = carry.to_u16().unwrap();
        self.binary_op(lhs, rhs, |x, y, mut f| {
            // TODO: Figure out overflow in a nicer way.
            let (res, overflow1) = x.overflowing_add(y);
            // FIXME: Is carry included in the overflow calculation?
            let (res, overflow2) = res.overflowing_add(carry);
            f[flag::N].reset();
            f[flag::H].set_bool((x ^ y ^ res) & 0x100 != 0);
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
            // TODO: Figure out overflow in a nicer way.
            let (res, overflow1) = x.overflowing_sub(y);
            // FIXME: Is carry included in the overflow calculation?
            let (res, overflow2) = res.overflowing_sub(carry);
            f[flag::Z].set_bool(res == 0);
            f[flag::N].set();
            f[flag::H].set_bool((x ^ y ^ res) & 0x10 != 0);
            f[flag::C].set_bool(overflow1 || overflow2);
            (res, f)
        })
    }

    fn xor<LHS, RHS, Num>(&mut self, lhs: LHS, rhs: RHS) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        Num: num_traits::PrimInt + num_traits::Unsigned,
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
        Num: num_traits::PrimInt + num_traits::Unsigned,
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
        Num: num_traits::PrimInt + num_traits::Unsigned,
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
            f[flag::H].set_bool((x ^ y ^ res) & 0x10 != 0);
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
            f[flag::H].set_bool((x ^ y ^ res) & 0x10 != 0);
            (res, f)
        })
    }

    fn dec16<T>(&mut self, lhs: T) -> Self::Output
    where
        T: mem::Read<Out = u16> + mem::Write<In = u16>,
    {
        self.binary_op(lhs, Constant(1), |x, y, f| (x - y, f))
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

    fn binary_op<LHS, RHS, F, Num>(&mut self, lhs: LHS, rhs: RHS, op: F) -> Self::Output
    where
        LHS: mem::Read<Out = Num> + mem::Write<In = Num>,
        RHS: mem::Read<Out = Num>,
        F: FnOnce(Num, Num, RegisterF) -> (Num, RegisterF),
        Num: num_traits::PrimInt + num_traits::Unsigned,
    {
        let lhs_ = lhs.read(self);
        let rhs_ = rhs.read(self);
        let (result, f) = op(lhs_, rhs_, *(self.cpu.register.f()));
        *(self.cpu.register.f()) = f;
        lhs.write(self, result)?;
        Ok(1)
    }

    fn jump<Offset, V>(&mut self, flags: Flags, offset: Offset) -> Self::Output
    where
        V: Into<u16>,
        Offset: mem::Read<Out = V>,
    {
        use self::Flags::*;
        let offset_ = offset.read(self);
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
            *(&mut self.cpu.register.pc()) += offset_.into();
            Ok(3)
        } else {
            Ok(2)
        }
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

    fn set_interrupt(&mut self, _: Interrupt) -> Self::Output {
        unimplemented!()
    }
}
