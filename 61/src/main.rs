use std::io::Write;
const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let bytes = std::fs::read(path).unwrap();

    let mut out = String::new();
    let mut i = 0;

    while i + 3 <= bytes.len() {
        let a = bytes[i] as u32;
        let b = bytes[i + 1] as u32;
        let c = bytes[i + 2] as u32;
        i += 3;

        let triple = (a << 16) | (b << 8) | c;
        out.push(num_to_b64(((triple >> 18) & 0x3F) as u8).unwrap());
        out.push(num_to_b64(((triple >> 12) & 0x3F) as u8).unwrap());
        out.push(num_to_b64(((triple >> 6) & 0x3F) as u8).unwrap());
        out.push(num_to_b64((triple & 0x3F) as u8).unwrap());
    }

    let rem = bytes.len() - i;
    if rem == 1 {
        let a = bytes[i] as u32;
        let triple = a << 16;
        out.push(num_to_b64(((triple >> 18) & 0x3F) as u8).unwrap());
        out.push(num_to_b64(((triple >> 12) & 0x3F) as u8).unwrap());
        out.push('=');
        out.push('=');
    } else if rem == 2 {
        let a = bytes[i] as u32;
        let b = bytes[i + 1] as u32;
        let triple = (a << 16) | (b << 8);
        out.push(num_to_b64(((triple >> 18) & 0x3F) as u8).unwrap());
        out.push(num_to_b64(((triple >> 12) & 0x3F) as u8).unwrap());
        out.push(num_to_b64(((triple >> 6) & 0x3F) as u8).unwrap());
        out.push('=');
    }

    let mut wrapped = String::with_capacity(out.len() + out.len() / 76 + 2);
    let mut count = 0;
    for b in out.bytes() {
        if count == 76 {
            wrapped.push('\n');
            count = 0;
        }
        wrapped.push(b as char);
        count += 1;
    }
    wrapped.push('\n');

    std::io::stdout().write_all(wrapped.as_bytes()).unwrap();
}

fn num_to_b64(n: u8) -> Option<char> {
    if n >= 64 {
        return None;
    }
    Some(B64[n as usize] as char)
}