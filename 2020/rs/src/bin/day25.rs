mod day25 {
    const SUBJECT: usize = 7;
    const BASE: usize = 20201227;

    pub fn solve(a: usize, b: usize) -> usize {
        let mut key = 1;
        let mut loop_size = 0;
        while key != a {
            key = (key * SUBJECT) % BASE;
            loop_size += 1;
        }

        let mut res = 1;
        for _ in 0..loop_size {
            res = (res * b) % BASE;
        }

        res
    }

    #[cfg(test)]
    mod test_day25 {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(5764801, 17807724), 14897079);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let (a, b) = {
        let ab: Vec<usize> = stdin.lock().lines()
            .map(|v| v.unwrap().parse().unwrap())
            .collect();
        (ab[0], ab[1])
    };

    println!("Solution: {}", day25::solve(a, b));
}
