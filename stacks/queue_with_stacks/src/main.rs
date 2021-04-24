// https://www.hackerrank.com/challenges/ctci-queue-using-two-stacks/

use std::io;
use std::io::BufRead;

enum Query {
    Enqueue(i32),
    Dequeue,
    Print,
}

impl Query {
    fn from_string(s: &str) -> Query {
        let values = s.split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect::<Vec<i32>>();

        return match values[0] {
            1 => Query::Enqueue(values[1]),
            2 => Query::Dequeue,
            3 => Query::Print,
            _ => panic!("unknown query: {}", values[0]),
        };
    }

    fn run(queries: &[Query]) {
        let mut insert_stack: Vec<i32> = Vec::with_capacity(queries.len());
        let mut pop_stack: Vec<i32> = Vec::with_capacity(queries.len());

        for query in queries {
            match query {
                Query::Enqueue(element) => insert_stack.push(*element),
                Query::Dequeue => {
                    if pop_stack.len() > 0 {
                        pop_stack.pop();
                    } else {
                        while let Some(top) = insert_stack.pop() {
                            pop_stack.push(top);
                        }
                        pop_stack.pop();
                    }
                }
                Query::Print =>
                    if pop_stack.len() > 0 {
                        println!("{}", pop_stack.last().unwrap())
                    } else {
                        while let Some(top) = insert_stack.pop() {
                            pop_stack.push(top);
                        }
                        println!("{}", pop_stack.last().unwrap())
                    }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    let queries: Vec<Query> = io::stdin()
        .lock()
        .lines()
        .map(|s| Query::from_string(&s.unwrap()))
        .collect();

    Query::run(&queries);
}
