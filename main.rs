use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    // TODO: choose between FizzBuzz, Fizz, Buzz and the number itself.
    match (n % 3 == 0, n % 5 == 0) {
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (true, true) => println!("FizzBuzz"),
        _ => println!("{}", n),
    }
}
