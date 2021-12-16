mod day22 {
    use std::collections::HashMap;

    struct Carrier {
        pos: (i32, i32),
        direction: i32, //0: up, 1: right, 2: down, 3: left
    }

    impl Carrier {
        fn step_a(&mut self, map: &mut HashMap<(i32, i32), char>) -> usize {
            let mut infect = 0;

            if map.contains_key(&self.pos) {
                self.direction += 1;
            }
            else {
                self.direction += 3;
            }

            self.direction %= 4;

            if !map.contains_key(&self.pos) {
                map.insert(self.pos, '#');
                infect += 1;
            }
            else {
                map.remove(&self.pos);
            }

            match self.direction {
                0 => { self.pos.0 -= 1; },
                1 => { self.pos.1 += 1; },
                2 => { self.pos.0 += 1; },
                3 => { self.pos.1 -= 1; },
                _ => unreachable!(),
            }

            infect
        }

        fn step_b(&mut self, map: &mut HashMap<(i32, i32), char>) -> usize {
            let mut infect = 0;

            match map.get(&self.pos) {
                Some('#') => { self.direction += 1; },
                Some('F') => { self.direction += 2; },
                Some('W') => {},
                None => { self.direction += 3; },
                _ => unreachable!(),
            }

            self.direction %= 4;

            match map.get(&self.pos) {
                Some('#') => { map.insert(self.pos, 'F'); },
                Some('F') => { map.remove(&self.pos); },
                Some('W') => { map.insert(self.pos, '#'); infect += 1; },
                None => { map.insert(self.pos, 'W'); },
                _ => unreachable!(),
            }

            match self.direction {
                0 => { self.pos.0 -= 1; },
                1 => { self.pos.1 += 1; },
                2 => { self.pos.0 += 1; },
                3 => { self.pos.1 -= 1; },
                _ => unreachable!(),
            }

            infect
        }
    }

    fn parse(init_map: &[Vec<char>]) -> HashMap<(i32, i32), char> {
        let max_i = init_map.len() as i32;
        let max_j = init_map[0].len() as i32;

        init_map.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate()
                      .filter(|(_j, node)| **node == '#')
                      .map(move |(j, _node)| (i as i32, j as i32)))
            .map(|(i, j)| ((i - max_i/2, j - max_j/2), '#'))
            .collect()
    }

    pub fn solve_a(init_map: &[Vec<char>], steps: usize) -> usize {
        let mut map = parse(init_map);

        let mut carrier = Carrier { pos: (0, 0), direction: 0 };

        let mut count = 0;

        for _ in 0..steps {
            count += carrier.step_a(&mut map);
        }

        count
    }

    pub fn solve_b(init_map: &[Vec<char>], steps: usize) -> usize {
        let mut map = parse(init_map);

        let mut carrier = Carrier { pos: (0, 0), direction: 0 };

        let mut count = 0;

        for _ in 0..steps {
            count += carrier.step_b(&mut map);
        }

        count
    }

    #[cfg(test)]
    mod test_day22 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let init_map: Vec<Vec<char>> = "..#
#..
...".lines().map(|l| l.chars().collect()).collect();

            assert_eq!(solve_a(&init_map, 1), 1);
            assert_eq!(solve_a(&init_map, 2), 1);
            assert_eq!(solve_a(&init_map, 6), 5);
            assert_eq!(solve_a(&init_map, 7), 5);
            assert_eq!(solve_a(&init_map, 70), 41);
            assert_eq!(solve_a(&init_map, 10000), 5587);
        }

        #[test]
        fn test_solve_b() {
            let init_map: Vec<Vec<char>> = "..#
#..
...".lines().map(|l| l.chars().collect()).collect();

            assert_eq!(solve_b(&init_map, 100), 26);
            assert_eq!(solve_b(&init_map, 10000000), 2511944);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let init_map: Vec<Vec<char>> = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    println!("Solution A-part: {}", day22::solve_a(&init_map, 10000));
    println!("Solution B-part: {}", day22::solve_b(&init_map, 10000000));
}
