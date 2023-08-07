extern crate regex;
extern crate itertools;

mod day22 {
    use regex::Regex;
    use itertools::Itertools;
    use std::collections:: HashMap;

    type Map = Vec<Vec<char>>;

    #[derive(Debug)]
    enum Turn {
        Left,
        Right,
    }

    #[derive(Debug)]
    enum Instruction {
        Turn(Turn),
        Move(usize),
    }

    fn parse(input: &str) -> (Map, Vec<Instruction>) {
        let raw_map_inst: Vec<_> = input.split("\n\n").collect();
        let (raw_map, raw_inst) = (raw_map_inst[0], raw_map_inst[1]);

        let width = raw_map.lines()
            .map(|l| l.len())
            .max().unwrap();

        let mut map: Map = raw_map.lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .map(|mut v| {
                v.resize(width, ' ');
                v.insert(0, ' '); v.push(' ');
                v
            })
            .collect();

        map.insert(0, vec![' '; width + 2]);
        map.push(vec![' '; width + 2]);

        let re_moves = Regex::new(r"[LR]").unwrap();
        let moves = re_moves.split(raw_inst)
            .map(|mv| Instruction::Move(mv.parse().unwrap()));

        let re_turns = Regex::new(r"\d+").unwrap();
        let turn = re_turns.split(raw_inst)
            .filter_map(|c| match c {
                "L" => Some(Instruction::Turn(Turn::Left)),
                "R" => Some(Instruction::Turn(Turn::Right)),
                _ => None,
            });

        let inst: Vec<Instruction> = moves.interleave(turn).collect();

        assert_eq!(raw_inst.to_string(), inst.iter().map(
            |ins| match ins {
                Instruction::Move(n) => n.to_string(),
                Instruction::Turn(turn) => match turn {
                    Turn::Left => "L".to_string(),
                    Turn::Right => "R".to_string(),
                },
            }).collect::<String>());

        (map, inst)
    }

    #[inline]
    fn find_next_step(pos: (usize, usize), dir: usize, map: &Map) -> (usize, usize) {
        let next_pos = match dir {
            0 => (pos.0, pos.1 + 1),
            1 => (pos.0 + 1, pos.1),
            2 => (pos.0, pos.1 - 1),
            3 => (pos.0 - 1, pos.1),
            _ => unreachable!(),
        };

        match map[next_pos.0][next_pos.1] {
            '.' => next_pos,
            '#' => pos,
            ' ' => {
                let flip_pos = match dir {
                    0 => (
                        next_pos.0,
                        map[next_pos.0].iter()
                            .position(|&c| c != ' ').unwrap()
                    ),
                    1 => (
                        map.iter()
                            .map(|row| row[next_pos.1])
                            .position(|c| c != ' ').unwrap(),
                        next_pos.1,
                    ),
                    2 => (
                        next_pos.0,
                        map[next_pos.0].iter()
                            .rposition(|&c| c != ' ').unwrap()
                    ),
                    3 => (
                        map.iter()
                            .map(|row| row[next_pos.1])
                            .rposition(|c| c != ' ').unwrap(),
                        next_pos.1,
                    ),
                    _ => unreachable!(),
                };

                if map[flip_pos.0][flip_pos.1] == '#' { pos }
                else { flip_pos }
            },
            _ => unreachable!(),
        }
    }

    pub fn solve_a(input: &str) -> usize {
        let (map, instructions) = parse(input);

        let mut pos = (1, map[1].iter().position(|&c| c == '.').unwrap());
        let mut dir = 0;

        for inst in instructions {
            match inst {
                Instruction::Turn(lr) => match lr {
                    Turn::Left => { dir = (dir + 3) % 4; },
                    Turn::Right => { dir = (dir + 1) % 4; },
                },
                Instruction::Move(n_steps) => {
                    for _ in 0..n_steps {
                        let next_pos = find_next_step(pos, dir, &map);

                        if next_pos == pos { break; }
                        else { pos = next_pos; }
                    }
                },
            }
        }

        1000*pos.0 + 4*pos.1 + dir
    }

    #[inline]
    fn flip_backslash(rel_pos: (usize, usize), _size: usize) -> (usize, usize) {
        (rel_pos.1, rel_pos.0)
    }

    #[inline]
    fn flip_slash(rel_pos: (usize, usize), size: usize) -> (usize, usize) {
        (size -1 - rel_pos.1, size - 1 - rel_pos.0)
    }

    #[inline]
    fn flip_dash(rel_pos: (usize, usize), size: usize) -> (usize, usize) {
        (size - 1 - rel_pos.0, rel_pos.1)
    }

    #[inline]
    fn flip_column(rel_pos: (usize, usize), size: usize) -> (usize, usize) {
        (rel_pos.0, size - 1 - rel_pos.1)
    }

