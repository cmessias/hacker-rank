// https://www.hackerrank.com/challenges/mark-and-toys

use std::io;

fn maximum_toys(budget: u32, prices: &mut [u32]) -> u32 {
    prices.sort_unstable();

    let mut remaining_budget = budget;
    let mut number_of_toys = 0;
    for price in prices {
        if remaining_budget >= *price {
            remaining_budget -= *price;
            number_of_toys += 1;
        } else {
            break;
        }
    }
    return number_of_toys;
}

fn main() {
    let mut input = String::with_capacity(2);
    io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input.trim().split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let k: u32 = input_iter.next().unwrap().parse().unwrap();

    let mut input = String::with_capacity(n);
    io::stdin().read_line(&mut input).unwrap();
    let mut prices: Vec<u32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", maximum_toys(k, &mut prices));
}
