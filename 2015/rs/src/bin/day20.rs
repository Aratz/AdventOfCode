mod day20 {
    fn div_sum(n: u64) -> u64 {
        (1..=(n as f32).sqrt().floor() as u64)
            .filter(|i| n % i == 0)
            .flat_map(|i| if i != n/i { vec![i, n/i] } else { vec![i] })
            .sum()
    }

    fn div_sum_filter(n: u64) -> u64 {
        (1..=(n as f32).sqrt().floor() as u64)
            .filter(|i| n % i == 0)
            .flat_map(|i| if i != n/i { vec![i, n/i] } else { vec![i] })
            .filter(|i| 50*i >= n)
            .sum()
    }

    pub fn solve_a(input: &str) -> u64 {
        let min_present = input.parse::<u64>().unwrap()/10;

        (1..).find(|&n| div_sum(n) >= min_present).unwrap()
    }

    pub fn solve_b(input: &str) -> u64 {
        let min_present = input.parse::<u64>().unwrap();

        (1..).find(|&n| 11 * div_sum_filter(n) >= min_present).unwrap()
    }

    #[cfg(test)]
    mod test_day20 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("80"), 6);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_a("80"), 6);
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

    println!("Solution A-part: {}", day20::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day20::solve_b(&buffer.trim()));
}
