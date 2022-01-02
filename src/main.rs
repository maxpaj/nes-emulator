mod core;

use crate::core::{rom,debug,cpu};

fn main() {
    let r = rom::read_rom_from_file("./super_mario.nes".to_string()).expect("Could not read ROM");
    debug::print_bytes_table(r.program.clone(), 0x00, 0x0f, 8);
    let m = cpu::run(r.program.clone());
    debug::print_bytes_table(r.program.clone(), 0x00, 0x0f, 8);
}
