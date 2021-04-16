// https://www.hackerrank.com/challenges/ctci-find-the-running-median

use io::BufRead;
use std::cmp::Reverse;
use std::{collections::BinaryHeap, io};

fn running_median(numbers: Vec<i32>) -> Vec<f64> {
    numbers
        .iter()
        .fold(
            (
                Vec::with_capacity(numbers.len()),
                BinaryHeap::<i32>::new(),
                BinaryHeap::<Reverse<i32>>::new(),
            ),
            |(mut medians, mut low, mut high), n| {
                if high.len() <= low.len() {
                    high.push(Reverse(*n));
                } else {
                    low.push(*n);
                }

                balance(&mut low, &mut high);
                medians.push(median(&low, &high));
                
                return (medians, low, high);
            },
        )
        .0
}

fn balance(low: &mut BinaryHeap<i32>, high: &mut BinaryHeap<Reverse<i32>>) {
    if !high.is_empty() && !low.is_empty() && high.peek().unwrap().0 < *low.peek().unwrap() {
        let h = high.pop().unwrap().0;
        high.push(Reverse(low.pop().unwrap()));
        low.push(h);
    }
}

fn median(low: &BinaryHeap<i32>, high: &BinaryHeap<Reverse<i32>>) -> f64 {
    return if high.len() > low.len() {
        f64::from(high.peek().unwrap().0)
    } else {
        f64::from(high.peek().unwrap().0 + low.peek().unwrap()) / 2.0
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    let numbers: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|n| n.unwrap().trim().parse().unwrap())
        .collect();

    running_median(numbers).iter().for_each(|m| {
        println!("{:.1}", m);
    });
}
