mod day02 {
    fn parse(input: &str) -> Vec<Vec<i32>> {
        input.lines()
            .map(|s| s.split('x').map(|v| v.parse().unwrap()).collect())
            .collect()
    }

    fn wrap(present: &[i32]) -> i32 {
        let faces = vec![
            present[0]*present[1],
            present[1]*present[2],
            present[0]*present[2],
        ];

        2*faces.iter().sum::<i32>() + faces.iter().min().unwrap()
    }

    pub fn solve_a(input: &str) -> i32 {
        let presents = parse(input);

        presents.iter()
            .map(|present| wrap(present))
            .sum()
    }

    fn ribbon(present: &[i32]) -> i32 {
        let mut present = present.to_vec();
        present.select_nth_unstable(1);

        2*(present[0] + present[1]) + present.iter().product::<i32>()
    }

    pub fn solve_b(input: &str) -> i32 {
        let presents = parse(input);

        presents.iter()
            .map(|present| ribbon(present))
            .sum()
    }

    #[cfg(test)]
    mod test_day02 {
        use super::*;

        #[test]
        fn test_wrap() {
            assert_eq!(wrap(&[2, 3, 4]), 58);
            assert_eq!(wrap(&[1, 1, 10]), 43);
        }

        #[test]
        fn test_ribbon() {
            assert_eq!(ribbon(&[2, 3, 4]), 34);
            assert_eq!(ribbon(&[1, 1, 10]), 14);
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

    println!("Solution A-part: {}", day02::solve_a(&buffer));
    println!("Solution B-part: {}", day02::solve_b(&buffer));
}