    #[inline]
    fn find_next_step_b(
        pos: (usize, usize),
        dir: usize,
        map: &Map,
        faces: &HashMap<(usize, usize), [((usize, usize), usize); 4]>,
        size: usize,
    ) -> ((usize, usize), usize) {
        let next_pos = match dir {
            0 => (pos.0, pos.1 + 1),
            1 => (pos.0 + 1, pos.1),
            2 => (pos.0, pos.1 - 1),
            3 => (pos.0 - 1, pos.1),
            _ => unreachable!(),
        };

        match map[next_pos.0][next_pos.1] {
            '.' => (next_pos, dir),
            '#' => (pos, dir),
            ' ' => {
                let quadrant = ((pos.0 - 1) / size, (pos.1 - 1) / size);
                let rel_pos  = ((pos.0 - 1) % size, (pos.1 - 1) % size);
                let (next_face, new_dir) = faces[&quadrant][dir];

                let new_rel_pos = match dir {
                    x if x == new_dir => {
                        if new_dir % 2 == 0 {
                            flip_column(rel_pos, size)
                        }
                        else {
                            flip_dash(rel_pos, size)
                        }
                    },
                    x if x == (new_dir + 1)%4 || x == (new_dir + 3)%4 => {
                        match (dir + new_dir) % 4 {
                            1 => {
                                flip_slash(rel_pos, size)
                            },
                            3 => {
                                flip_backslash(rel_pos, size)
                            },
                            _ => unreachable!(),
                        }
                    },
                    x if x == (new_dir + 2)%4 => {
                        if new_dir % 2 == 0 {
                            flip_dash(rel_pos, size)
                        }
                        else {
                            flip_column(rel_pos, size)
                        }
                    },
                    _ => unreachable!(),
                };

                let new_pos = (
                    next_face.0 * size + new_rel_pos.0 + 1,
                    next_face.1 * size + new_rel_pos.1 + 1
                );

                if map[new_pos.0][new_pos.1] == '#' { (pos, dir) }
                else { (new_pos, new_dir) }
            },
            _ => unreachable!(),
        }
    }


    pub fn solve_b(
        input: &str,
        faces: &HashMap<(usize, usize), [((usize, usize), usize); 4]>,
        size: usize,
    ) -> usize {
        let (map, instructions) = parse(input);

        let mut pos = (1, map[1].iter().position(|&c| c == '.').unwrap());
        let mut dir = 0;

        for inst in instructions {
            match inst {
                Instruction::Turn(lr) => match lr {
                    Turn::Left => { dir = (dir + 3) % 4; },
                    Turn::Right => { dir = (dir + 1) % 4; },
                },
                Instruction::Move(n_steps) => {
                    for _ in 0..n_steps {
                        let (next_pos, next_dir) = find_next_step_b(
                            pos,
                            dir,
                            &map,
                            &faces,
                            size);

                        if next_pos == pos { break; }
                        else { (pos, dir) = (next_pos, next_dir); }
                    }
                },
            }
        }

        1000*pos.0 + 4*pos.1 + dir
    }

    #[cfg(test)]
    mod test_day22 {
        use super::*;

        static INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        #[test]
        fn test_flip_slash() {
            assert_eq!(flip_slash((1, 3), 4), (0, 2));
        }

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 6032);
        }

        #[test]
        fn test_solve_b() {
            let faces = vec![
                ((0, 2), [
                 ((2, 3), 2),
                 ((1, 2), 1),
                 ((1, 1), 1),
                 ((1, 0), 1),
                ]),
                ((1, 0), [
                 ((1, 1), 0),
                 ((2, 2), 3),
                 ((2, 3), 3),
                 ((0, 2), 1),
                ]),
                ((1, 1), [
                 ((1, 2), 0),
                 ((2, 2), 0),
                 ((1, 0), 2),
                 ((0, 2), 0),
                ]),
                ((1, 2), [
                 ((2, 3), 1),
                 ((2, 2), 1),
                 ((1, 1), 2),
                 ((0, 2), 3),
                ]),
                ((2, 2), [
                 ((2, 3), 0),
                 ((1, 0), 3),
                 ((1, 1), 3),
                 ((1, 2), 3),
                ]),
                ((2, 3), [
                 ((0, 2), 2),
                 ((1, 0), 0),
                 ((2, 2), 2),
                 ((1, 2), 2),
                ]),
            ].into_iter().collect();

            assert_eq!(solve_b(INPUT, &faces, 4), 5031);
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

    println!("Solution A-part: {}", day22::solve_a(&buffer.trim_end()));

    let faces = vec![
        ((0, 1), [
         ((0, 2), 0),
         ((1, 1), 1),
         ((2, 0), 0),
         ((3, 0), 0),
        ]),
        ((0, 2), [
         ((2, 1), 2),
         ((1, 1), 2),
         ((0, 1), 2),
         ((3, 0), 3),
        ]),
        ((1, 1), [
         ((0, 2), 3),
         ((2, 1), 1),
         ((2, 0), 1),
         ((0, 1), 3),
        ]),
        ((2, 0), [
         ((2, 1), 0),
         ((3, 0), 1),
         ((0, 1), 0),
         ((1, 1), 0),
        ]),
        ((2, 1), [
         ((0, 2), 2),
         ((3, 0), 2),
         ((2, 0), 2),
         ((1, 1), 3),
        ]),
        ((3, 0), [
         ((2, 1), 3),
         ((0, 2), 1),
         ((0, 1), 1),
         ((2, 0), 3),
        ]),
    ].into_iter().collect();

    println!("Solution B-part: {}", day22::solve_b(&buffer.trim_end(), &faces, 50));
}
