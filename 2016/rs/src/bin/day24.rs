extern crate itertools;

mod day24 {
    use std::collections::{VecDeque, HashSet, HashMap};
    use itertools::Itertools;

    fn acquire_targets(grid: &[Vec<char>]) -> HashMap<char, (i32, i32)> {
        grid.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate()
                      .map(move |(j, loc)| (*loc, (i as i32, j as i32))))
            .filter(|&(loc, _ij)| loc.is_digit(10))
            .collect()
    }

    fn is_valid(pos: (i32, i32), grid: &[Vec<char>]) -> bool {
        let (i, j) = pos;
        0 <= i && i < grid.len() as i32
            && 0 <= j && j < grid[0].len() as i32
            && grid[i as usize][j as usize] != '#'
    }

    fn dist(a: (i32, i32), b: (i32, i32), grid: &[Vec<char>]) -> i32 {
        let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut queue = VecDeque::new();
        queue.push_back((a, 0));

        let mut visited = HashSet::new();

        while let Some((pos, d)) = queue.pop_front() {
            if visited.contains(&pos) { continue; }

            if pos == b { return d; }

            visited.insert(pos);

            let (i, j) = pos;
            for (di, dj) in &neighbors {
                if is_valid((i + di, j + dj), grid) {
                    queue.push_back(((i + di, j + dj), d + 1));
                }
            }
        }

        unreachable!()
    }

    pub fn solve_a(grid: &[Vec<char>]) -> i32 {
        let targets = acquire_targets(grid);

        let cross_dist: HashMap<(char, char), i32> = targets.keys()
            .flat_map(|a| targets.keys()
                      .map(|b| ((*a, *b), dist(targets[a], targets[b], grid))))
            .collect();

        targets.keys()
            .filter(|loc| **loc != '0')
            .permutations(targets.len() - 1)
            .map(|perm| cross_dist[&('0', *perm[0])]
                 + perm.iter().zip(perm.iter().skip(1))
                     .map(|(a, b)| cross_dist[&(**a, **b)])
                     .sum::<i32>())
            .min().unwrap()
    }

    pub fn solve_b(grid: &[Vec<char>]) -> i32 {
        let targets = acquire_targets(grid);

        let cross_dist: HashMap<(char, char), i32> = targets.keys()
            .flat_map(|a| targets.keys()
                      .map(|b| ((*a, *b), dist(targets[a], targets[b], grid))))
            .collect();

        targets.keys()
            .filter(|loc| **loc != '0')
            .permutations(targets.len() - 1)
            .map(|perm| cross_dist[&('0', *perm[0])]
                 + perm.iter().zip(perm.iter().skip(1))
                     .map(|(a, b)| cross_dist[&(**a, **b)])
                     .sum::<i32>()
                + cross_dist[&(*perm[perm.len() - 1], '0')])
            .min().unwrap()
    }

    #[cfg(test)]
    mod test_day24 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let grid: Vec<Vec<char>> = "###########\n#0.1.....2#\n#.#######.#\n#4.......3#\n###########".split('\n')
                .map(|l| l.chars().collect())
                .collect();

            assert_eq!(solve_a(&grid), 14);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let grid: Vec<Vec<char>> = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    println!("Solution A-part: {}", day24::solve_a(&grid));
    println!("Solution B-part: {}", day24::solve_b(&grid));
}
