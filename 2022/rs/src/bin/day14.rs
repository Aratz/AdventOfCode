mod day14 {
    use std::collections::HashMap;
    use std::cmp::{min, max};

    #[inline]
    fn parse_corner(input: &str) -> (i32, i32) {
        let coord: Vec<i32> = input.split(',').map(|v| v.parse().unwrap()).collect();

        (coord[0], coord[1])
    }

    fn parse(input: &str) -> HashMap<(i32, i32), char> {
        input.lines().flat_map(
            |l| l.split(" -> ").zip(l.split(" -> ").skip(1))
                .map(|raw_corners| (parse_corner(raw_corners.0), parse_corner(raw_corners.1)))
                .flat_map(|corners| {
                    if corners.0.0 != corners.1.0 {
                        let (min_c, max_c) = (
                            min(corners.0.0, corners.1.0),
                            max(corners.0.0, corners.1.0)
                            );
                        (min_c..=max_c).map(
                            Box::new(move |x| (x, corners.0.1))
                            as Box<dyn Fn(i32) -> (i32, i32)>
                            )
                    }
                    else {
                        let (min_c, max_c) = (
                            min(corners.0.1, corners.1.1),
                            max(corners.0.1, corners.1.1)
                            );
                        (min_c..=max_c).map(
                            Box::new(move |y| (corners.0.0, y))
                            as Box<dyn Fn(i32) -> (i32, i32)>
                            )
                    }
                }))
            .map(|wall| (wall, '#'))
            .collect()
    }

    pub fn solve_a(input: &str) -> usize {
        let mut cave = parse(input);

        let abyss = cave.keys().map(|coord| coord.1).max().unwrap();

        loop {
            let mut pos = (500, 0);

            while pos.1 <= abyss {
                if !cave.contains_key(&(pos.0, pos.1 + 1)) {
                    pos.1 += 1;
                }
                else if !cave.contains_key(&(pos.0 - 1, pos.1 + 1)) {
                    pos.0 -= 1;
                    pos.1 += 1;
                }
                else if !cave.contains_key(&(pos.0 + 1, pos.1 + 1)) {
                    pos.0 += 1;
                    pos.1 += 1;
                }
                else {
                    cave.insert(pos, 'o');
                    break;
                }
            }

            if pos.1 > abyss { break; }
        }

        cave.values().filter(|&&c| c == 'o').count()
    }

    pub fn solve_b(input: &str) -> usize {
        let mut cave = parse(input);

        let abyss = cave.keys().map(|coord| coord.1).max().unwrap();

        loop {
            let mut pos = (500, 0);

            loop {
                if !cave.contains_key(&(pos.0, pos.1 + 1)) && pos.1 < abyss + 1 {
                    pos.1 += 1;
                }
                else if !cave.contains_key(&(pos.0 - 1, pos.1 + 1)) && pos.1 < abyss + 1 {
                    pos.0 -= 1;
                    pos.1 += 1;
                }
                else if !cave.contains_key(&(pos.0 + 1, pos.1 + 1)) && pos.1 < abyss + 1 {
                    pos.0 += 1;
                    pos.1 += 1;
                }
                else {
                    cave.insert(pos, 'o');
                    break;
                }
            }

            if pos == (500, 0) { break; }
        }

        cave.values().filter(|&&c| c == 'o').count()
    }

    #[cfg(test)]
    mod test_day14 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

            assert_eq!(solve_a(&input), 24);
        }

        #[test]
        fn test_solve_b() {
            let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

            assert_eq!(solve_b(&input), 93);
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
