/*
// Borrow checker does not let me sort the borrowed slices
fn fraudulent_notifications(expenditure: Vec<u32>, d: usize) -> u32 {
    expenditure
        .windows(d)
        .zip(expenditure.iter().skip(d))
        .fold(0, |notifications, (&mut expenditure, current_spending)| {
            let mut sorted = expenditure.clone().sort();
            if (*current_spending as f64) > median(&expenditure) {
                notifications + 1
            } else {
                notifications
            }
        })
}
 */

// Solution exceeds time limits
fn fraudulent_notifications(mut expenditure: Vec<u32>, d: usize) -> u32 {
    let mut notifications = 0;
    for index in d..expenditure.len() {
        if f64::from(expenditure[index]) >= 2.0 * median(&mut expenditure[(index - d)..index]) {
            notifications += 1;
        }
    }

    return notifications;
}

fn median(values: &mut [u32]) -> f64 {
    values.sort();

    return if values.len() % 2 == 0 {
        let index = values.len() / 2;
        f64::from(values[index] + values[index + 1]) / 2.0
    } else {
        let index = values.len() / 2;
        f64::from(values[index])
    };
}

fn main() {
    let mut input = String::with_capacity(2);
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let d: usize = input_iter.next().unwrap().parse().unwrap();


    let mut input = String::with_capacity(n);
    std::io::stdin().read_line(&mut input).unwrap();
    let expenditure: Vec<u32> = input
        .split_whitespace().map(|s| s.parse().unwrap()).collect();

    println!("{}", fraudulent_notifications(expenditure, d));
}
