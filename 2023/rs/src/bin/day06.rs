extern crate regex;

mod day06 {
    use regex::Regex;
    use itertools::Itertools;

    fn parse_a(input: &str) -> (Vec<usize>, Vec<usize>) {
        let re_nums = Regex::new(r"\d+").unwrap();
        let mut lines = input.lines();

        (
            re_nums.find_iter(lines.next().unwrap())
                .map(|mch| mch.as_str().parse().unwrap())
                .collect(),
            re_nums.find_iter(lines.next().unwrap())
                .map(|mch| mch.as_str().parse().unwrap())
                .collect()
        )
    }

    fn parse_b(input: &str) -> (usize, usize) {
        let re_nums = Regex::new(r"\d+").unwrap();
        let mut lines = input.lines();
        let res = (
            re_nums.find_iter(lines.next().unwrap())
                .map(|mch| mch.as_str().to_string())
                .join("").parse().unwrap(),
            re_nums.find_iter(lines.next().unwrap())
                .map(|mch| mch.as_str().to_string())
                .join("").parse().unwrap()
        );
        res
    }

    fn solve_race(max_t: usize, min_d: usize) -> usize {
        let sqrt_delta = ((max_t*max_t - 4*min_d) as f64).sqrt();
        let eps = 1e-10;
        let roots = (
            ((1. + eps)*(max_t as f64 - sqrt_delta)/2.).ceil() as usize,
            ((1. - eps)*(max_t as f64 + sqrt_delta)/2.).floor() as usize
        );

        roots.1 - roots.0 + 1
    }

    pub fn solve_a(input: &str) -> usize {
        let (times, dists) = parse_a(input);

        times.into_iter().zip(dists.into_iter())
            .map(|(max_t, min_d)| solve_race(max_t, min_d))
            .product()
    }

    pub fn solve_b(input: &str) -> usize {
        let (max_t, min_d) = parse_b(input);

        solve_race(max_t, min_d)
    }

    #[cfg(test)]
    mod test_day06 {
        use super::*;

        static INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 288);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 71503);
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
