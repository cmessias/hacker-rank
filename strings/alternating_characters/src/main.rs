// https://www.hackerrank.com/challenges/alternating-characters/
// code golf, fewest lines wins

use std::io;
use std::io::BufRead;

fn deletions_required(s: &str) -> usize {
    s.as_bytes().windows(2).filter(|w| return w[0] == w[1]).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    let strings: Vec<String> = io::stdin().lock().lines()
        .take(n).map(|s| s.unwrap()).collect();

    for string in strings {
        println!("{}", deletions_required(&string))
    }
}
