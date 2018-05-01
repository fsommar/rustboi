mod cpu;
mod instr;
mod mmu;

use self::instr::Instruction;
use self::cpu::*;
use std::{convert::TryInto,
          ops::{Index, IndexMut}};

#[derive(Debug, Default)]
pub(crate) struct GameBoy {
    cpu: cpu::CPU,
    mmu: mmu::MMU,
}

pub(crate) trait Ctx {
    fn reg(&self) -> &cpu::Register;
    fn reg_mut(&mut self) -> &mut cpu::Register;
    fn read_u8_mut(&mut self, addr: u16) -> &mut u8;
    fn read_u8(&self, addr: u16) -> u8;
    fn read_u8_imm(&self, pos: u8) -> u8;
    fn read_u16(&self, addr: u16) -> u16;
    fn read_u16_imm(&self) -> u16;
}

// TODO: Figure out a way to index a trait object without having to add a read_u8 -> &u8 method.
// impl<'a> Index<u16> for Ctx + 'a {
//     type Output = u8;

//     fn index(&self, addr: u16) -> &Self::Output {
//         &self.read_u8(addr)
//     }
// }

// impl<'a> IndexMut<u16> for Ctx + 'a {
//     fn index_mut(&mut self, addr: u16) -> &mut Self::Output {
//         self.read_u8_mut(addr)
//     }
// }

impl Ctx for GameBoy {
    fn reg_mut(&mut self) -> &mut cpu::Register {
        &mut self.cpu.register
    }

    fn reg(&self) -> &cpu::Register {
        &self.cpu.register
    }

    fn read_u8_mut(&mut self, addr: u16) -> &mut u8 {
        self.mmu.read_u8_mut(addr)
    }

    fn read_u8(&self, addr: u16) -> u8 {
        self.mmu.read_u8(addr)
    }

    fn read_u8_imm(&self, pos: u8) -> u8 {
        let addr = self.reg().pc() + pos as u16;
        self.mmu.read_u8(addr)
    }

    fn read_u16(&self, addr: u16) -> u16 {
        self.mmu.read_u16(addr)
    }

    fn read_u16_imm(&self) -> u16 {
        let addr = self.reg().pc();
        self.mmu.read_u16(addr)
    }

}

impl GameBoy {
    pub fn run(&mut self) {
        while let Ok(instr) = self.fetch() {
            self.dispatch(instr);
        }
    }

    fn fetch(&mut self) -> Result<Instruction, String> {
        self.mmu.read_u8(self.cpu.register.pc()).try_into()
    }

    fn dispatch(&mut self, instr: Instruction) {}
}
