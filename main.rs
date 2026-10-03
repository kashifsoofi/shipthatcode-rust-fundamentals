use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let w: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    let h: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    let k: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    // TODO: compute the area, each person's whole share, and the leftover,
    // then print those three numbers instead of the zeros below.
    let area = w * h;
    let share = area / k;
    let remaining = area % k;
    println!("{}", area);
    println!("{}", share);
    println!("{}", remaining);
}
