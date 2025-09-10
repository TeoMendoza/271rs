use std::fs::File;
use std::io::{self, Read};

// A list of valid words, truncated for this example.
const WORDS: [&str; 5] = ["sator", "arepo", "tenet", "opera", "rotas"];

const RED : &str = "\u{001b}[31m";                                                                                                                                                                                                                                                
const WHT : &str = "\u{001b}[0m";                                                                                                                                                                                                                                                 
const GRN: &str = "\u{001b}[32m";                                                                                                                                                                                                                                                 
const YEL: &str = "\u{001b}[33m";   
const RESET: &str = "\x1b[0m";

// Box-drawing characters for the game board
const T : &str = "┌───┬───┬───┬───┬───┐";  // Top border
const M : &str = "├───┼───┼───┼───┼───┤";  // Middle border
const B : &str = "└───┴───┴───┴───┴───┘";  // Bottom border

fn main() {
    // Make a mutable vector of six "     " placeholders
    let mut words: [String; 6] = std::array::from_fn(|_| "     ".to_string());
    // -----------------------------------
    // Pick random answer using /dev/random
    // -----------------------------------
    let mut devrnd = File::open("/dev/random").expect("failed to open /dev/random");
    let mut buffer = [0u8; (usize::BITS / 8) as usize];
    devrnd.read_exact(&mut buffer).expect("failed to read random bytes");
    let secret = usize::from_ne_bytes(buffer);
    let answer: &str = WORDS[secret % WORDS.len()];

    // ----------------------
    // Screen + initial prompt
    // ----------------------
    print!("\x1b[2J"); // clear screen (no newline)
    println!("Use lowercase only btw.");

    // -------------
    // Main game loop
    // -------------
    let mut attempts: usize = 0;

    // Keep going while the last slot is still the blank placeholder
    while words[5] == "     " {
        // Read input line
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess_trim = guess.trim();

        // Check guess validity
        if WORDS.contains(&guess_trim) {
            // Put guess into current attempts slot
            words[attempts] = guess_trim.to_string();

            // Call your game renderer; it wants &[&str], so make a borrowed view
            let view: [&str; 6] = std::array::from_fn(|i| words[i].as_str());
	    game(&view, answer);
            // Win check
            if guess_trim == answer {
                println!("Winner");
                return;
            }

            attempts += 1;
            if attempts >= 6 {
                break; // safety, though the while-condition also stops us
            }
        } else {
            println!("Not a valid word!");
        }
    }

    println!("Game over :(");
}

fn letter(a: char, c: &str) {
    print!("│ {c}{a}{RESET} ");
}

fn colors(s: &str, answer: &str) {

    for i in 0..5 {
        let ch = s.chars().nth(i).unwrap();
        let ans_ch = answer.chars().nth(i).unwrap();

        let color = if ch == ans_ch {
            GRN
        } else if answer.contains(ch) {
            YEL
        } else {
            RED
        };

        letter(ch, color);
    }

    println!("│"); // right border
}

fn game(words: &[&str], answer: &str) {
    print!("\x1b[2J");
    println!("{T}");

    for i in 0..5 {
        colors(words[i], answer);  // ✅ indexing is fine
        println!("{M}");
    }

    colors(words[5], answer);      // ✅ also fine if words has 6 entries
    println!("{B}");
}
