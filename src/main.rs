use crate::gameboy::GameBoy;

mod gameboy;

fn main() {
    let mut gb: GameBoy = Default::default();
    let boot_rom: &[u8; 256] = include_bytes!("DMG_ROM.bin");
    gb.mmu.mem[0..256].copy_from_slice(boot_rom);
    for b in &gb.mmu.mem[..] {
        print!("{:x?}", b);
    }
    gb.run();
}
