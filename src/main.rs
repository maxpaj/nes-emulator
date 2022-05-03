mod core;
mod libs;

use crate::core::{rom,cpu};

fn main() {
    let rom = rom::read_rom_from_file("./roms/nestest.nes".to_string()).expect("Could not read ROM");

    let mut c = cpu::CPU::new();
    c.run(rom);
}
