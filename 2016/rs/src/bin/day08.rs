extern crate regex;

mod day08 {
    use regex::Regex;

    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;


    enum Operation {
        Rect { width: usize, height: usize },
        RotateRow {row: usize, offset: usize },
        RotateColumn { column: usize, offset: usize },
    }

    impl Operation {
        fn new(line: &str) -> Self {
            let reg_rect = Regex::new(r"rect (?P<width>\d+)x(?P<height>\d+)").unwrap();
            let reg_row = Regex::new(r"rotate row y=(?P<row>\d+) by (?P<offset>\d+)").unwrap();
            let reg_column = Regex::new(r"rotate column x=(?P<column>\d+) by (?P<offset>\d+)").unwrap();

            match line {
                s if reg_rect.is_match(s) => {
                    let caps = reg_rect.captures(s).unwrap();
                    let width = caps.name("width").unwrap().as_str().parse().unwrap();
                    let height = caps.name("height").unwrap().as_str().parse().unwrap();

                    Operation::Rect { width, height }
                },
                s if reg_row.is_match(s) => {
                    let caps = reg_row.captures(s).unwrap();
                    let row = caps.name("row").unwrap().as_str().parse().unwrap();
                    let offset = caps.name("offset").unwrap().as_str().parse().unwrap();

                    Operation::RotateRow { row, offset }
                },
                s if reg_column.is_match(s) => {
                    let caps = reg_column.captures(s).unwrap();
                    let column = caps.name("column").unwrap().as_str().parse().unwrap();
                    let offset = caps.name("offset").unwrap().as_str().parse().unwrap();

                    Operation::RotateColumn { column, offset }
                },
                _ => panic!("Unrecognized operation"),
            }
        }
    }

    fn execute(ops: &Vec<Operation>) -> [[bool; WIDTH]; HEIGHT] {
        let mut screen: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

        for op in ops {
            match op {
                Operation::Rect { width, height } => {
                    for y in 0..*height {
                        for x in 0..*width {
                            screen[y][x] = true;
                        }
                    }
                },
                Operation::RotateRow { row, offset } => {
                    let row_cpy = screen[*row].to_vec();
                    for x in 0..WIDTH {
                        screen[*row][x] = row_cpy[(x + WIDTH - *offset)%WIDTH];
                    }
                }
                Operation::RotateColumn { column, offset }=> {
                    let column_cpy: Vec<_> = screen.iter().map(|row| row[*column]).collect();
                    for y in 0..HEIGHT {
                        screen[y][*column] = column_cpy[(y + HEIGHT - *offset)%HEIGHT];
                    }
                }
            }
        }

        screen
    }

    pub fn solve_a(raw_ops: &Vec<String>) -> usize {
        let ops: Vec<Operation> = raw_ops.iter().map(|s| Operation::new(s)).collect();

        let screen = execute(&ops);

        screen.iter().map(|row| row.iter().filter(|&&px| px).count()).sum()
    }

    pub fn solve_b(raw_ops: &Vec<String>) {
        let ops: Vec<Operation> = raw_ops.iter().map(|s| Operation::new(s)).collect();

        let screen = execute(&ops);

        let display: Vec<String> = screen.iter()
            .map(|row| row.iter().map(|&px| if px { '█' } else { ' ' }).collect()).collect();

        for row in display {
            println!("{}", row);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<_> = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect();

    println!("Solution A-part: {}", day08::solve_a(&lines));
    println!("Solution B-part:");
    day08::solve_b(&lines);
}
