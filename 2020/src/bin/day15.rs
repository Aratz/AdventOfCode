#![feature(test)]

extern crate test;

use std::io::{self, BufRead};

mod day15 {
    pub fn solve(n: usize, numbers: &Vec<usize>) -> usize {
        let mut last_spoken = vec![None; n];

        for (i, &n) in numbers.iter().take(numbers.len() - 1).enumerate() {
            last_spoken[n]  = Some(i + 1);
        }

        let mut last = *numbers.last().unwrap();

        for i in numbers.len()..n {
            let ith_spoken = match last_spoken[last] {
                Some(j) => { i - j },
                None => 0,
            };

            last_spoken[last] = Some(i);
            last = ith_spoken;
        }

        last
    }
}

fn main() {
    let stdin = io::stdin();

    let numbers: Vec<usize> = stdin.lock().lines().next().unwrap().unwrap()
        .split(",").map(|n| n.parse().unwrap()).collect();


    println!("Solution A-part: {}", day15::solve(2_020, &numbers));
    println!("Solution B-part: {}", day15::solve(30_000_000, &numbers));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solve_2020() {
        assert_eq!(day15::solve(2_020, &vec![0, 3, 6]), 436);

        assert_eq!(day15::solve(2_020, &vec![1, 3, 2]),    1);
        assert_eq!(day15::solve(2_020, &vec![2, 1, 3]),   10);
        assert_eq!(day15::solve(2_020, &vec![1, 2, 3]),   27);
        assert_eq!(day15::solve(2_020, &vec![2, 3, 1]),   78);
        assert_eq!(day15::solve(2_020, &vec![3, 2, 1]),  438);
        assert_eq!(day15::solve(2_020, &vec![3, 1, 2]), 1_836);
    }

    #[test]
    fn test_solve_3e7() {
        assert_eq!(day15::solve(30_000_000, &vec![0, 3, 6]),   175_594);

        assert_eq!(day15::solve(30_000_000, &vec![1, 3, 2]),     2_578);
        assert_eq!(day15::solve(30_000_000, &vec![2, 1, 3]), 3_544_142);
        assert_eq!(day15::solve(30_000_000, &vec![1, 2, 3]),   261_214);
        assert_eq!(day15::solve(30_000_000, &vec![2, 3, 1]), 6_895_259);
        assert_eq!(day15::solve(30_000_000, &vec![3, 2, 1]),        18);
        assert_eq!(day15::solve(30_000_000, &vec![3, 1, 2]),       362);
    }

    #[bench]
    fn bench_solve_1e5(b: &mut Bencher) {
        b.iter(|| day15::solve(300_000, &vec![0, 3, 6]));
    }
}
