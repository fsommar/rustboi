#[macro_use]
extern crate num_derive;

mod gameboy;

use crate::gameboy::GameBoy;

fn main() {
    let mut gb: GameBoy = Default::default();
    println!("{:#?}", gb);
    gb.run();
}
