use std::io::Write;
const B85: &[u8; 85] = br##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstu"##;


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let bytes = std::fs::read(path).unwrap();

    let mut out = String::new();
    let mut i = 0;

    while i + 4 <= bytes.len() {
        let a = bytes[i] as u32;
        let b = bytes[i + 1] as u32;
        let c = bytes[i + 2] as u32;
        let d = bytes[i + 3] as u32;
        i += 4;

        let quad = (a << 24) | (b << 16) | (c << 8) | d;
        let mut v = quad;
        let mut digits = [0u32; 5];
        for j in (0..5).rev() {
            digits[j] = v % 85;
            v /= 85;
        }

        for j in 0..5 {
            out.push(num_to_b85(digits[j] as u8).unwrap());
        }
    }

    let rem = bytes.len() - i;
    if rem == 1 {
        let a = bytes[i] as u32;
        let quad = a << 24;
        let mut v = quad;
        let mut digits = [0u32; 5];

        for j in (0..5).rev() {
            digits[j] = v % 85;
            v /= 85;
        }

        for j in 0..2 {
            out.push(num_to_b85(digits[j] as u8).unwrap());
        }

    }

    else if rem == 2 {
        let a = bytes[i] as u32;
        let b = bytes[i + 1] as u32;
        let quad = (a << 24) | (b << 16);
        let mut v = quad;
        let mut digits = [0u32; 5];

        for j in (0..5).rev() {
            digits[j] = v % 85;
            v /= 85;
        }

        for j in 0..3 {
            out.push(num_to_b85(digits[j] as u8).unwrap());
        }
    }

    else if rem == 3 {
        let a = bytes[i] as u32;
        let b = bytes[i + 1] as u32;
        let c = bytes[i + 2] as u32;
        let quad = (a << 24) | (b << 16) | (c << 8);
        let mut v = quad;
        let mut digits = [0u32; 5];
        
        for j in (0..5).rev() {
            digits[j] = v % 85;
            v /= 85;
        }

        for j in 0..4 {
            out.push(num_to_b85(digits[j] as u8).unwrap());
        }
    }

    let mut wrapped = String::with_capacity(out.len() + out.len() / 80 + 4);
    wrapped.push_str("<~");
    let mut count = 2;
    for b in out.bytes() {
        if count == 80 {
            wrapped.push('\n');
            count = 0;
        }
        wrapped.push(b as char);
        count += 1;
    }
    wrapped.push_str("~>");

    std::io::stdout().write_all(wrapped.as_bytes()).unwrap();
}


fn num_to_b85(n: u8) -> Option<char> {
    if n >= 85 {
        return None;
    }
    Some(B85[n as usize] as char)
}