extern crate itertools;

mod day11 {
    use itertools::Itertools;

    const MAXCH: u8 = 26;
    const PSSLN: usize = 8;

    fn ord(c: char) -> u8 {
        c as u8 - 'a' as u8
    }

    type Password = [u8; PSSLN];

    fn parse(input: &str) -> Password {
        input.chars()
            .map(|c| ord(c))
            .collect::<Vec<u8>>().try_into().unwrap()
    }

    fn to_string(password: &Password) -> String {
        password.iter()
            .map(|c| (c + 'a' as u8) as char)
            .collect()
    }

    fn has_inc(password: &Password) -> bool {
        password.iter()
            .zip(password.iter().skip(1).zip(password.iter().skip(2)))
            .any(|(c1, (&c2, &c3))| c1 + 1 == c2 && c2 + 1 == c3)
    }

    fn no_iol(password: &Password) -> bool {
        password.iter()
            .all(|&c| ['i', 'l', 'o'].iter().all(|&iol| ord(iol) != c))
    }

    fn has_pairs(password: &Password) -> bool {
        password.iter()
            .dedup_with_count()
            .filter_map(|(count, _)| if count == 2 || count >= 4 {
                Some(count/2)
            } else { None})
            .sum::<usize>() >= 2
    }

    struct PassIter {
        curr: Password,
    }

    impl PassIter {
        fn new(u0: &Password) -> Self {
            PassIter { curr: u0.clone() }
        }
    }

    fn inc(v: &mut u8) {
        *v += 1;
        if vec![ord('i'), ord('o'), ord('l')].contains(v) {
            *v += 1;
        }
    }

    impl Iterator for PassIter {
        type Item = Password;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr.clone();

            inc(&mut self.curr[PSSLN - 1]);
            for i in 0..PSSLN {
                let idx = PSSLN - 1 -i;
                if self.curr[idx] >= MAXCH {
                    self.curr[idx] %= MAXCH;

                    if idx > 0 {
                        inc(&mut self.curr[idx - 1]);
                    }
                }
                else { break; }
            }

            Some(current)
        }
    }

    pub fn solve_a(input: &str) -> String {
        let password = parse(input);
        to_string(&PassIter::new(&password)
            .skip(1)
            .filter(|p| has_inc(p))
            .filter(|p| no_iol(p))
            .filter(|p| has_pairs(p))
            .next().unwrap())
    }

    pub fn solve_b(input: &str) -> String {
        let next_pass = solve_a(input);
        solve_a(&next_pass)
    }

    #[cfg(test)]
    mod test_day11 {
        use super::*;

        #[test]
        fn test_has_inc() {
            assert!(has_inc(&parse("hijklmmn")));

            assert!(!has_inc(&parse("abbceffg")));
        }

        #[test]
        fn test_no_iol() {
            assert!(!no_iol(&parse("hijklmmn")));

            assert!(no_iol(&parse("abbceffg")));
        }

        #[test]
        fn test_has_pairs() {
            assert!(!has_pairs(&parse("hijklmmn")));
            assert!(!has_pairs(&parse("abbcegjk")));

            assert!(has_pairs(&parse("abbceffg")));
        }

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("abcdefgh"), String::from("abcdffaa"));
            assert_eq!(solve_a("ghijklmn"), String::from("ghjaabcc"));
        }

        #[test]
        fn test_solve_b() {

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

    println!("Solution A-part: {}", day11::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day11::solve_b(&buffer.trim()));
}
