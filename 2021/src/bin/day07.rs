mod day07 {
    use std::cmp::min;

    #[allow(non_snake_case)]
    fn minimal_cost(crabs: &[i32], cost: &dyn Fn(&[i32], i32) -> i32) -> i32 {
        let mut start = 0;
        let mut end = crabs.len() as i32;
        let mut m = ((end - start) / 3 + start) as i32;
        let mut M = (2 * (end - start) / 3 + start) as i32;

        while (M - m).abs() > 1 {
            if cost(crabs, m) <= cost(crabs, M) {
                end = M;
            }

            if cost(crabs, m) >= cost(crabs, M) {
                start = m;
            }

            m = (end - start) / 3 + start;
            M = 2 * (end - start) / 3 + start;
        }

        min(cost(crabs, m), cost(crabs, M))
    }

    pub fn solve_a(crabs: &[i32]) -> i32 {
        fn cost(crabs: &[i32], pos: i32) -> i32 {
            crabs.iter().map(|c| (c - pos).abs()).sum()
        }

        minimal_cost(crabs, &cost)
    }

    pub fn solve_b(crabs: &[i32]) -> i32 {
        fn cost(crabs: &[i32], pos: i32) -> i32 {
            crabs.iter().map(|c|{
                    let diff = (c - pos).abs();
                    diff * (diff + 1) / 2
                }).sum()
        }

        minimal_cost(crabs, &cost)
    }


    #[cfg(test)]
    mod test_day07 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(&vec![16,1,2,0,4,2,7,1,2,14]), 37);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(&vec![16,1,2,0,4,2,7,1,2,14]), 168);
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

    let crabs: Vec<i32> = buffer.trim()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    println!("Solution A-part: {}", day07::solve_a(&crabs));
    println!("Solution B-part: {}", day07::solve_b(&crabs));
}
