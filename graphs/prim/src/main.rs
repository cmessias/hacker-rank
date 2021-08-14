// https://www.hackerrank.com/challenges/primsmstsub/problem

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::convert::From;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
struct Edge {
    from: usize,
    to: usize,
    weight: u32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.weight.cmp(&self.weight))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl Eq for Edge {}

impl From<String> for Edge {
    fn from(s: String) -> Self {
        let mut iter = s.split_whitespace();
        let from = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let weight = iter.next().unwrap().parse().unwrap();
        Edge { from, to, weight }
    }
}

fn build_adjacency_list(edges: &[Edge]) -> Vec<Vec<Edge>> {
    return edges
        .iter()
        .fold(vec![vec![]; edges.len()], |mut acc, edge| {
            acc[edge.from].push(Edge { from: edge.from, to: edge.to, weight: edge.weight, });
            acc[edge.to].push(Edge { from: edge.to, to: edge.from, weight: edge.weight, });
            acc
        });

}

fn prim(graph: &[Vec<Edge>], start: usize) -> u32 {
    let mut total = 0;

    let mut visited = HashSet::with_capacity(graph.len());
    let mut to_visit = BinaryHeap::with_capacity(graph.len());
    add_edges(&graph[start], &mut to_visit);
    visited.insert(start);

    while let Some(edge) = to_visit.pop() {
        if !visited.contains(&edge.to) {
            visited.insert(edge.to);
            add_edges(&graph[edge.to], &mut to_visit);
            total += edge.weight
        }
    }

    return total;
}

fn add_edges(nodes: &[Edge], heap: &mut BinaryHeap<Edge>) {
    for node in nodes {
        heap.push(*node)
    }
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input.split_whitespace();
    let _n: usize = input_iter.next().unwrap().parse().unwrap();
    let m: usize = input_iter.next().unwrap().parse().unwrap();

    let edges: Vec<Edge> = std::io::stdin()
        .lock()
        .lines()
        .take(m)
        .map(|s| Edge::from(s.unwrap()))
        .collect();

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let s: usize = input.trim().parse::<usize>().unwrap() - 1;

    println!("{}", prim(&build_adjacency_list(&edges), s))
}
