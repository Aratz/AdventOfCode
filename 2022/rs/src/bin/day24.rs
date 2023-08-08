mod day24 {
    enum Cell {
        Blizzard(Vec<(i32, i32)>),
        Dist(i32),
        Wall,
        Empty,
    }

    fn parse(input: &str) -> Vec<Vec<Cell>> {
        input.lines()
            .map(|row| row.chars().map(|c| match c {
                '#' => Cell::Wall,
                '.' => Cell:: Empty,
                '>' => Cell::Blizzard(vec![(0, 1)]),
                'v' => Cell::Blizzard(vec![(1, 0)]),
                '<' => Cell::Blizzard(vec![(0, -1)]),
                '^' => Cell::Blizzard(vec![(-1, 0)]),
                _ => unreachable!(),
            }).collect())
            .collect()
    }

    fn neighbors(pos: (usize, usize), map: &[Vec<Cell>])
            -> Vec<(usize, usize)> {
        let (h, w) = (map.len(), map[0].len());

        let mut res = vec![pos];

        if pos.0 > 0 {
            res.push((pos.0 - 1, pos.1));
        }
        if pos.0 < h - 1 {
            res.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            res.push((pos.0, pos.1 - 1));
        }
        if pos.1 < w - 1 {
            res.push((pos.0, pos.1 + 1));
        }

        res
    }

    fn cross_map(
        start: (usize, usize),
        stop: (usize, usize),
        map: &mut Vec<Vec<Cell>>
    ) -> i32 {
        let (h, w) = (map.len(), map[0].len());

        for i in 0..h {
            for j in 0..w {
                if let Cell::Dist(_) = map[i][j] {
                    map[i][j] = Cell::Empty;
                }
            }
        }

        map[start.0][start.1] = Cell::Dist(0);

        loop {
            if let Cell::Dist(res) = map[stop.0][stop.1] {
                return res;
            }

            let mut new_map: Vec<Vec<Cell>> = map.iter()
                .map(|row| row.iter().map(|cell| match cell {
                    Cell::Wall => Cell::Wall,
                    _ => Cell::Empty,
                }).collect())
                .collect();

            for (i, row) in map.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    match cell {
                        Cell::Dist(dist) => {
                            for (i_next, j_next) in neighbors((i, j), &map) {
                                if let Cell::Empty = new_map[i_next][j_next] {
                                    new_map[i_next][j_next] = Cell::Dist(dist + 1);
                                }
                            }
                        },
                        Cell::Blizzard(v) => {
                            for &(di, dj) in v {
                                let (mut next_i, mut next_j) = (
                                    (i as i32 + di) as usize,
                                    (j as i32 + dj) as usize
                                );
                                if let Cell::Wall = new_map[next_i][next_j] {
                                    (next_i, next_j) = match (di, dj) {
                                        (-1, 0) => { (h - 2, next_j) },
                                        (1, 0) => { (1, next_j) },
                                        (0, -1) => { (next_i, w - 2) },
                                        (0, 1) => { (next_i, 1)},
                                        _ => unreachable!(),
                                    };
                                }

                                if let Cell::Blizzard(v) = &mut new_map[next_i][next_j] {
                                    v.push((di, dj));
                                }
                                else {
                                    new_map[next_i][next_j] = Cell::Blizzard(vec![(di, dj)]);
                                }
                            }
                        },
                        _ => {},
                    }
                }
            }

            *map = new_map;
        }

    }

    pub fn solve_a(input: &str) -> i32 {
        let mut map = parse(input);
        let (h, w) = (map.len(), map[0].len());

        cross_map((0, 1), (h - 1, w - 2), &mut map)
    }

    pub fn solve_b(input: &str) -> i32 {
        let mut map = parse(input);
        let (h, w) = (map.len(), map[0].len());
        let mut dist = 0;

        dist += cross_map((0, 1), (h - 1, w - 2), &mut map);
        dist += cross_map((h - 1, w - 2), (0, 1), &mut map);
        dist += cross_map((0, 1), (h - 1, w - 2), &mut map);
        dist
    }

    #[cfg(test)]
    mod test_day24 {
        use super::*;

        static INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 18);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 54);
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

    println!("Solution A-part: {}", day24::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day24::solve_b(&buffer.trim()));
}
