extern crate regex;

mod day07 {
    use regex::Regex;

    struct Ipv7 {
        no_bracket: Vec<String>,
        bracket: Vec<String>,
    }

    impl Ipv7 {
        fn new(s: &str) -> Self {
            let reg_ipv7 = Regex::new(r"[a-z]+").unwrap();
            let mut no_bracket: Vec<String> = Vec::new();
            let mut bracket: Vec<String> = Vec::new();

            for (i, capt) in reg_ipv7.captures_iter(s).enumerate() {
                if i % 2 == 0 {
                    no_bracket.push(capt.get(0).unwrap().as_str().into());
                }
                else {
                    bracket.push(capt.get(0).unwrap().as_str().into());
                }
            }
            Ipv7 { bracket, no_bracket }
        }

        #[allow(non_snake_case)]
        fn is_TLS(&self) -> bool {
            self.no_bracket.iter().any(|s| is_abba(s))
                && !self.bracket.iter().any(|s| is_abba(s))
        }
    }

    fn is_abba(s: &str) -> bool {
        s.chars().zip(s.chars().skip(1)).zip(s.chars().skip(2).zip(s.chars().skip(3)))
            .filter(|((a1, b1), (b2, a2))| a1 == a2 && b1 == b2 && a1 != b1)
            .next().is_some()
    }

    fn parse_ips(lines: &Vec<String>) -> Vec<Ipv7> {
        lines.iter()
            .map(|line| Ipv7::new(line))
            .collect()
    }

    pub fn solve_a(lines: &Vec<String>) -> usize {
        parse_ips(lines).iter()
            .filter(|ip| ip.is_TLS())
            .count()
    }

    #[cfg(test)]
    mod test_day07 {
        use super::*;

        #[test]
        fn test_is_TLS() {
            assert!(Ipv7::new("abba[mnop]qrst").is_TLS());
            assert!(Ipv7::new("ioxxoj[asdfgh]zxcvbn").is_TLS());
            assert!(Ipv7::new("ioxxoj[asdfgh]zxcvbn[auneatuie]aunetau").is_TLS());

            assert!(!Ipv7::new("abcd[bddb]xyyx").is_TLS());
            assert!(!Ipv7::new("aaaa[qwer]tyui").is_TLS());
            assert!(!Ipv7::new("ioxxoj[asdfgh]zxcvbn[aunallaie]aunetau").is_TLS());
        }
    }

}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<_> = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect();

    println!("Solution A-part: {}", day07::solve_a(&lines));
}
