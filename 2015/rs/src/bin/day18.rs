mod day18 {
    use std::collections::HashSet;

    fn parse(input: &str) -> HashSet<(i32, i32)> {
        input.lines()
            .enumerate()
            .flat_map(|(i, row)| row.chars()
                 .enumerate()
                 .filter(|(_j, c)| *c == '#')
                 .map(move |(j, _c)| (i as i32, j as i32)))
            .collect()
    }

    fn get_neighbours(i: i32, j: i32, max_grid: i32) -> Vec<(i32, i32)> {
        vec![-1, 0, 1].into_iter()
            .flat_map(|di| vec![-1, 0, 1].into_iter()
                 .map(move |dj| (i + di, j + dj)))
            .filter(|(ni, nj)| (*ni != i || *nj != j)
                    && 0 <= *ni && *ni < max_grid
                    && 0 <= *nj && *nj < max_grid)
            .collect()
    }

    fn step(lights: &HashSet<(i32, i32)>, max_grid: i32, a: bool) -> HashSet<(i32, i32)> {
        let mut lights: HashSet<(i32, i32)> = (0..max_grid).flat_map(|i| (0..max_grid).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                let neighbours = get_neighbours(i, j, max_grid)
                    .into_iter()
                    .filter(|&(ni, nj)| lights.contains(&(ni, nj)))
                    .count();
                (lights.contains(&(i, j)) && neighbours == 2)
                    || neighbours == 3})
            .collect();

        if !a {
            lights.insert((0, 0));
            lights.insert((0, max_grid - 1));
            lights.insert((max_grid - 1, 0));
            lights.insert((max_grid - 1, max_grid - 1));
        }

        lights
    }

    pub fn solve_ab(input: &str, max_step: usize, max_grid: i32, a: bool) -> usize {
        let mut lights = parse(input);

        if !a {
            lights.insert((0, 0));
            lights.insert((0, max_grid - 1));
            lights.insert((max_grid - 1, 0));
            lights.insert((max_grid - 1, max_grid - 1));
        }

        for _ in 0..max_step {
            lights = step(&lights, max_grid, a);
        }

        lights.len()
    }

    #[cfg(test)]
    mod test_day18 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
            assert_eq!(solve_ab(&input, 4, 6, true), 4);
        }

        #[test]
        fn test_solve_b() {
            let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
            assert_eq!(solve_ab(&input, 5, 6, false), 17);
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

    println!("Solution A-part: {}", day18::solve_ab(&buffer.trim(), 100, 100, true));
    println!("Solution B-part: {}", day18::solve_ab(&buffer.trim(), 100, 100, false));
}
