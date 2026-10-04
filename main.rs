use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // TODO: build an iterator pipeline over nums that keeps only the
    // even values, squares each one, and adds them up.
    let total: i64 = nums.iter().filter(|n| *n % 2 == 0).map(|n| n * n).sum();

    println!("{}", total);
}
