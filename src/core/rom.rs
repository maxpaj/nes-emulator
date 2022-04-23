use std::fs;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ROM {
    pub header: Vec<u8>,
    pub trainer: Vec<u8>,
    pub program: Vec<u8>,
    pub chr: Vec<u8>,
    pub mirroring_vertical: bool,
    pub has_battery_backed_prg_ram: bool,
    pub has_trainer: bool,
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
    let bytes = fs::read(file_path).expect("Unable to read file");

    // 16 bytes header
    let header = bytes[0..0x10].to_vec();

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

    // 0 or 512 bytes trainer
    let trainer_size = if has_trainer { 0 } else { 0x200 };
    let trainer_byte_range = 0x10..(0x10 + trainer_size);
    let trainer = bytes[trainer_byte_range.clone()].to_vec();

    // 0x4000 * program_size bytes program
    let program_size = (header[4] as u16 * 0x4000) as usize;
    let program_byte_range = trainer_byte_range.end..(&trainer_byte_range.end + program_size);
    let program = bytes[program_byte_range.clone()].to_vec();

    // 0x2000 * chr_size bytes tile map
    let chr_size = (header[5] as u16 * 0x2000) as usize;
    let chr_byte_range = program_byte_range.end..(&program_byte_range.end + chr_size);
    let chr = bytes[chr_byte_range.clone()].to_vec();

    let rom = ROM {
        header,
        chr,
        program,
        trainer,
    };

    return Ok(rom);
}