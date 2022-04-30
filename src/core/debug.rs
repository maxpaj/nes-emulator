pub fn print_bytes_table(bytes: Vec<u8>, start: usize, end: usize, chunk_size: usize) {
    let range: Vec<_> = (start..end + 1).collect();

    for chunk in range.chunks(chunk_size) {
        for address in chunk {
            print!("{:>width$} ", format!("{:x}", address), width=6);
        }
        
        print!("\n");
        
        for address in chunk {
            print!("{:>width$} ", format!("{:x}", bytes[*address as usize]), width=6);
        }
        
        print!("\n\n");
    }
}

pub fn disassemble(bytes: Vec<u8>, start: usize, end: usize) {
    //  print("{}");
}