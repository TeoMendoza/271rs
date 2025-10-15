
use std::env;

use numerical::{
    ix,
    ix_from_hex,
    ix_to_hex,
    add_ix,
    sub_ix,
    mul_ix,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    // Expect: cargo run -- 0xA 0xB OP
    if args.len() != 4 {
        eprintln!("Incorrect Arguments");
        std::process::exit(1);
    }

    let a_hex = &args[1];
    let b_hex = &args[2];
    let op = args[3].to_uppercase();

    let a: ix = ix_from_hex(a_hex);
    let b: ix = ix_from_hex(b_hex);

    let result = match op.as_str() {
        "ADD" => add_ix(&a, &b),
        "SUB" => sub_ix(&a, &b),
        "MUL" => mul_ix(&a, &b),
        // "QUO" | "DIV" => quo_ix(&a, &b),
        // "REM" => rem_ix(&a, &b),
        other => {
            eprintln!("unknown op: {}", other);
            std::process::exit(2);
        }
    };

    print!("{}", ix_to_hex(&result));
}
