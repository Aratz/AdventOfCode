extern crate regex;

mod day18 {
    use regex::Regex;
    use std::collections::HashMap;
    use std::cmp::{min, max};

    fn parse(input: &str) -> Vec<((char, i64), (char, i64))> {
        let re_inst = Regex::new(r"(?P<dir>[UDLR]) (?P<dist>\d+) \(#(?P<color>[0-9a-f]{6})\)").unwrap();

        re_inst.captures_iter(input)
            .map(|capt_inst| (
                (
                    capt_inst["dir"].chars().next().unwrap(),
                    capt_inst["dist"].parse().unwrap(),
                ),
                (
                    match &capt_inst["color"][5..] {
                        "0" => { 'R' },
                        "1" => { 'D' },
                        "2" => { 'L' },
                        "3" => { 'U' },
                        _ => unreachable!(),
                    },
                    i64::from_str_radix(&capt_inst["color"][..5], 16).unwrap(),
                ),
            )).collect()
    }

    fn compute_union(intervals: &[(i64, i64)]) -> Vec<(i64, i64)> {
        let mut union = Vec::new();
        union.push(intervals[0]);

        for &next in intervals.iter().skip(1) {
            let last = union.last_mut().unwrap();
            if last.1 >= next.0 {
                *last = (last.0, max(last.1, next.1));
            }
            else {
                union.push(next);
            }
        }

        union
    }

    fn comp_new_intervals(old_intervals: &[(i64, i64)]) -> Vec<(i64, i64)> {
        let mut new_intervals = Vec::new();
        let mut cur_opt = Some(old_intervals[0]);
        for &next in old_intervals.iter().skip(1) {
            if let Some(current) =cur_opt {
                if current.1 < next.0 {
                    new_intervals.push(current);
                    cur_opt = Some(next);
                }
                else if current == next {
                    cur_opt = None;
                }
                else if current.1 == next.0 {
                    cur_opt = Some((current.0, next.1));
                }
                else if current.0 == next.0 {
                    cur_opt = Some((min(current.1, next.1), max(current.1, next.1)));
                }
                else if current.1 == next.1 {
                    cur_opt = Some((min(current.0, next.0), max(current.0, next.0)));
                }
                else if current.0 < next.0 && next.1 < current.1 {
                    new_intervals.push((current.0, next.0));
                    cur_opt = Some((next.1, current.1));
                }
                else {
                    unreachable!();
                }
            }
            else { cur_opt = Some(next); }
        }
        if let Some(current) = cur_opt { new_intervals.push(current); }

        new_intervals
    }

    fn solve(inst: &[(char, i64)]) -> usize {
        let n = inst.len();

        let dir_map = vec![
            ('U', (0, 1)),
            ('D', (0, -1)),
            ('L', (-1, 0)),
            ('R', (1, 0)),
        ].into_iter().collect::<HashMap<_, _>>();

        let mut corners= Vec::new();
        let mut pos = (0i64, 0i64);
        corners.push(pos);

        for (dir, len) in inst {
            pos = (pos.0 + len*dir_map[&dir].0, pos.1 + len*dir_map[&dir].1);
            corners.push(pos);
        }

        corners.sort();
        corners.dedup();

        let intervals: Vec<(i64, Vec<(i64, i64)>)> = corners
            .chunk_by(|a, b| a.0 == b.0)
            .map(|group| (
                    group[0].0,
                    group.chunks(2).map(|slice| (slice[0].1, slice[1].1)).collect()
                )
            )
            .collect();

        let mut prev_x = intervals[0].0;
        let mut prev_intervals = intervals[0].1.clone();

        let mut total_area: i64 = prev_intervals.iter().map(|(b, e)| e - b + 1).sum();

        for (x, ys) in intervals.iter().skip(1) {
            total_area += (x - prev_x - 1)
                *(prev_intervals.iter().map(|(b, e)| e - b + 1).sum::<i64>());

            let mut tmp_union = prev_intervals.clone();
            tmp_union.extend_from_slice(ys);
            tmp_union.sort();

            let union = compute_union(&tmp_union);
            total_area += union.iter().map(|(b, e)| e - b + 1).sum::<i64>();

            let new_intervals = comp_new_intervals(&tmp_union);

            prev_x = *x;
            prev_intervals = new_intervals;
        }

        total_area as usize
    }

    pub fn solve_a(input: &str) -> usize {
        let inst = parse(input).into_iter()
            .map(|((dir, l), _)| (dir, l))
            .collect::<Vec<(char, i64)>>();

        solve(&inst)
    }

    pub fn solve_b(input: &str) -> usize {
        let inst = parse(input).into_iter()
            .map(|(_, (dir, l))| (dir, l))
            .collect::<Vec<(char, i64)>>();

        solve(&inst)
    }

    #[cfg(test)]
    mod test_day18 {
        use super::*;

        static INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        #[test]
        fn test_union() {
            let input = vec![
                (-5, -4),
                (-5, 0),
                (-3, -2),
                (-1, 0),
            ];
            assert_eq!(compute_union(&input), vec![(-5, 0)]);

        }
        #[test]
        fn test_solve() {
            let input = vec![
                ('R', 1),
                ('D', 1),
                ('R', 1),
                ('D', 1),
                ('L', 1),
                ('D', 1),
                ('R', 1),
                ('D', 1),
                ('L', 1),
                ('D', 1),
                ('L', 1),
                ('U', 5),
            ];
            assert_eq!(solve(&input), 16);
        }

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 62);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 952408144115);
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

    println!("Solution A-part: {}", day18::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day18::solve_b(&buffer.trim()));
}
