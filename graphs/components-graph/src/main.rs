// https://www.hackerrank.com/challenges/components-in-graph/
// Code golf, fewest lines wins

use std::cmp;
use std::collections::HashMap;
use std::io::BufRead;

fn make_set(n: usize) -> Vec<usize> {
    (0..=2 * n).collect()
}

fn find(x: usize, parent: &[usize]) -> usize {
    return if x == parent[x] { x }
        else { find(parent[x], parent) };
}

fn union(x: usize, y: usize, parent: &mut [usize]) {
    let x = find(x, parent);
    let y = find(y, parent);

    if x == y { return; }
    parent[y] = x;
}

fn count_components(parent: &[usize]) -> (u32, u32) {
    let mut components = HashMap::<usize, u32>::new();
    for &x in parent {
        *components.entry(find(x, parent)).or_insert(0) += 1;
    }

    let components = components.values().filter(|x| **x > 1);
    return (*components.clone().min().unwrap(), * components.max().unwrap());
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let connections: Vec<(usize, usize)> = std::io::stdin()
        .lock()
        .lines()
        .take(n)
        .map(|s| split_pair(&s.unwrap()))
        .collect();

    let mut parent = make_set(n);
    for (x, y) in connections {
        union(x, y, &mut parent)
    }

    println!("{:?}", count_components(&parent))
}

fn split_pair(s: &str) -> (usize, usize) {
    let mut iter = s.split_whitespace().into_iter();
    let n1 = iter.next().unwrap().parse().unwrap();
    let n2 = iter.next().unwrap().parse().unwrap();
    return (n1, n2);
}
