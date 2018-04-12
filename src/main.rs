mod gameboy;

use gameboy::GameBoy;

fn main() {
    let gb: GameBoy = Default::default();
    println!("{:#?}", gb);
}
