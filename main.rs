use std::io::{self, BufRead};

// Count the vowels (a, e, i, o, u, in either case) in the borrowed text.
fn vowel_count(s: &str) -> usize {
    // TODO: walk the characters and count the vowels
    let mut count: usize = 0;
    for c in s.chars() {
        if c == 'a'
            || c == 'A'
            || c == 'e'
            || c == 'E'
            || c == 'i'
            || c == 'I'
            || c == 'o'
            || c == 'O'
            || c == 'u'
            || c == 'U'
        {
            count += 1;
        }
    }
    count
}

// Change the caller's String in place: make it uppercase, then make sure
// it ends with '!' (add one only if it does not already end in '!').
fn make_loud(s: &mut String) {
    // TODO: modify the String behind the mutable reference
    *s = s.to_uppercase().to_string();
    if !s.ends_with('!') {
        s.push('!')
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = stdin.lock().lines().next().unwrap().unwrap();
    let before = vowel_count(&line);
    make_loud(&mut line);
    println!("{}", before);
    println!("{}", line);
    println!("{}", vowel_count(&line));
}
