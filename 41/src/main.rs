fn main() {
    for i in [2,3,5,7,11] {
        println!("{i}");
    }
    let mut cnt = 5;
    let mut val = 13;
    while cnt < 8 {
        if is_prime(val) {
            println!("{val}");
            cnt += 1;
        }
        val += 2;
    }

    let nums: [u128; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

    for &n in &nums {
        let k = integer_sqrt(n);
        println!("n = {n}, integer_sqrt(n) = {k}, check: {}^2 = {}, ({}+1)^2 = {}", 
                 k, (k as u128) * (k as u128), k, (k as u128 + 1) * (k as u128 + 1));
    }
}

fn is_prime(n: u64) -> bool {

    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn integer_sqrt(n: u128) -> u64 {
    if n <= 1 { return n as u64; }

    let approx = (n as f64).sqrt();

    let int_approx = approx as u64;

    let mut root = int_approx as u128;

    for bit in (0..63).rev() {
    let candidate = root | (1u128 << bit);
        if candidate * candidate <= n {
            root = candidate;
        }
    }

    return root as u64
}