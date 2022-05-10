pub fn print_bytes_table(bytes: &Vec<u8>, start: usize, end: usize, chunk_size: usize) {
    let range: Vec<_> = (start..end + 1).collect();

    for chunk in range.chunks(chunk_size) {
        for address in chunk {
            eprint!("{:>width$} ", format!("{:x}", address), width = 6);
        }

        eprint!("\n");

        for address in chunk {
            eprint!(
                "{:>width$} ",
                to_hex_u16(bytes[*address as usize] as u16),
                width = 6
            );
        }

        eprint!("\n\n");
    }
}

pub fn to_hex_u8(x: u8) -> String {
    return format!("{:x}", x).to_uppercase();
}

pub fn to_hex_u16(x: u16) -> String {
    return format!("{:x}", x).to_uppercase();
}

pub fn to_hex_u32(x: u32) -> String {
    return format!("{:x}", x).to_uppercase();
}

pub fn to_binary_u8(x: u8) -> String {
    return format!("{:#010b}", x);
}

pub fn to_binary_u16(x: u16) -> String {
    return format!("{:#018b}", x);
}
