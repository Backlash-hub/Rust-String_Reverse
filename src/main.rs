//Pass in a string via cargo run <string>  and it will return the string reversed
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let reversed_pattern: String = pattern.chars().rev().collect();

    println!("Original: {}", pattern);
    println!("Reversed: {}", reversed_pattern);
}
