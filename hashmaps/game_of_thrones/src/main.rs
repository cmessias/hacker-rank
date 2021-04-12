// https://www.hackerrank.com/challenges/game-of-thrones

use std::{collections::HashMap, io};

fn game_of_thrones(s: &str) -> String {
    let odd_frequencies = s
        .chars()
        .fold(HashMap::<char, u32>::new(), |mut acc, c| {
            let freq = acc.entry(c).or_insert(0);
            *freq += 1;
            acc
        })
        .iter()
        .filter(|(_char, freq)| *freq % 2 != 0)
        .count();

    return if odd_frequencies <= 1 {
        String::from("YES")
    } else {
        String::from("NO")
    };
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", game_of_thrones(input.trim()));
}

#[cfg(test)]
mod test {
    use super::game_of_thrones;

    #[test]
    fn test_sample() {
        assert_eq!(game_of_thrones("aaabbbb"), String::from("YES"));
        assert_eq!(game_of_thrones("cdefghmnopqrstuvw"), String::from("NO"));
        assert_eq!(game_of_thrones("cdcdcdcdeeeef"), String::from("YES"))
    }
}
