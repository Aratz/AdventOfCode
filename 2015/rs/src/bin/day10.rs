extern crate itertools;

mod day10 {
    use itertools::Itertools;

    fn parse(input: &str) -> Vec<usize> {
        input.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>()
    }

    fn iterate(seq: &[usize]) -> Vec<usize> {
        seq.iter()
            .dedup_with_count()
            .flat_map(|(count, &ch)| vec![count, ch])
            .collect()
    }

    pub fn solve_ab(input: &str, n: usize) -> usize {
        let mut seq = parse(input);

        for _ in 0..n {
            seq = iterate(&seq);
        }

        seq.len()
    }

    #[cfg(test)]
    mod test_day10 {
        use super::*;

        #[test]
        fn test_iterate() {
            assert_eq!(iterate(&[1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
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

    println!("Solution A-part: {}", day10::solve_ab(&buffer.trim(), 40));
    println!("Solution B-part: {}", day10::solve_ab(&buffer.trim(), 50));
}
