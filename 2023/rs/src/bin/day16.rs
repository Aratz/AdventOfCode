mod day16 {
    use std::collections::HashSet;

    fn parse(input: &str) -> Vec<Vec<char>> {
        input.lines().map(
            |l| l.chars().collect()
        ).collect()
    }

    fn push(
        queue: &mut Vec<((i32, i32), (i32, i32))>,
        x: i32, y: i32, h: i32, w: i32, dir: (i32, i32),
    ) {
        if 0 <= x && x < h && 0 <= y && y < w {
            queue.push(((x, y), dir));
        }
    }

    fn energize(grid: &[Vec<char>], start: (i32, i32), dir: (i32, i32)) -> usize {
        let (h, w) = (grid.len() as i32, grid[0].len() as i32);
        let mut visited = vec![vec![HashSet::new(); w as usize]; h as usize];

        let mut queue = vec![(start, dir)];

        while let Some(((x, y), (dx, dy))) = queue.pop() {
            if visited[x as usize][y as usize].contains(&(dx, dy)) { continue; }

            visited[x as usize][y as usize].insert((dx, dy));
            match grid[x as usize][y as usize] {
                '.' => {
                    push(&mut queue, x + dx, y + dy, h, w, (dx, dy));
                },
                '/' => {
                    let (dx, dy) = (-dy, -dx);
                    push(&mut queue, x + dx, y + dy, h, w, (dx, dy));
                },
                '\\' => {
                    let (dx, dy) = (dy, dx);
                    push(&mut queue, x + dx, y + dy, h, w, (dx, dy));
                },
                '|' => {
                    if dy == 0 {
                        push(&mut queue, x + dx, y + dy, h, w, (dx, dy));
                    }
                    else {
                        let (dx, dy) = (1, 0);
                        push(&mut queue, x + dx, y + dy, h, w, (dx, dy));

                        let (dx, dy) = (-1, 0);
                        push(&mut queue, x + dx, y + dy, h, w, (dx, dy));
                    }
                },
                '-' => {
                    if dx == 0 {
                        push(&mut queue, x + dx, y + dy, h, w, (dx, dy));
                    }
                    else {
                        let (dx, dy) = (0, 1);
                        push(&mut queue, x + dx, y + dy, h, w, (dx, dy));

                        let (dx, dy) = (0, -1);
                        push(&mut queue, x + dx, y + dy, h, w, (dx, dy));
                    }
                },
                _ => unreachable!(),
            }
        }


        return visited.into_iter().map(
            |row| row.into_iter().filter(|c| !c.is_empty()).count()
        ).sum()
    }

    pub fn solve_a(input: &str) -> usize {
        let grid = parse(input);
        energize(&grid, (0, 0), (0, 1))
    }

    pub fn solve_b(input: &str) -> usize {
        let grid = parse(input);
        let (h, w) = (grid.len() as i32, grid[0].len() as i32);

        (0..h).map(|x| ((x, 0), (0, 1)))
            .chain((0..h).map(|x| ((x, w - 1), (0, -1))))
            .chain((0..w).map(|y| ((0, y), (1, 0))))
            .chain((0..w).map(|y| ((h - 1, y), (-1, 0))))
            .map(|(start, dir)| energize(&grid, start, dir))
            .max().unwrap()
    }

    #[cfg(test)]
    mod test_day16 {
        use super::*;

        static INPUT: &str = r##".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."##;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 46);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 51);
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

    println!("Solution A-part: {}", day16::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day16::solve_b(&buffer.trim()));
}
