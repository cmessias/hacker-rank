// https://www.hackerrank.com/challenges/sherlock-and-valid-string/

use std::collections::HashMap;
use std::io;
use std::iter::FromIterator;

fn is_valid_string(s: &str) -> bool {
    let frequencies = s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    }).values().fold(HashMap::<i32, i32>::new(), |mut acc, &i| {
        *acc.entry(i).or_insert(0) += 1;
        acc
    });

    if frequencies.len() == 1 { return true; }
    if frequencies.len() > 2 { return false; }

    let key1 = frequencies.keys().next().unwrap();
    let key2 = frequencies.keys().nth(1).unwrap();

    return frequencies[key1] == 1 && (key1 - 1 == *key2 || key1 - 1 == 0)
        || frequencies[key2] == 1 && (key2 - 1 == *key1 || key2 - 1 == 0);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", if is_valid_string(&input.trim()) { "YES" } else { "NO " })
}
