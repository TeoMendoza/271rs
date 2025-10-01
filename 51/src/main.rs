use f16::f16;

fn main() {
    println_f16(i32_to_f16(12));
}

fn i32_to_f16(n:i32) -> f16 {
    if n == 0 {
            return f16 { bits: 0 };
        }

    // 1. Sign
    let sign = if n < 0 { 1u16 } else { 0u16 };

    // 2. Absolute value
    let x = n.unsigned_abs();

    // 3. Exponent = index of top bit
    let e = 31 - x.leading_zeros();
    let mut exp = (e as i32) + 15;

    // 4. Handle overflow (too big for f16 → infinity)
    if exp >= 31 {
        return f16 { bits: (sign << 15) | (0x1F << 10) };
    }

    // 5. Mantissa = bits after the leading 1
    let frac = x & ((1 << e) - 1); // drop the top 1
    let mantissa: u16 = if e <= 10 {
        // Fits exactly, just shift left
        ((frac << (10 - e)) as u16) & 0x03FF
    } else {
        // Too many bits, drop the extras (simple truncation here)
        ((frac >> (e - 10)) as u16) & 0x03FF
    };

    // 6. Pack into 16 bits
    let bits = (sign << 15) | ((exp as u16) << 10) | mantissa;
    f16 { bits }
}

fn print_f16(x: f16) {
    let sign_bit = (x.bits >> 15) & 1;
    let exp_bits = (x.bits >> 10) & 0x1F;
    let mant = x.bits & 0x03FF;

    let sign = if sign_bit == 1 { -1.0 } else { 1.0 };

    let value = if exp_bits == 0 {
        // Zero or subnormal
        let frac = mant as f32 / 1024.0;
        sign * frac * 2f32.powi(-14)
    } else if exp_bits == 31 {
        // Inf or NaN
        if mant == 0 { f32::INFINITY * sign } else { f32::NAN }
    } else {
        // Normalized
        let frac = mant as f32 / 1024.0;
        let significand = 1.0 + frac;
        let exp = (exp_bits as i32) - 15;
        sign * significand * 2f32.powi(exp)
    };

    println!("{}", value);
}


fn println_f16(x: f16) {
    print_f16(x);
    println!(); // just adds the newline
}