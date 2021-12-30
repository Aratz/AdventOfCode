mod day11 {
    use std::collections::{HashSet, VecDeque};

    pub fn solve_a(grid: &[Vec<i32>], steps: usize) -> usize {
        let neighboors:Vec<(i32, i32)> = (-1..=1)
            .flat_map(|i| (-1..=1).map(move |j| (i, j)))
            .filter(|&(i, j)| (i, j) != (0, 0))
            .collect();

        let mut count = 0;
        let mut grid = grid.to_vec();

        for _ in 0..steps {
            let mut flashing = HashSet::new();
            let mut to_increase: VecDeque<(i32, i32)> = (0..(grid.len() as i32))
                .flat_map(|i| (0..(grid[0].len() as i32)).map(move |j| (i, j)))
                .collect();

            while let Some((i, j)) = to_increase.pop_front() {
                if flashing.contains(&(i, j)) { continue; }

                grid[i as usize][j as usize] += 1;

                if grid[i as usize][j as usize] > 9 {
                    flashing.insert((i, j));

                    for (di, dj) in &neighboors {
                        if 0 <= i + di && i + di < grid.len() as i32
                        && 0 <= j + dj && j + dj < grid[0].len() as i32 {
                            to_increase.push_back((i + di, j + dj));
                        }
                    }
                }
            }

            count += flashing.len();
            for i in 0..grid.len() as i32 {
                for j in 0..grid[0].len() as i32 {
                    if flashing.contains(&(i, j)) {
                        grid[i as usize][j as usize] = 0;
                    }
                }
            }
        }

        count
    }

    pub fn solve_b(grid: &[Vec<i32>]) -> usize {
        let neighboors:Vec<(i32, i32)> = (-1..=1)
            .flat_map(|i| (-1..=1).map(move |j| (i, j)))
            .filter(|&(i, j)| (i, j) != (0, 0))
            .collect();

        let mut grid = grid.to_vec();

        for step in 0.. {
            let sync_val = grid[0][0];
            if grid.iter().flat_map(|row| row.iter()).all(|&v| v == sync_val) {
                return step;
            }

            let mut flashing = HashSet::new();
            let mut to_increase: VecDeque<(i32, i32)> = (0..(grid.len() as i32))
                .flat_map(|i| (0..(grid[0].len() as i32)).map(move |j| (i, j)))
                .collect();

            while let Some((i, j)) = to_increase.pop_front() {
                if flashing.contains(&(i, j)) { continue; }

                grid[i as usize][j as usize] += 1;

                if grid[i as usize][j as usize] > 9 {
                    flashing.insert((i, j));

                    for (di, dj) in &neighboors {
                        if 0 <= i + di && i + di < grid.len() as i32
                        && 0 <= j + dj && j + dj < grid[0].len() as i32 {
                            to_increase.push_back((i + di, j + dj));
                        }
                    }
                }
            }

            for i in 0..grid.len() as i32 {
                for j in 0..grid[0].len() as i32 {
                    if flashing.contains(&(i, j)) {
                        grid[i as usize][j as usize] = 0;
                    }
                }
            }
        }

        unreachable!();
    }

    #[cfg(test)]
    mod test_day11 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let grid = vec![
                vec![5,4,8,3,1,4,3,2,2,3],
                vec![2,7,4,5,8,5,4,7,1,1],
                vec![5,2,6,4,5,5,6,1,7,3],
                vec![6,1,4,1,3,3,6,1,4,6],
                vec![6,3,5,7,3,8,5,4,7,8],
                vec![4,1,6,7,5,2,4,6,4,5],
                vec![2,1,7,6,8,4,1,7,2,1],
                vec![6,8,8,2,8,8,1,1,3,4],
                vec![4,8,4,6,8,4,8,5,5,4],
                vec![5,2,8,3,7,5,1,5,2,6]
            ];

            assert_eq!(solve_a(&grid, 100), 1656);
        }

        #[test]
        fn test_solve_b() {
            let grid = vec![
                vec![5,4,8,3,1,4,3,2,2,3],
                vec![2,7,4,5,8,5,4,7,1,1],
                vec![5,2,6,4,5,5,6,1,7,3],
                vec![6,1,4,1,3,3,6,1,4,6],
                vec![6,3,5,7,3,8,5,4,7,8],
                vec![4,1,6,7,5,2,4,6,4,5],
                vec![2,1,7,6,8,4,1,7,2,1],
                vec![6,8,8,2,8,8,1,1,3,4],
                vec![4,8,4,6,8,4,8,5,5,4],
                vec![5,2,8,3,7,5,1,5,2,6]
            ];

            assert_eq!(solve_b(&grid), 195);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let grid: Vec<Vec<i32>> = stdin.lock().lines()
        .map(|l| l.unwrap().chars()
             .map(|c| c.to_digit(10).unwrap() as i32)
             .collect::<Vec<_>>())
        .collect();

    println!("Solution A-part: {}", day11::solve_a(&grid, 100));
    println!("Solution B-part: {}", day11::solve_b(&grid));
}

