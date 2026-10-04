use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // TODO: walk `nums`, keep the largest value seen so far,
    // and print it once after the loop.
    let mut largest = nums[0];
    for n in &nums {
        if largest < *n {
            largest = *n;
        }
    }
    println!("{}", largest); // placeholder: prints the count, not the answer; replace it
}
