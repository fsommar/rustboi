use std::{self, ops::{Deref, DerefMut}};

use super::flag;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(super) struct Register {
    af: RegisterPair,
    bc: RegisterPair,
    de: RegisterPair,
    hl: RegisterPair,
    sp: StackPointer,
    pc: ProgramCounter,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(super) struct RegisterPair {
    lo: u8,
    hi: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(super) struct RegisterF {
    pub(super) f: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(super) struct StackPointer {
    sp: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(super) struct ProgramCounter {
    pc: u16
}

impl Deref for RegisterPair {
    type Target = u16;

    fn deref(&self) -> &u16 {
        self.as_u16()
    }
}

impl DerefMut for RegisterPair {
    fn deref_mut(&mut self) -> &mut u16 {
        self.as_u16_mut()
    }
}

impl Register {
    pub fn a(&mut self) -> &mut u8 {
        self.af.hi_mut()
    }
    
    pub fn f(&mut self) -> &mut RegisterF {
        unsafe { std::mem::transmute(self.af.lo_mut()) }
    }

    pub fn af(&mut self) -> &mut RegisterPair {
        &mut self.af
    }

    pub fn b(&mut self) -> &mut u8 {
        self.bc.hi_mut()
    }
    
    pub fn c(&mut self) -> &mut u8 {
        self.bc.lo_mut()
    }

    pub fn bc(&mut self) -> &mut RegisterPair {
        &mut self.bc
    }

    pub fn d(&mut self) -> &mut u8 {
        self.de.hi_mut()
    }
    
    pub fn e(&mut self) -> &mut u8 {
        self.de.lo_mut()
    }

    pub fn de(&mut self) -> &mut RegisterPair {
        &mut self.de
    }

    pub fn h(&mut self) -> &mut u8 {
        self.hl.hi_mut()
    }
    
    pub fn l(&mut self) -> &mut u8 {
        self.hl.lo_mut()
    }

    pub fn hl(&mut self) -> &mut RegisterPair {
        &mut self.hl
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

    fn as_u16(&self) -> &u16 {
        unsafe { std::mem::transmute::<&Self, &u16>(self) }
    }

    fn as_u16_mut(&mut self) -> &mut u16 {
        unsafe { std::mem::transmute::<&mut Self, &mut u16>(self) }
    }
}

#[test]
fn test_as_u16() {
    let rr = RegisterPair { lo: 0b0000_0111_u8, hi: 0b1111_0000_u8 };
    assert_eq!(0b1111_0000_0000_0111_u16, *rr.as_u16());
}

#[test]
fn test_as_u16_mut() {
    let mut rr = RegisterPair { lo: 0b0000_0111_u8, hi: 0b1111_0000_u8 };
    *rr = 0b0000_0000_0000_1111;
    assert_eq!(RegisterPair { lo: 0b0000_1111, hi: 0 }, rr);
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
