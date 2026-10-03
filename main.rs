use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i64 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // TODO: make `total` an accumulator and use a for loop over a range
    // so that it ends up holding 1 + 2 + ... + n.
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }

    println!("{}", total);
}
