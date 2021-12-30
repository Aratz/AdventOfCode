mod day06 {
    pub fn solve_ab(fish: &[usize], days: usize) -> u64 {
        let mut adults = vec![0; 9];
        let mut youngs = vec![0; 9];

        for &f in fish {
            adults[f + 2] += 1;
        }

        for _ in 0..days {
            let i = adults.len();
            adults.push(adults[i - 7] + youngs[i - 9]);
            youngs.push(adults[i - 7] + youngs[i - 9]);
        }

        let len = adults.len();
        adults[len - 7..len].iter().sum::<u64>()
            + youngs[len - 9..len].iter().sum::<u64>()
    }

    #[cfg(test)]
    mod test_day06 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let fish = vec![3, 4, 3, 1, 2];

            assert_eq!(solve_ab(&fish, 80), 5934);
        }

        #[test]
        fn test_solve_b() {
            let fish = vec![3, 4, 3, 1, 2];

            assert_eq!(solve_ab(&fish, 256), 26984457539);
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

    let fish: Vec<usize> = buffer.trim().split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    println!("Solution A-part: {}", day06::solve_ab(&fish, 80));
    println!("Solution B-part: {}", day06::solve_ab(&fish, 256));
}
