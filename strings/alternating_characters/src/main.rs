use std::io;
use std::io::BufRead;

fn deletions_required(s: &str) -> u32 {
    s.as_bytes().windows(2).fold(0, |acc, window| {
        if window[0] == window[1] { acc + 1 } else { acc }
    })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    let strings: Vec<String> = io::stdin()
        .lock()
        .lines()
        .take(n)
        .map(|s| s.unwrap())
        .collect();

    for string in strings {
        println!("{}", deletions_required(&string))
    }
}
