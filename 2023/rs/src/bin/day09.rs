mod day09 {
    fn parse(input: &str) -> Vec<Vec<i32>> {
        input.lines()
            .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
            .collect()
    }

    fn next_value(values: &[i32]) -> i32 {
        if values.iter().all(|&n| n == 0) {
            0
        }
        else {
            let diffs: Vec<i32> = values.iter().zip(values.iter().skip(1))
                .map(|(v1, v2)| v2 - v1)
                .collect();

            values[values.len() - 1] + next_value(&diffs)
        }
    }

    fn previous_value(values: &[i32]) -> i32 {
        if values.iter().all(|&n| n == 0) {
            0
        }
        else {
            let diffs: Vec<i32> = values.iter().zip(values.iter().skip(1))
                .map(|(v1, v2)| v2 - v1)
                .collect();

            values[0] - previous_value(&diffs)
        }
    }

    pub fn solve_a(input: &str) -> i32 {
        let readings = parse(input);

        readings.into_iter()
            .map(|values| next_value(&values))
            .sum()
    }

    pub fn solve_b(input: &str) -> i32 {
        let readings = parse(input);

        readings.into_iter()
            .map(|values| previous_value(&values))
            .sum()
    }

    #[cfg(test)]
    mod test_day09 {
        use super::*;

        static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 114);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 2);
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

    println!("Solution A-part: {}", day09::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day09::solve_b(&buffer.trim()));
}
