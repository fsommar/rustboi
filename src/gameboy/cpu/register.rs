use std::fmt::Formatter;
use std::{
    self,
    ops::{Deref, DerefMut},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub(crate) struct Register {
    pub(crate) af: RegisterPair,
    pub(crate) bc: RegisterPair,
    pub(crate) de: RegisterPair,
    pub(crate) hl: RegisterPair,
    pub(crate) sp: StackPointer,
    pub(crate) pc: ProgramCounter,
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

impl Deref for StackPointer {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.sp
    }
}

impl DerefMut for StackPointer {
    fn deref_mut(&mut self) -> &mut u16 {
        &mut self.sp
    }
}

impl Deref for RegisterPair {
    type Target = u16;

    fn deref(&self) -> &u16 {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            &*(self as *const RegisterPair as *const u16)
        }
    }
}

impl DerefMut for RegisterPair {
    fn deref_mut(&mut self) -> &mut u16 {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            &mut *(self as *mut RegisterPair as *mut u16)
        }
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
        self.af.hi
    }
    fn b(self) -> Self::Output {
        self.bc.hi
    }
    fn c(self) -> Self::Output {
        self.bc.lo
    }
    fn d(self) -> Self::Output {
        self.de.hi
    }
    fn e(self) -> Self::Output {
        self.de.lo
    }
    fn h(self) -> Self::Output {
        self.hl.hi
    }
    fn l(self) -> Self::Output {
        self.hl.lo
    }
}

impl<'a> Register8 for &'a mut Register {
    type Output = &'a mut u8;
    fn a(self) -> Self::Output {
        &mut self.af.hi
    }
    fn b(self) -> Self::Output {
        &mut self.bc.hi
    }
    fn c(self) -> Self::Output {
        &mut self.bc.lo
    }
    fn d(self) -> Self::Output {
        &mut self.de.hi
    }
    fn e(self) -> Self::Output {
        &mut self.de.lo
    }
    fn h(self) -> Self::Output {
        &mut self.hl.hi
    }
    fn l(self) -> Self::Output {
        &mut self.hl.lo
    }
}

impl Register {
    pub(crate) fn f(&mut self) -> &mut RegisterF {
        unsafe { &mut *(&mut self.af.lo as *mut u8 as *mut RegisterF) }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"
    A ${:02X} F ${:02X} AF ${:04X}
    B ${:02X} C ${:02X} BC ${:04X}
    D ${:02X} E ${:02X} DE ${:04X}
    H ${:02X} L ${:02X} HL ${:04X}
    SP ${:04X}
    PC ${:04X}
        "#,
            self.af.hi,
            self.af.lo,
            *self.af,
            self.bc.hi,
            self.bc.lo,
            *self.bc,
            self.de.hi,
            self.de.lo,
            *self.de,
            self.hl.hi,
            self.hl.lo,
            *self.hl,
            *self.sp,
            *self.pc,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::flag;
    use super::*;

    #[test]
    fn test_as_u16() {
        let rr = RegisterPair {
            lo: 0b0000_0111_u8,
            hi: 0b1111_0000_u8,
        };
        assert_eq!(0b1111_0000_0000_0111_u16, *rr);
    }

    #[test]
    fn test_as_u16_mut() {
        let mut rr = RegisterPair {
            lo: 0b0000_0111_u8,
            hi: 0b1111_0000_u8,
        };
        *rr = 0b0000_0000_0000_1111;
        assert_eq!(
            RegisterPair {
                lo: 0b0000_1111,
                hi: 0,
            },
            rr
        );
    }

    #[test]
    fn test_lo_hi_mut() {
        let mut rr: RegisterPair = Default::default();
        rr.lo = 5;
        assert_eq!(rr, RegisterPair { lo: 5, hi: 0 });
        rr.hi = 10;
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
}
