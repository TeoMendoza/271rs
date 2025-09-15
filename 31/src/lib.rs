pub fn distance_u8(b:u8, c:u8) -> u64 {
    let mut diff : u8 = b ^ c;
    let mut count : u64 = 0;

    while diff > 0 {
        if diff & 1 == 1 {
            count += 1;
        }
        diff >>= 1;
    }

    return count
}

pub fn distance_u64(w:u64, x:u64) -> u64 {
    let mut diff: u64 = w ^ x;
    let mut count: u64 = 0;

    while diff > 0 {
        if diff & 1 == 1 {
            count += 1;
        }
        diff >>= 1;
    }

    return count
}

pub fn distance_bytes(bs:Vec<u8>, cs:Vec<u8>) -> u64 {
    let mut total: u64 = 0;

    for (b, c) in bs.iter().zip(cs.iter()) {
        total += distance_u8(*b, *c);
    }

    return total
}

pub fn distance_words(ws:Vec<u64>, xs:Vec<u64>) -> u64 {
    let mut total: u64 = 0;

    for (w, x) in ws.iter().zip(xs.iter()) {
        total += distance_u64(*w, *x);
    }

    return total
}