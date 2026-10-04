use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();

    let mut counts: HashMap<&str, i64> = HashMap::new();
    // TODO 1: count how many times each word appears, using counts.entry(...).
    for w in &words {
        *counts.entry(*w).or_insert(0) += 1;
    }

    let mut best_word = words[0];
    let mut best_count: i64 = 0;
    // TODO 2: walk `words` in input order, look up each word's count,
    // and keep the word with the highest count (the earliest one wins a tie).
    let _ = &mut counts;
    for w in &words {
        match counts.get(w) {
            Some(c) => {
                if best_count < *c {
                    best_word = w;
                    best_count = *c;
                }
            }
            None => {}
        }
    }

    println!("{} {}", best_word, best_count);
}
