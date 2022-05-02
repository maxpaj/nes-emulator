mod core;
mod libs;

use crate::core::{rom,debug,cpu};

fn main() {
    let r = rom::read_rom_from_file("./super_mario.nes".to_string()).expect("Could not read ROM");
    debug::print_bytes_table(r.program.clone(), 0x00, 0x0f, 8);

    let mut c = cpu::CPU::new();
    let mut ram = vec![0, 0xFF];

    c.execute_one(r.program, &mut ram);
    
    debug::print_bytes_table(ram, 0x00, 0x0f, 8);
}
