mod day11 {
    use std::collections::HashSet;
    use std::cmp::{min, max};

    fn parse(input: &str) -> (HashSet<(i64, i64)>, HashSet<i64>, HashSet<i64>) {
        let galaxies: HashSet<(i64, i64)> = input.lines().enumerate()
            .flat_map(
                |(i, l)|
                l.chars().enumerate()
                    .filter(|&(_j, c)| c == '#')
                    .map(move |(j, _c)| (i as i64, j as i64))
            ).collect();

        let g_rows: HashSet<i64> = galaxies.iter().map(|&(i, _j)| i).collect();
        let empty_rows = (*g_rows.iter().min().unwrap()..*g_rows.iter().max().unwrap())
            .filter(|i| !g_rows.contains(i))
            .collect();

        let g_cols: HashSet<i64> = galaxies.iter().map(|&(_i, j)| j).collect();
        let empty_cols = (*g_cols.iter().min().unwrap()..*g_cols.iter().max().unwrap())
            .filter(|j| !g_cols.contains(j))
            .collect();

        (galaxies, empty_rows, empty_cols)
    }

    pub fn solve(input: &str, exp: i64) -> i64 {
        let (galaxies, empty_rows, empty_cols) = parse(input);

        galaxies.iter().map(
            |&(i1, j1)|
            galaxies.iter().map(
                |&(i2, j2)|
                (i2 - i1).abs()
                + (exp - 1) * (empty_rows.iter()
                    .filter(|&&i| min(i1, i2) < i && i < max(i1, i2)).count() as i64)
                + (j2 - j1).abs()
                + (exp - 1) * (empty_cols.iter()
                    .filter(|&&j| min(j1, j2) < j && j < max(j1, j2)).count() as i64)
            ).sum::<i64>()
        ).sum::<i64>() / 2
    }

    #[cfg(test)]
    mod test_day11 {
        use super::*;

        static INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve(INPUT, 2), 374);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve(INPUT, 10), 1030);
            assert_eq!(solve(INPUT, 100), 8410);
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

    println!("Solution A-part: {}", day11::solve(&buffer.trim(), 2));
    println!("Solution B-part: {}", day11::solve(&buffer.trim(), 1_000_000));
}
