fn get_min_cost(crews: &mut [u32], jobs: &mut [u32]) -> u32 {
    crews.sort_unstable();
    jobs.sort_unstable();

    crews
        .iter()
        .zip(jobs)
        .fold(0, |acc, (crew, job)| acc + (*job - *crew))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::get_min_cost;

    #[test]
    fn case1() {
        let mut crews = vec![5, 1, 4, 2];
        let mut jobs = vec![4, 7, 9, 10];
        assert_eq!(get_min_cost(&mut crews, &mut jobs), 18);
    }

    #[test]
    fn case2() {
        let mut crews = vec![5, 3, 1, 4, 6];
        let mut jobs = vec![9, 8, 3, 15, 1];
        assert_eq!(get_min_cost(&mut crews, &mut jobs), 17);
    }
}
