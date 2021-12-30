mod day15 {
    use std::collections::{HashMap, BinaryHeap};
    use std::cmp::Reverse;

    fn parse_input(input: &str) -> Vec<Vec<i32>> {
        input.lines()
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
            .collect()
    }

    fn is_valid((i, j): (i32, i32), map: &[Vec<i32>]) -> bool {
        0 <= i  && i < map.len() as i32
            && 0 <= j && j < map[0].len() as i32
    }

    fn pathfinder(
        start: (i32, i32),
        end: (i32, i32),
        map: &[Vec<i32>]
        ) -> HashMap<(i32, i32), i32> {

        let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut queue = BinaryHeap::new();
        let mut visited = HashMap::new();

        queue.push((Reverse(0), start));

        while let Some((Reverse(d), pos)) = queue.pop() {
            if visited.contains_key(&pos) { continue; }

            visited.insert(pos, d);

            if pos == end { break; }

            for (di, dj) in &neighbors {
                let new_pos = (pos.0 + di, pos.1 + dj);
                if is_valid(new_pos, map) {
                    queue.push((
                            Reverse(d + map[new_pos.0 as usize][new_pos.1 as usize]),
                            new_pos));
                }
            }
        }

        visited

    }

    pub fn solve_a(input: &str) -> i32 {
        let risk_level = parse_input(input);

        let start = (0, 0);
        let end = (
            (risk_level.len() - 1) as i32,
            (risk_level[0].len() - 1) as i32,
            );

        pathfinder(start, end, &risk_level)[&end]
    }

    pub fn solve_b(input: &str) -> i32 {
        let risk_level_tile = parse_input(input);
        let max_i = risk_level_tile.len();
        let max_j = risk_level_tile[0].len();

        let risk_level = (0..5*max_i)
            .map(|i| (0..5*max_j)
                 .map(move |j| (i, j))
                 .map(|(i, j)| (
                         risk_level_tile[i % max_i][j % max_j]
                         + (i/max_i + j/max_j) as i32))
                 .map(|cost| if cost > 9 { cost % 9 } else { cost })
                 .collect::<Vec<i32>>())
            .collect::<Vec<_>>();

        let start = (0, 0);
        let end = (
            (risk_level.len() - 1) as i32,
            (risk_level[0].len() - 1) as i32,
            );

        pathfinder(start, end, &risk_level)[&end]
    }

    #[cfg(test)]
    mod test_day15 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

            assert_eq!(solve_a(&input), 40);

        }

        #[test]
        fn test_solve_b() {
            let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

            assert_eq!(solve_b(&input), 315);

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


    println!("Solution A-part: {}", day15::solve_a(&buffer));
    println!("Solution B-part: {}", day15::solve_b(&buffer));
}
