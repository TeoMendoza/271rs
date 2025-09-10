# A list of valid words, truncated for this example.
const WORDS: [&str; 5] = ["sator", "arepo", "tenet", "opera", "rotas"];

# ANSI color codes for colored text
# 31: Red, 32: Green, 33: Yellow
const RED : &str = "\u{001b}[31m";                                                                                                                                                                                      
const WHT : &str = "\u{001b}[0m";                                                                                                                                                                                       
const GRN: &str = "\u{001b}[32m";                                                                                                                                                                                       
const YEL: &str = "\u{001b}[33m";     

# Box-drawing characters for the game board
const T : str = "┌───┬───┬───┬───┬───┐";  # Top border
const M : str = "├───┼───┼───┼───┼───┤";  # Middle border
const B : str = "└───┴───┴───┴───┴───┘";  # Bottom border

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

    if WORDS.contains(&guess.trim()) {                                                                                                                                                                                  
        println!("BOOM BITCH");                                                                                                                                                                                         
    }                                                                                                                                                                                                                   
    else {                                                                                                                                                                                                              
        println!("No bitch")                                                                                                                                                                                            
    }                                                                                                                                                                                                                   
                                                                                                                                                                                                                        
                                                                                                                                                                                                                        
                                                                                                                                                                                                                        
}  

fn letter(a: &str, c: i32) {
    """
    Prints a single letter with a specified ANSI color.

    Args:
        a: The letter to print.
        c: The ANSI color code.
    """
    print!("│ \u{001b}[{}m{}\u{001b}[0m ", c, a);
}

fn colors(s: &str, answer: &str):
    """
    Analyzes a guessed word and prints it with the appropriate colors.

    Args:
        s: The guessed word.
        answer: The correct answer word.
    """
    for i in range(5):
        char = s[i]
        color_code = R
        if answer[i] == char:
            color_code = G
        elif char in answer:
            color_code = Y
        letter(char, color_code)
    print("│")

def game(words: list[str], answer: str):
    """
    Clears the screen and draws the game board with the current guesses.

    Args:
        words: A list of guessed words.
        answer: The correct answer word.
    """
    print("\u001b[2J")  # Clear the screen
    print(T)
    for i in range(5):
        colors(words[i], answer)
        print(M)
    colors(words[5], answer)
    print(B)

def main():
    """
    The main game loop.
    """
    words = ["     "] * 6

    ###############################################
    #                                             #
    # You are required to use /dev/random in Rust #
    #                                             #
    ###############################################
    import random
    answer = random.choice(WORDS)

    attempts = 0

    print("\u001b[2J", end="")  # Clear the screen
    print("Use lowercase only btw.")

    while words[5] == "     ":
        guess = input().strip()  # Convert input to lowercase
        if guess in WORDS:
            words[attempts] = guess
            game(words, answer)
            if guess == answer:
                print("Winner")
                return
            attempts += 1
        else:
            print("Not a valid word!")

    print("Game over: (")

