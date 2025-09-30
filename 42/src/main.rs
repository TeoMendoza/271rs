use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // ---- read whole file into bytes ----
    let path = env::args().nth(1).expect("usage: sha512 <file>");
    let mut msg = fs::read(&path)?;

    // ---- SHA-512 constants (FIPS 180-4) ----
    // Initial hash state (H0..H7)
    let mut h: [u64; 8] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    ];

    // Round constants K[0..80)
    let k: [u64; 80] = [
        0x428a2f98d728ae22,0x7137449123ef65cd,0xb5c0fbcfec4d3b2f,0xe9b5dba58189dbbc,
        0x3956c25bf348b538,0x59f111f1b605d019,0x923f82a4af194f9b,0xab1c5ed5da6d8118,
        0xd807aa98a3030242,0x12835b0145706fbe,0x243185be4ee4b28c,0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,0x80deb1fe3b1696b1,0x9bdc06a725c71235,0xc19bf174cf692694,
        0xe49b69c19ef14ad2,0xefbe4786384f25e3,0x0fc19dc68b8cd5b5,0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,0x4a7484aa6ea6e483,0x5cb0a9dcbd41fbd4,0x76f988da831153b5,
        0x983e5152ee66dfab,0xa831c66d2db43210,0xb00327c898fb213f,0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,0xd5a79147930aa725,0x06ca6351e003826f,0x142929670a0e6e70,
        0x27b70a8546d22ffc,0x2e1b21385c26c926,0x4d2c6dfc5ac42aed,0x53380d139d95b3df,
        0x650a73548baf63de,0x766a0abb3c77b2a8,0x81c2c92e47edaee6,0x92722c851482353b,
        0xa2bfe8a14cf10364,0xa81a664bbc423001,0xc24b8b70d0f89791,0xc76c51a30654be30,
        0xd192e819d6ef5218,0xd69906245565a910,0xf40e35855771202a,0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,0x1e376c085141ab53,0x2748774cdf8eeb99,0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,0x4ed8aa4ae3418acb,0x5b9cca4f7763e373,0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,0x78a5636f43172f60,0x84c87814a1f0ab72,0x8cc702081a6439ec,
        0x90befffa23631e28,0xa4506cebde82bde9,0xbef9a3f7b2c67915,0xc67178f2e372532b,
        0xca273eceea26619c,0xd186b8c721c0c207,0xeada7dd6cde0eb1e,0xf57d4f7fee6ed178,
        0x06f067aa72176fba,0x0a637dc5a2c898a6,0x113f9804bef90dae,0x1b710b35131c471b,
        0x28db77f523047d84,0x32caab7b40c72493,0x3c9ebe0a15c9bebc,0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,0x597f299cfc657e2a,0x5fcb6fab3ad6faec,0x6c44198c4a475817,
    ];

    // ---- tiny inline helpers (no separate functions) ----
    let rotr  = |x: u64, n: u32| -> u64 { (x >> n) | (x << (64 - n)) };
    let shr   = |x: u64, n: u32| -> u64 { x >> n };
    let ch    = |x: u64, y: u64, z: u64| -> u64 { (x & y) ^ ((!x) & z) };
    let maj   = |x: u64, y: u64, z: u64| -> u64 { (x & y) ^ (x & z) ^ (y & z) };
    let big_s0 = |x: u64| -> u64 { rotr(x,28) ^ rotr(x,34) ^ rotr(x,39) };
    let big_s1 = |x: u64| -> u64 { rotr(x,14) ^ rotr(x,18) ^ rotr(x,41) };
    let sml_s0 = |x: u64| -> u64 { rotr(x, 1) ^ rotr(x, 8) ^  shr(x, 7) };
    let sml_s1 = |x: u64| -> u64 { rotr(x,19) ^ rotr(x,61) ^  shr(x, 6) };

    // ---- padding: append 0x80, zeros to 112 mod 128, then 128-bit length ----
    let bit_len: u128 = (msg.len() as u128) * 8;
    msg.push(0x80);
    while (msg.len() % 128) != 112 { msg.push(0x00); }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // ---- process 1024-bit blocks ----
    let mut w = [0u64; 80];
    let mut i = 0;
    while i < msg.len() {
        // W[0..15]: load 16 u64s (big-endian)
        for t in 0..16 {
            let j = i + t * 8;
            w[t] = ((msg[j + 0] as u64) << 56)
                 | ((msg[j + 1] as u64) << 48)
                 | ((msg[j + 2] as u64) << 40)
                 | ((msg[j + 3] as u64) << 32)
                 | ((msg[j + 4] as u64) << 24)
                 | ((msg[j + 5] as u64) << 16)
                 | ((msg[j + 6] as u64) <<  8)
                 | ((msg[j + 7] as u64) <<  0);
        }
        // W[16..79]: message schedule expansion
        for t in 16..80 {
            w[t] = w[t-16]
                 .wrapping_add(sml_s0(w[t-15]))
                 .wrapping_add(w[t-7])
                 .wrapping_add(sml_s1(w[t-2]));
        }

        // working variables
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

        // 80 rounds
        for t in 0..80 {
            let t1 = hh
                .wrapping_add(big_s1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(k[t])
                .wrapping_add(w[t]);
            let t2 = big_s0(a).wrapping_add(maj(a, b, c));
            hh = g;
            g  = f;
            f  = e;
            e  = d.wrapping_add(t1);
            d  = c;
            c  = b;
            b  = a;
            a  = t1.wrapping_add(t2);
        }

        // add back into state (mod 2^64)
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);

        i += 128;
    }

    // ---- output 128-hex digest + "  <filename>" (sha512sum style) ----
    let mut out = String::with_capacity(128);
    for &word in &h { out.push_str(&format!("{:016x}", word)); }
    println!("{}  {}", out, path);
    Ok(())
}
