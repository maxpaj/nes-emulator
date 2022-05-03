use std::fs;
use std::error::Error;
use std::fmt;

use crate::core::debug::print_bytes_table;

#[derive(Debug)]
pub struct ROM {
    pub header: Vec<u8>,
    pub trainer: Vec<u8>,
    pub program: Vec<u8>,
    pub chr: Vec<u8>,

    pub trainer_size: usize,
    pub program_size: usize,
    pub chr_size: usize,

    // Mapper, decides how ROM on the cartridge maps to the CPU and PPU memory
    // https://www.nesdev.org/wiki/Mapper
    pub mapper: u16,

    ///
    /// What is this used for?
    ///
    pub mirroring_vertical: bool,

    ///
    /// What is this used for?
    ///
    pub has_battery_backed_prg_ram: bool,

    ///
    /// What is this used for?
    ///
    pub has_trainer: bool,

    ///
    /// What is this used for?
    ///
    pub has_four_screen_vram: bool
}

#[derive(Debug)]
pub struct FormatParseError {
    details: String
}

impl FormatParseError {
    fn new(msg: &str) -> FormatParseError {
        FormatParseError {details: msg.to_string()}
    }
}

impl fmt::Display for FormatParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for FormatParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn read_rom_from_file(file_path: String) -> Result<ROM, FormatParseError> {
    let file_bytes = fs::read(file_path).expect("Unable to read file");

    println!("Reading ROM file with {} bytes", file_bytes.len());

    // 16 bytes header
    let header = file_bytes[0..0x10].to_vec();

    // header should start with "NES" followed by MS-DOS end-of-file
    if  header[0] != 0x4E ||
        header[1] != 0x45 ||
        header[2] != 0x53 ||
        header[3] != 0x1A
    {
        return Err(FormatParseError::new("Not NES format file."));
    }

    // Parse flags 6
    let flags_6 = header[6];
    let mirroring_vertical = flags_6 & 0b00000001 > 0;
    let has_battery_backed_prg_ram = flags_6 & 0b00000010 > 0;
    let has_trainer = flags_6 & 0b00000100 > 0;
    let has_four_screen_vram = flags_6 & 0b00001000 > 0;
    
    // Mapper
    let flags_7 = header[7];
    let mapper_lo = flags_6 & 0b11110000;
    let mapper_hi = flags_7 & 0b11110000;
    let mapper = (mapper_hi as u16) << 8 | (mapper_lo as u16);

    // 0 or 512 bytes trainer
    let trainer_size = if has_trainer { 0x200 } else { 0 };
    let trainer_byte_range = 0x10..(0x10 + trainer_size);
    let trainer = file_bytes[trainer_byte_range.clone()].to_vec();
    println!("Read trainer size {} bytes", trainer.len());

    // 0x4000 * program_size bytes program
    let program_size = (header[4] as u16 * 0x4000) as usize;
    let program_byte_range = trainer_byte_range.end..(&trainer_byte_range.end + program_size);
    let program = file_bytes[program_byte_range.clone()].to_vec();
    println!("Read program size {} bytes", program.len());
    print_bytes_table(&program, 0x00, 0xFF, 8);

    // 0x2000 * chr_size bytes tile map
    let chr_size = (header[5] as u16 * 0x2000) as usize;
    let chr_byte_range = program_byte_range.end..(&program_byte_range.end + chr_size);
    let chr = file_bytes[chr_byte_range.clone()].to_vec();
    println!("Read chr size {} bytes", chr.len());

    println!("ROM total headers size {} bytes", chr_size + program_size + trainer_size);

    let rom = ROM {
        header,
        chr,
        chr_size,
        program,
        program_size,
        trainer,
        trainer_size,
        mapper,
        has_battery_backed_prg_ram,
        has_four_screen_vram,
        has_trainer,
        mirroring_vertical
    };

    return Ok(rom);
}