use std::{
    self,
    ops::{Deref, DerefMut},
};

#[cfg(test)]
use super::flag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct Register {
    af: RegisterPair,
    bc: RegisterPair,
    de: RegisterPair,
    hl: RegisterPair,
    sp: StackPointer,
    pc: ProgramCounter,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct RegisterPair {
    lo: u8,
    hi: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct RegisterF {
    pub(super) f: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct StackPointer {
    sp: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct ProgramCounter {
    pc: u16,
}

impl Deref for ProgramCounter {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.pc
    }
}

impl DerefMut for ProgramCounter {
    fn deref_mut(&mut self) -> &mut u16 {
        &mut self.pc
    }
}

pub trait Register8 {
    type Output;
    fn a(self) -> Self::Output;
    fn b(self) -> Self::Output;
    fn c(self) -> Self::Output;
    fn d(self) -> Self::Output;
    fn e(self) -> Self::Output;
    fn h(self) -> Self::Output;
    fn l(self) -> Self::Output;
}

impl Register8 for Register {
    type Output = u8;
    fn a(self) -> Self::Output {
        self.af.hi()
    }
    fn b(self) -> Self::Output {
        self.bc.hi()
    }
    fn c(self) -> Self::Output {
        self.bc.lo()
    }
    fn d(self) -> Self::Output {
        self.de.hi()
    }
    fn e(self) -> Self::Output {
        self.de.lo()
    }
    fn h(self) -> Self::Output {
        self.hl.hi()
    }
    fn l(self) -> Self::Output {
        self.hl.lo()
    }
}

impl<'a> Register8 for &'a mut Register {
    type Output = &'a mut u8;
    fn a(self) -> Self::Output {
        self.af.hi_mut()
    }
    fn b(self) -> Self::Output {
        self.bc.hi_mut()
    }
    fn c(self) -> Self::Output {
        self.bc.lo_mut()
    }
    fn d(self) -> Self::Output {
        self.de.hi_mut()
    }
    fn e(self) -> Self::Output {
        self.de.lo_mut()
    }
    fn h(self) -> Self::Output {
        self.hl.hi_mut()
    }
    fn l(self) -> Self::Output {
        self.hl.lo_mut()
    }
}

pub trait Register16 {
    type Output;
    fn af(self) -> Self::Output;
    fn bc(self) -> Self::Output;
    fn de(self) -> Self::Output;
    fn hl(self) -> Self::Output;
    fn sp(self) -> Self::Output;
}

impl Register16 for Register {
    type Output = u16;
    fn af(self) -> Self::Output {
        self.af.as_u16()
    }
    fn bc(self) -> Self::Output {
        self.bc.as_u16()
    }
    fn de(self) -> Self::Output {
        self.de.as_u16()
    }
    fn hl(self) -> Self::Output {
        self.hl.as_u16()
    }
    fn sp(self) -> Self::Output {
        self.sp.sp
    }
}

impl<'a> Register16 for &'a mut Register {
    type Output = &'a mut u16;
    fn af(self) -> Self::Output {
        self.af.as_u16_mut()
    }
    fn bc(self) -> Self::Output {
        self.bc.as_u16_mut()
    }
    fn de(self) -> Self::Output {
        self.de.as_u16_mut()
    }
    fn hl(self) -> Self::Output {
        self.hl.as_u16_mut()
    }
    fn sp(self) -> Self::Output {
        &mut self.sp.sp
    }
}

pub trait RegisterPc {
    type Output;
    fn pc(self) -> Self::Output;
}

impl RegisterPc for Register {
    type Output = u16;
    fn pc(self) -> Self::Output {
        self.pc.pc
    }
}

impl<'a> RegisterPc for &'a mut Register {
    type Output = &'a mut u16;
    fn pc(self) -> Self::Output {
        &mut self.pc.pc
    }
}

impl Register {
    pub(crate) fn f(&mut self) -> &mut RegisterF {
        unsafe { std::mem::transmute(self.af.lo_mut()) }
    }
}

impl RegisterPair {
    fn lo(&self) -> u8 {
        self.lo
    }

    fn lo_mut(&mut self) -> &mut u8 {
        &mut self.lo
    }

    fn hi_mut(&mut self) -> &mut u8 {
        &mut self.hi
    }

    fn hi(&self) -> u8 {
        self.hi
    }

    pub(crate) fn as_u16(&self) -> u16 {
        *unsafe { std::mem::transmute::<&Self, &u16>(self) }
    }

    pub(crate) fn as_u16_mut(&mut self) -> &mut u16 {
        unsafe { std::mem::transmute::<&mut Self, &mut u16>(self) }
    }
}

#[test]
fn test_as_u16() {
    let rr = RegisterPair {
        lo: 0b0000_0111_u8,
        hi: 0b1111_0000_u8,
    };
    assert_eq!(0b1111_0000_0000_0111_u16, rr.as_u16());
}

#[test]
fn test_as_u16_mut() {
    let mut rr = RegisterPair {
        lo: 0b0000_0111_u8,
        hi: 0b1111_0000_u8,
    };
    *rr.as_u16_mut() = 0b0000_0000_0000_1111;
    assert_eq!(
        RegisterPair {
            lo: 0b0000_1111,
            hi: 0
        },
        rr
    );
}

#[test]
fn test_lo_hi_mut() {
    let mut rr: RegisterPair = Default::default();
    *rr.lo_mut() = 5;
    assert_eq!(rr, RegisterPair { lo: 5, hi: 0 });
    *rr.hi_mut() = 10;
    assert_eq!(rr, RegisterPair { lo: 5, hi: 10 });
}

#[test]
fn test_struct_sizes() {
    assert_eq!(1, std::mem::size_of::<RegisterF>());
    assert_eq!(2, std::mem::size_of::<StackPointer>());
    assert_eq!(2, std::mem::size_of::<ProgramCounter>());
    assert_eq!(2, std::mem::size_of::<RegisterPair>());
    assert_eq!(12, std::mem::size_of::<Register>());
}

#[test]
fn test_get_f_register() {
    let mut register: Register = Default::default();
    assert_eq!(false, register.f()[flag::C].into());
    assert_eq!(false, register.f()[flag::Z].into());
    assert_eq!(false, register.f()[flag::H].into());
    register.f()[flag::Z].toggle();
    register.f()[flag::C].set();
    assert_eq!(true, register.f()[flag::C].into());
    assert_eq!(true, register.f()[flag::Z].into());
    assert_eq!(false, register.f()[flag::H].into());
    register.f()[flag::Z].toggle();
    register.f()[flag::C].reset();
    assert_eq!(false, register.f()[flag::C].into());
    assert_eq!(false, register.f()[flag::Z].into());
    assert_eq!(false, register.f()[flag::H].into());
}
