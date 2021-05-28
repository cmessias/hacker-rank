fn maximum_cost_incurred(costs: &[u32], labels: &[&str], daily_target: u32) -> u32 {
    let mut total_cost = 0;
    let mut daily_count = daily_target;
    let mut results = vec![];

    for (cost, label) in costs.iter().zip(labels) {
        total_cost += cost;
        if *label == "legal" {
            daily_count -= 1;
        }
        if daily_count == 0 {
            results.push(total_cost);
            total_cost = 0;
            daily_count = daily_target;
        }
    }

    return *results.iter().max().unwrap();
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::maximum_cost_incurred;

    #[test]
    fn case1() {
        let costs = vec![0, 3, 2, 3, 4];
        let labels = vec!["legal", "legal", "illegal", "legal", "legal"];
        let daily_target = 1;
        assert_eq!(maximum_cost_incurred(&costs, &labels, daily_target), 5)
    }
}
