#[macro_use]
extern crate num_derive;

mod gameboy;

use gameboy::GameBoy;

fn main() {
    let mut gb: GameBoy = Default::default();
    println!("{:#?}", gb);
    gb.run();
}
