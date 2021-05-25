// https://www.hackerrank.com/challenges/pairs/

use std::collections::HashSet;
use std::iter::FromIterator;

fn pairs(values: &[u32], k: u32) -> usize {
    let set: HashSet<&u32> = HashSet::from_iter(values.iter());
    values.iter().filter(|&&x| set.contains(&(x + k))).count()
}

fn main() {
    let mut input = String::with_capacity(2);
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let k: u32 = input_iter.next().unwrap().parse().unwrap();

    let mut input = String::with_capacity(n);
    std::io::stdin().read_line(&mut input).unwrap();
    let values: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", pairs(&values, k));
}
