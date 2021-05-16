use std::io;

fn minimum_swap(arr: Vec<i32>) -> Option<i32> {
    let mut swaps: i32 = 0;
    for (person, i) in arr.into_iter().zip(1..) {
        //println!("{} {}", person, i);
        if person > i + 2 {
            //println!("gotten person {} at position {}", person, i);
            return None;
        }
        if person > i {
            swaps += person - i & arr[5..].iter().count()
        }
    }
    return Some(swaps);
}

fn main() {
    let mut input = String::with_capacity(1);
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let queues: Vec<Vec<i32>> = (0..t)
        .map(|_| {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let n: usize = input.trim().parse().unwrap();

            let mut queue_input = String::with_capacity(n);
            io::stdin().read_line(&mut queue_input).unwrap();

            let queue: Vec<i32> = queue_input
                .split_whitespace()
                .map(|s| s.trim().parse().unwrap())
                .collect();

            return queue;
        })
        .collect();

    for queue in queues {
        if let Some(swap) = minimum_swap(queue) {
            println!("{}", swap);
        } else {
            println!("Too chaotic")
        }
    }
}
