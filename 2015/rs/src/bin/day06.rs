extern crate regex;

mod day06 {
    use regex::Regex;
    use std::cmp::max;

    const GRID_SIZE: usize = 1000;

    enum Inst {
        TurnOn((usize, usize), (usize, usize)),
        TurnOff((usize, usize), (usize, usize)),
        Toggle((usize, usize), (usize, usize)),
    }

    fn parse(input: &str) -> Vec<Inst> {
        let re_inst = Regex::new(r"(?P<type>toggle|turn off|turn on) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();

        re_inst.captures_iter(input)
            .map(|capt|
                match &capt["type"] {
                    "toggle" => Inst::Toggle(
                        (capt["x1"].parse().unwrap(), capt["x2"].parse().unwrap()),
                        (capt["y1"].parse().unwrap(), capt["y2"].parse().unwrap()),
                        ),
                    "turn off" => Inst::TurnOff(
                        (capt["x1"].parse().unwrap(), capt["x2"].parse().unwrap()),
                        (capt["y1"].parse().unwrap(), capt["y2"].parse().unwrap()),
                        ),
                    "turn on" => Inst::TurnOn(
                        (capt["x1"].parse().unwrap(), capt["x2"].parse().unwrap()),
                        (capt["y1"].parse().unwrap(), capt["y2"].parse().unwrap()),
                        ),
                    _ => unreachable!(),
                })
            .collect()
    }

    pub fn solve_a(input: &str) -> usize {
        let insts = parse(input);

        let mut lights = vec![vec![false; GRID_SIZE]; GRID_SIZE];

        for inst in insts {
            match inst {
                Inst::Toggle((x1, x2), (y1, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            lights[x][y] = !lights[x][y];
                        }
                    }
                }
                Inst::TurnOff((x1, x2), (y1, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            lights[x][y] = false;
                        }
                    }
                }
                Inst::TurnOn((x1, x2), (y1, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            lights[x][y] = true;
                        }
                    }
                }
            }
        }

        lights.iter()
            .map(|row| row.iter().map(|&l| if l { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>()
    }

    pub fn solve_b(input: &str) -> usize {
        let insts = parse(input);

        let mut lights = vec![vec![0; GRID_SIZE]; GRID_SIZE];

        for inst in insts {
            match inst {
                Inst::Toggle((x1, x2), (y1, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            lights[x][y] += 2;
                        }
                    }
                }
                Inst::TurnOff((x1, x2), (y1, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            lights[x][y] = max(1, lights[x][y]) - 1;
                        }
                    }
                }
                Inst::TurnOn((x1, x2), (y1, y2)) => {
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            lights[x][y] += 1;
                        }
                    }
                }
            }
        }

        lights.iter()
            .flat_map(|row| row.iter())
            .sum::<usize>()
    }

    #[cfg(test)]
    mod test_day06 {
        use super::*;

        #[test]
        fn test_solve_b() {
            let input = "toggle 0,0 through 999,999";
            assert_eq!(solve_b(&input), 2000000);

            let input = "turn off 0,0 through 999,999";
            assert_eq!(solve_b(&input), 0);
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

    println!("Solution A-part: {}", day06::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day06::solve_b(&buffer.trim()));
}
