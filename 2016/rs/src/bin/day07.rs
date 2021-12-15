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

        #[allow(non_snake_case)]
        fn is_SSL(&self) -> bool {
            let abas: Vec<String> = self.no_bracket.iter()
                .map(|s| get_abas(s))
                .flatten()
                .collect();
            self.bracket.iter().any(|s| abas.iter().any(|aba| is_bab(s, aba)))
        }
    }

    fn is_abba(s: &str) -> bool {
        s.chars().zip(s.chars().skip(1)).zip(s.chars().skip(2).zip(s.chars().skip(3)))
            .find(|((a1, b1), (b2, a2))| a1 == a2 && b1 == b2 && a1 != b1).is_some()
    }

    fn get_abas(s: &str) -> Vec<String> {
        s.chars().zip(s.chars().skip(1).zip(s.chars().skip(2)))
            .filter(|(a1, (b, a2))| a1 == a2 && a1 != b)
            .map(|(a1, (b, a2))| vec![a1, b, a2].iter().collect())
            .collect()
    }

    fn is_bab(s: &str, aba: &str) -> bool {
        let v_aba: Vec<_> = aba.chars().collect();
        let bab: String = vec![v_aba[1], v_aba[0], v_aba[1]].iter().collect();
        s.find(&bab).is_some()
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

    pub fn solve_b(lines: &Vec<String>) -> usize {
        parse_ips(lines).iter()
            .filter(|ip| ip.is_SSL())
            .count()
    }

    #[cfg(test)]
    mod test_day07 {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn test_is_TLS() {
            assert!(Ipv7::new("abba[mnop]qrst").is_TLS());
            assert!(Ipv7::new("ioxxoj[asdfgh]zxcvbn").is_TLS());
            assert!(Ipv7::new("ioxxoj[asdfgh]zxcvbn[auneatuie]aunetau").is_TLS());

            assert!(!Ipv7::new("abcd[bddb]xyyx").is_TLS());
            assert!(!Ipv7::new("aaaa[qwer]tyui").is_TLS());
            assert!(!Ipv7::new("ioxxoj[asdfgh]zxcvbn[aunallaie]aunetau").is_TLS());
        }

        #[test]
        #[allow(non_snake_case)]
        fn test_is_SSL() {
            assert!(Ipv7::new("aba[bab]xyz").is_SSL());
            assert!(Ipv7::new("aaa[kek]eke").is_SSL());
            assert!(Ipv7::new("zazbz[bzb]cdb").is_SSL());
            assert!(Ipv7::new("xyx[xyx]xyx[yxy]auieaue").is_SSL());

            assert!(!Ipv7::new("xyx[xyx]xyx").is_SSL());
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
    println!("Solution B-part: {}", day07::solve_b(&lines));
}
