use hamming::{weight_u8, weight_u64, weight_bytes, weight_words, distance_u8, distance_u64, distance_bytes, distance_words};

fn main() {
    println!("{}", weight_u8(5));          // 2 
    println!("{}", weight_u64(15));        // 4
    println!("{}", weight_bytes(vec![1,3,7])); // 6
    println!("{}", weight_words(vec![5,7])); // 5


    println!("distance_u8(5, 7) = {}", distance_u8(5, 7)); 

    println!("distance_u64(5, 7) = {}", distance_u64(5, 7));

    let bs = vec![1u8, 2u8, 3u8];
    let cs = vec![1u8, 3u8, 1u8];
    println!("distance_bytes([1,2,3], [1,3,1]) = {}", distance_bytes(bs, cs));

    let ws = vec![5u64, 7u64];
    let xs = vec![1u64, 7u64];
    println!("distance_words([5,7], [1,7]) = {}", distance_words(ws, xs));

    // distance_u8(5, 7) = 1
    // distance_u64(5, 7) = 1
    // distance_bytes([1,2,3], [1,3,1]) = 2
    // distance_words([5,7], [1,7]) = 1
}
