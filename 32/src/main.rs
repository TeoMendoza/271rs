mod macros;

fn main() {
    /* Various Variables*/
    let a: [u64; 4] = [
        0x1111_1111_1111_0000,
        0x1111_0000_1100_1100,
        0x1100_1100_1010_1010,
        0x0123_4567_89AB_CDEF,
    ];

    println!("*Rotates use a decimal shift value, but print in hexadecimal:\n");

    let choice_val = choice!(a[0], a[1], a[2]);
    println!(
        "CHOICE(\n{:016X},\n{:016X},\n{:016X}) = \n--------\n{:016X}\n",
        a[0], a[1], a[2], choice_val
    );
    // Expected: 1111000011001010
    // (0x1111000011001010)

    let median_val = majority!(a[0], a[1], a[2]);
    println!(
        "MEDIAN(\n{:016X},\n{:016X},\n{:016X}) = \n--------\n{:016X}\n",
        a[0], a[1], a[2], median_val
    );
    // Expected: 1111110011101000
    // (0x1111110011101000)

    println!("*Rotates use a decimal shift value, but print in hexadecimal:\n");

    let r4  = rotate_right!(a[3], 4);
    let r8  = rotate_right!(a[3], 8);
    let r12 = rotate_right!(a[3], 12);
    let r2_1000  = rotate_right!(0x1000u64, 2);
    let r30_1000 = rotate_right!(0x1000u64, 30);

    println!("ROTATE(\n{:016X}, 04) = \n--------\n{:016X}\n", a[3],  r4);
    // Expected: F0123456789ABCDE

    println!("ROTATE(\n{:016X}, 08) = \n--------\n{:016X}\n", a[3],  r8);
    // Expected: EF0123456789ABCD

    println!("ROTATE(\n{:016X}, 12) = \n--------\n{:016X}\n", a[3],  r12);
    // Expected: DEF0123456789ABC

    println!("ROTATE(\n{:016X}, 02) = \n--------\n{:016X}\n", 0x1000u64, r2_1000);
    // Expected: 0000000000000400

    println!("ROTATE(\n{:016X}, 30) = \n--------\n{:016X}\n", 0x1000u64, r30_1000);
    // Expected: 0000400000000000
}

