pub fn to_bytes(mut number: usize, length: u8) -> Vec<u8> {
    let mut out = Vec::new();
    
    for _ in 0..length {
        out.insert(0, (number & 0xFF) as u8);
        number >>= 8;
    }
    
    out
}

pub fn print_slice(slice: &[u8]) {
    for byte in slice {
        print!("{:02X} ", byte);
    }
    println!("")
}

pub fn format_slice_hex(slice: &[u8]) -> String {
    let mut s = String::new();
    for byte in slice {
        s.push_str(&format!("{:02X} ", byte));
    }
    s.trim().to_owned()
}

pub fn format_slice_bin(slice: &[u8]) -> String {
    let mut s = String::new();
    for byte in slice {
        s.push_str(&format!("{:08b} ", byte));
    }
    s
}