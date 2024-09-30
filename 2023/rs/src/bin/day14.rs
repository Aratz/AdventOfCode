mod day14 {
    use std::collections::HashMap;

    type Platform = Vec<Vec<char>>;

    fn parse(input: &str) -> Platform {
        input.lines()
            .map(|l| l.chars().collect())
            .collect()
    }

    fn move_stone(
        platform: &mut Platform,
        stone: (i32, i32),
        direction: (i32, i32)
    ) {
        let (mut s_x, mut s_y) = stone;
        let (d_x, d_y) = direction;
        let (w, h) = (platform[0].len(), platform.len());

        while s_x + d_x >= 0 && ((s_x + d_x) as usize) < w
            && s_y + d_y >= 0 && ((s_y + d_y) as usize) < h
            && platform[(s_x + d_x) as usize][(s_y + d_y) as usize] == '.'
         {
            platform[s_x as usize][s_y as usize] = '.';
            (s_x, s_y) = (s_x + d_x, s_y + d_y);
            platform[s_x as usize][s_y as usize] = 'O';
        }
    }

    fn compute_load(platform: &Platform) -> usize {
        let h = platform.len();
        platform.iter().enumerate()
            .map(|(i, row)| (h-i)*row.iter().filter(|&&c| c == 'O').count())
            .sum()

    }


    pub fn solve_a(input: &str) -> usize {
        let mut platform = parse(input);
        let h = platform.len();
        let w = platform[0].len();

        for j in 0..w {
            for i in 0..h {
                if platform[i][j] == 'O' {
                    move_stone(&mut platform, (i as i32, j as i32), (-1, 0));
                }
            }
        }

        compute_load(&platform)
    }

    fn cycle(platform: &mut Platform) {
        let h = platform.len();
        let w = platform[0].len();
        // North
        for j in 0..w {
            for i in 0..h {
                if platform[i][j] == 'O' {
                    move_stone(platform, (i as i32, j as i32), (-1, 0));
                }
            }
        }

        // West
        for i in 0..h {
            for j in 0..w {
                if platform[i][j] == 'O' {
                    move_stone(platform, (i as i32, j as i32), (0, -1));
                }
            }
        }

        // South
        for j in 0..w {
            for i in (0..h).rev() {
                if platform[i][j] == 'O' {
                    move_stone(platform, (i as i32, j as i32), (1, 0));
                }
            }
        }

        // East
        for i in 0..h {
            for j in (0..w).rev() {
                if platform[i][j] == 'O' {
                    move_stone(platform, (i as i32, j as i32), (0, 1));
                }
            }
        }

    }

    fn compute_period(mut platform: &mut Platform) -> (usize, usize) {
        let mut states = HashMap::new();
        states.insert(platform.to_vec(), 0);

        for i in 1.. {
            cycle(&mut platform);
            if states.contains_key(platform) {
                let cycle_len = i - states[platform];
                let offset = states[platform];
                return (cycle_len, offset);
            }
            else {
                states.insert(platform.to_vec(), i);
            }
        }

        unreachable!();
    }

    pub fn solve_b(input: &str) -> usize {
        let mut platform = parse(input);

        let (cycle_len, offset) = compute_period(&mut platform);

        let n_cycle = (1_000_000_000 - offset) % cycle_len;

        for _ in 0..n_cycle {
            cycle(&mut platform);
        }

        compute_load(&platform)
    }

    #[cfg(test)]
    mod test_day14 {
        use super::*;

        static INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 136);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 64);
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

    println!("Solution A-part: {}", day14::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day14::solve_b(&buffer.trim()));
}
