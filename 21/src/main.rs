use std::fs::File;
const RED : &str = "\u{001b}[31m";
const WHT : &str = "\u{001b}[0m";
const GRN: &str = "\u{001b}[32m";
const YEL: &str = "\u{001b}[33m";
const WORDS : [&str; 5] = ["rotas", "opera", "tenet", "arepo", "sator"];

fn main() {
    let s = "Hello, world";
    println!("{:?}", s.chars().nth(10).unwrap());
    let mut devrnd = std::fs::File::open("/dev/urandom").unwrap();
    let mut buffer = [0u8; (usize::BITS / 8) as usize];
    std::io::Read::read_exact(&mut devrnd, &mut buffer).unwrap();
    let mut secret = usize::from_ne_bytes(buffer);
    let answer : String = String::from(WORDS[secret % WORDS.len()]);
	let mut guess = String::new();    
std::io::stdin().read_line(&mut guess).unwrap();
    
    println!("{:?}", guess.trim());
    if WORDS.contains(&guess.trim()) {
    	println!("BOOM BITCH");
    }
    else {
        println!("No bitch")
    }



}
