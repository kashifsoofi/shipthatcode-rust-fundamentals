use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().next().unwrap().unwrap();

    let trimmed = line.trim();
    let uppercase = trimmed.to_uppercase();
    let char_count = trimmed.chars().count();
    let byte_count = trimmed.len();
    // TODO: take a trimmed &str view of `line`, then print three lines:
    //   1. that text in uppercase
    //   2. how many characters it has
    //   3. how many bytes it has
    println!("{}", uppercase);
    println!("{}", char_count);
    println!("{}", byte_count);
}
