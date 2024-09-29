mod day13 {
    fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
        input.split("\n\n")
            .map(
                |raw_pattern|
                raw_pattern.lines()
                    .map(|s| s.chars().collect())
                    .collect()
            )
            .collect()
    }

    fn transpose(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
        let w = pattern[0].len();

        (0..w)
            .map(|i| pattern.iter().map(|row| row[i]).collect())
            .collect()
    }

    fn reflexion_dist(s: &[char], axis: usize) -> usize {
        let (prefix, suffix) = (&s[0..axis], &s[axis..]);
        prefix.iter().rev()
            .zip(suffix.iter())
            .filter(|(c_p, c_s)| c_p != c_s)
            .count()
    }

    fn find_reflexion(pattern: &[Vec<char>], target_diff: usize) -> usize {
        let w = pattern[0].len();
        if let Some(axis) = (1..w).find(
            |&axis|
            pattern.iter()
                .map(|row| reflexion_dist(row, axis))
                .sum::<usize>() == target_diff
        ) {
            axis
        }
        else {
            let pattern = transpose(pattern);
            let h = pattern[0].len();
            (1..h).find(
                |&axis|
                pattern.iter()
                    .map(|column| reflexion_dist(column, axis))
                    .sum::<usize>() == target_diff
            ).unwrap() * 100
        }
    }

    pub fn solve_a(input: &str) -> usize {
        let patterns = parse(input);
        patterns.iter().map(|p| find_reflexion(p, 0)).sum()
    }

    pub fn solve_b(input: &str) -> usize {
        let patterns = parse(input);
        patterns.iter().map(|p| find_reflexion(p, 1)).sum()
    }

    #[cfg(test)]
    mod test_day13 {
        use super::*;

        static INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        #[test]
        fn test_reflextion_dist() {
            assert_eq!(reflexion_dist(&"...##..".chars().collect::<Vec<char>>(), 4), 0);
            assert_eq!(reflexion_dist(&"#..##..".chars().collect::<Vec<char>>(), 4), 0);
        }

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 405);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 400);
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

    println!("Solution A-part: {}", day13::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day13::solve_b(&buffer.trim()));
}
