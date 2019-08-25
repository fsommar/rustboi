use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub(crate) struct MMU {
    pub(crate) mem: Box<[u8]>,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            // Memory is 16 kB (simplified)
            mem: vec![0u8; 1 << 16].into_boxed_slice(),
        }
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        if addr as usize >= self.mem.len() {
            println!(
                "WARNING! Ignoring read from outside memory area 0x{:02X?}",
                addr
            );
        } else {
            self.mem[addr as usize] = value;
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        if addr as usize >= self.mem.len() {
            println!(
                "WARNING! Ignoring read from outside memory area 0x{:02X?}",
                addr
            );
            0
        } else {
            self.mem[addr as usize]
        }
    }

    pub fn read_u8_mut(&mut self, addr: u16) -> &mut u8 {
        &mut self.mem[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        if addr as usize >= self.mem.len() {
            println!(
                "WARNING! Ignoring read from outside memory area 0x{:04X?}",
                addr
            );
            0
        } else {
            LittleEndian::read_u16(&self.mem[addr as usize..=(addr + 1) as usize])
        }
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        LittleEndian::write_u16(&mut self.mem[addr as usize..=(addr + 1) as usize], value);
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
    const CYCLES: u8 = 0;
    fn read(&self, _: &mut super::GameBoy) -> Self::Out;
}

pub(crate) trait Write {
    type In;
    const CYCLES: u8 = 1;
    fn write(&self, _: &mut super::GameBoy, value: Self::In) -> Result<(), failure::Error>;
}
