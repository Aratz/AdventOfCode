mod day06 {
    use std::collections::HashSet;

    fn find_start(input: &str, len: usize) -> usize {
        let chars: Vec<char> = input.chars().collect();
        (0..).find(
            |&i| HashSet::<char>::from_iter(chars[i..i+len].iter().cloned()).len() == len)
            .unwrap()
            + len
    }

    pub fn solve_a(input: &str) -> usize {
        find_start(input, 4)
    }

    pub fn solve_b(input: &str) -> usize {
        find_start(input, 14)
    }

    #[cfg(test)]
    mod test_day06 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
            assert_eq!(solve_a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
            assert_eq!(solve_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
            assert_eq!(solve_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
            assert_eq!(solve_b("nppdvjthqldpwncqszvftbrmjlhg"), 23);
            assert_eq!(solve_b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
            assert_eq!(solve_b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
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

    println!("Solution A-part: {}", day06::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day06::solve_b(&buffer.trim()));
}
