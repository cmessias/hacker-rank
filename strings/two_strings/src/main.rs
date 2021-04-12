use std::collections::HashSet;
use std::iter::FromIterator;
use std::io;

fn two_strings(s1: &str, s2: &str) -> String {
    let s1: HashSet<char> = HashSet::from_iter(s1.to_lowercase().chars());
    let s2 = s2.to_lowercase();

    for c in s2.chars() {
        if s1.contains(&c) {
            return String::from("YES");
        }
    }

    return String::from("NO");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut s1 = String::new();
    let mut s2 = String::new();
    for _ in 0..n {
        io::stdin().read_line(&mut s1).unwrap();
        io::stdin().read_line(&mut s2).unwrap();
        println!("{}", two_strings(s1.trim(), s2.trim()));
        s1.clear();
        s2.clear();
    }
}

#[cfg(test)]
mod test {
    use crate::two_strings;

    #[test]
    fn test_sample(){
        assert_eq!(two_strings("and", "art"), "YES");
        assert_eq!(two_strings("be", "car"), "NO");
        assert_eq!(two_strings("hello", "world"), "YES");
        assert_eq!(two_strings("hellO", "worLd"), "YES");
        assert_eq!(two_strings("hi", "world"), "NO");
    }
}
