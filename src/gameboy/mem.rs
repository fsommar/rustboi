use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub(crate) struct MMU {
    mem: Box<[u8]>
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            // Memory is 16 kB (simplified)
            mem: vec![0u8; 1 << 14].into_boxed_slice(),
        }
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read_u8_mut(&mut self, addr: u16) -> &mut u8 {
        &mut self.mem[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        self.read_u8(addr + 1) as u16 | (self.read_u8(addr) as u16) << 8
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        self.mem[addr as usize] = (value >> 8) as u8;
        self.mem[addr as usize + 1] = (value & 0xFF) as u8;
    }
}

impl Default for MMU {
    fn default() -> MMU {
        MMU::new()
    }
}

#[test]
fn test_read_u16() {
    let mmu = MMU {
        mem: vec![0x12, 0x34].into_boxed_slice(),
    };
    assert_eq!(0x12, mmu.read_u8(0));
    assert_eq!(0x34, mmu.read_u8(1));
    assert_eq!(0x1234, mmu.read_u16(0));
}

#[test]
fn test_write_u16() {
    let mut mmu = MMU {
        mem: vec![0, 0].into_boxed_slice(),
    };
    mmu.write_u16(0, 0x1234);
    assert_eq!(0x12, mmu.read_u8(0));
    assert_eq!(0x34, mmu.read_u8(1));
}

pub(crate) trait Read {
    type Out;
    fn read(&self, &mut super::GameBoy) -> Self::Out;
}

pub(crate) trait Write {
    type In;
    fn write(&self, &mut super::GameBoy, value: Self::In) -> Result<(), String>;
}