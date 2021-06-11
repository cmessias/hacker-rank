use std::collections::{HashMap, VecDeque};
use std::io::BufRead;

fn find_distances(graph: Vec<Vec<usize>>, starting_node: usize) -> Vec<i32> {
    let mut distances: Vec<i32> = vec![-1; graph.len()];
    let mut to_visit: VecDeque<usize> = VecDeque::new();

    distances[starting_node] = 0;
    to_visit.push_front(starting_node);

    while let Some(node) = to_visit.pop_front() {
        for &child in &graph[node] {
            if not_visited(child, &distances) {
                distances[child] = distances[node] + 6;
                to_visit.push_back(child);
            }
        }
    }

    distances.remove(starting_node);
    return distances;
}

fn not_visited(child: usize, distances: &Vec<i32>) -> bool {
    return distances[child] == -1;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    (0..q).for_each(|_| {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input_iter = input.split_whitespace();
        let n: usize = input_iter.next().unwrap().parse().unwrap();
        let m: usize = input_iter.next().unwrap().parse().unwrap();

        let edges: Vec<(usize, usize)> = std::io::stdin()
            .lock()
            .lines()
            .take(m)
            .map(|s| extract_pair(&s.unwrap()))
            .collect();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let s: usize = input.trim().parse().unwrap();

        let graph = create_graph(n, edges);

        find_distances(graph, s - 1).iter().for_each(|d| {
            print!("{} ", d);
        });
        println!()
    });
}

fn create_graph(size: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = vec![vec![]; size];

    for (u, v) in edges {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    return graph;
}

fn extract_pair(s: &str) -> (usize, usize) {
    let mut s_iter = s.split_whitespace();
    let a: usize = s_iter.next().unwrap().parse().unwrap();
    let b: usize = s_iter.next().unwrap().parse().unwrap();
    return (a, b);
}
