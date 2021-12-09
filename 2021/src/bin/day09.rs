mod day09 {
    use std::collections::{VecDeque, HashSet};

    fn find_lows(map: &[Vec<i32>]) -> Vec<(usize, usize)> {
        let neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        map.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate()
                      .map(move |(j, _h)| (i, j))
                      .filter(|(i, j)| neighbors.iter()
                           .map(|(di, dj)| (*i as i32 + di, *j as i32 + dj))
                           .filter(|&(i_n, j_n)|
                                   0 <= i_n && i_n < map.len() as i32
                                && 0 <= j_n && j_n < map[0].len() as i32)
                           .all(|(i_n, j_n)| map[*i][*j] < map[i_n as usize][j_n as usize])))
            .collect()
    }

    fn fill_basin(low: (usize, usize), map: &[Vec<i32>]) -> usize {
        let neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(low);

        while let Some((i, j)) = queue.pop_front() {
            if map[i][j] == 9
                || visited.contains(&(i, j)) {
                continue;
            }

            visited.insert((i, j));
            for (i_n, j_n) in neighbors.iter()
                           .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                           .filter(|&(i_n, j_n)|
                                   0 <= i_n && i_n < map.len() as i32
                                && 0 <= j_n && j_n < map[0].len() as i32)
                           .map(|(i_n, j_n)| (i_n as usize, j_n as usize)) {
                queue.push_back((i_n, j_n));
            }
        }

        visited.len()
    }

    pub fn solve_a(map: &[Vec<i32>]) -> i32 {
        find_lows(map).iter().map(|(i, j)| map[*i][*j] + 1).sum()
    }

    pub fn solve_b(map: &[Vec<i32>]) -> usize {
        let mut basins: Vec<usize> = find_lows(map).iter()
            .map(|&low| fill_basin(low, map)).collect();
        basins.sort_unstable();
        basins.reverse();

        basins.iter().take(3).product()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let map = stdin.lock().lines()
        .map(|l| l.unwrap().chars()
             .map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    println!("Solution A-part: {}", day09::solve_a(&map));
    println!("Solution B-part: {}", day09::solve_b(&map));
}
