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
}

struct Queue {
    insert_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

impl Queue {
    fn with_capacity(n: usize) -> Queue {
        Queue {
            insert_stack: Vec::with_capacity(n),
            pop_stack: Vec::with_capacity(n),
        }
    }

    fn run(&mut self, queries: &[Query]) {
        for query in queries {
            match query {
                Query::Enqueue(element) => {
                    self.insert_stack.push(*element);
                }
                Query::Dequeue => {
                    self.prepare_top();
                    self.pop_stack.pop();
                }
                Query::Print => {
                    self.prepare_top();
                    println!("{}", self.pop_stack.last().unwrap())
                }
            }
        }
    }

    fn prepare_top(&mut self) {
        if self.pop_stack.is_empty() {
            while let Some(top) = self.insert_stack.pop() {
                self.pop_stack.push(top);
            }
        }
    }
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let queries: Vec<Query> = io::stdin()
        .lock()
        .lines()
        .map(|s| Query::from_string(&s.unwrap()))
        .collect();

    let mut queue = Queue::with_capacity(n);
    queue.run(&queries);
}
