mod day05 {
    use std::collections::HashSet;

    fn is_nice_a(s: &str) -> bool {
        let mut vowels = 0;
        let mut dup = false;

        let mut last = 'a';

        for (a, b) in s.chars().zip(s.chars().skip(1)) {
            if "aeiou".contains(&a.to_string()) {
                vowels += 1;
            }
            last = b;

            dup = dup || a == b;

            if [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')].contains(&(a, b)) {
                return false;
            }
        }

        if "aeiou".contains(&last.to_string()) {
            vowels += 1;
        }

        return vowels >= 3 && dup
    }

    pub fn solve_a(input: &str) -> usize {
        input.lines()
            .filter(|s| is_nice_a(&s))
            .count()
    }

    fn contains_pair(s: &str) -> bool {
        let mut pairs = HashSet::new();
        let mut double = false;

        for (a, b) in s.chars()
            .zip(s.chars().skip(1)) {
                if double && a == b {
                    double = false;
                    continue;
                }
                else {
                    double = a == b;
                }

                if pairs.contains(&(a, b)) { return true; }

                pairs.insert((a, b));
            }

        false
    }

    #[inline]
    fn contains_triplet(s: &str) -> bool {
        s.chars()
            .zip(s.chars().skip(2))
            .find(|(a, b)| a == b)
            .is_some()
    }

    #[inline]
    fn is_nice_b(s: &str) -> bool {
        contains_pair(s) && contains_triplet(s)
    }

    pub fn solve_b(input: &str) -> usize {
        input.lines()
            .filter(|s| is_nice_b(&s))
            .count()
    }
    //52 wrong

    #[cfg(test)]
    mod test_day05 {
        use super::*;

        #[test]
        fn test_is_nice_a() {
            assert!(is_nice_a("ugknbfddgicrmopn"));
            assert!(is_nice_a("aaa"));
            assert!(!is_nice_a("jchzalrnumimnmhp"));
            assert!(!is_nice_a("haegwjzuvuyypxyu"));
            assert!(!is_nice_a("dvszwmarrgswjxmb"));

        }

        #[test]
        fn test_is_nice_b() {
            assert!(is_nice_b("qjhvhtzxzqqjkmpb"));
            assert!(is_nice_b("xxyxx"));
            assert!(!is_nice_b("uurcxstgmygtbstg"));
            assert!(!is_nice_b("ieodomkazucvgmuy"));

        }
    }

}

fn main() {
    use std::io::{self, Read};

    let stdin = io::stdin();

    let mut buffer = String::new();

    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    println!("Solution A-part: {}", day05::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day05::solve_b(&buffer.trim()));
}
