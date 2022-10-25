// https://www.hackerrank.com/challenges/lonely-integer/
// IO boilerplate provided by hackerrank
// Complexity: O(n)
// Non leet-code solution: using a frequecy map or set

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn lonely_integer(arr: &[i32]) -> i32 {
    return arr.iter().fold(0, |acc, n| {
        acc ^ n
    });
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lonely_integer(&a);
    writeln!(&mut fptr, "{}", result).ok();
}
