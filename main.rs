use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first = lines.next().unwrap().unwrap();
    let second = lines.next().unwrap().unwrap();

    // TODO: shadow `first` and `second` as i64 numbers (the text may have spaces around it).
    // TODO: keep a mutable running total starting at 0; add each number and print the total after each addition.
    let first = first.trim();
    let second = second.trim();
    let first: i64 = first.parse().unwrap();
    let second: i64 = second.parse().unwrap();

    let mut total: i64 = 0;
    total += first;
    println!("{}", total);
    total += second;
    println!("{}", total);
}
