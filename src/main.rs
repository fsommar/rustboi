use crate::gameboy::GameBoy;

mod gameboy;

fn main() {
    let mut gb: GameBoy = Default::default();
    println!("{:#?}", gb);
    gb.run();
}
