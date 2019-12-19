extern crate rand;

use std::collections::{VecDeque, HashMap};

fn print_map(map: &HashMap<(i64, i64), i64>, pos: (i64, i64)) {
    let min_x = map.keys().map(|(x, _y)| x).min().unwrap();
    let max_x = map.keys().map(|(x, _y)| x).max().unwrap();
    let min_y = map.keys().map(|(_x, y)| y).min().unwrap();
    let max_y = map.keys().map(|(_x, y)| y).max().unwrap();

    let mut explicit_map = vec![
        vec![String::from(" "); (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for (key, value) in map.iter() {
        explicit_map[(key.1 - min_y) as usize][(key.0 - min_x) as usize] = match value {
            -1 => String::from("█"),
            v => (v%10).to_string(),
        };
    }

    explicit_map[(0 - min_y) as usize][(0 - min_x) as usize] = String::from("S");
    explicit_map[(pos.1 - min_y) as usize][(pos.0 - min_x) as usize] = String::from("D");

    println!("{}",
             explicit_map.iter() .map(|row| row.join(""))
             .collect::<Vec<String>>().join("\n"));
}

struct AmpComputer {
    i: usize,
    instructions: HashMap<usize, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative_base: usize,
    done: bool,
}

impl AmpComputer {
    fn current_opcode(&self) -> i64 {
        self.instructions[&self.i]
    }

    fn add_input(&mut self, v: i64) {
        self.input.push_back(v);
    }

    fn get_next_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    /// Get jth parameter depending on mode (parameters index start from 0)
    fn get_parameter_j(&mut self, j: usize) -> usize{
        self.instructions.entry(self.i).or_insert(0);
        self.instructions.entry(self.i + j + 1).or_insert(0);

        let p = match self.instructions[&self.i]/(10 as i64).pow((j + 2) as u32) % 10 {
            0 => self.instructions[&(self.i + j + 1)] as usize,
            1 => self.i + 1 + j,
            2 => ((self.relative_base as i64) + self.instructions[&(self.i + j + 1)]) as usize,
            _ => panic!("Invalid opcode! (pos: {}, opcode: {})", self.i, self.instructions[&self.i]),
        };

        self.instructions.entry(p).or_insert(0);

        p
    }

    /// Run computer until input is empty (returns true) or opcode 99 is
    /// found (return false)
    fn compute(&mut self) {
        if self.done { return }

        loop {

            self.instructions.entry(self.i).or_insert(0);
            self.instructions.entry(self.i + 1).or_insert(0);
            self.instructions.entry(self.i + 2).or_insert(0);
            self.instructions.entry(self.i + 3).or_insert(0);

            let opcode = self.instructions[&self.i] % 100;

            //println!("(pos: {}, opcode: {}, rel_b: {})",
            //    self.i, self.instructions[&self.i], self.relative_base);

            if opcode == 99 {
                self.done = true;
                return
            }

            // Get output address
            let out = match opcode {
                1 | 2 | 7 | 8 => Some(self.get_parameter_j(2)),
                3 => Some(self.get_parameter_j(0)),
                4 => Some(self.get_parameter_j(0)),
                5 | 6 | 9 => None,
                _ => panic!("Invalid position! (pos: {}, opcode: {})", self.i, opcode),
            };

            if out.is_some() { self.instructions.entry(out.unwrap()).or_insert(0); }

            //Get input parameters
            let in1 = match opcode {
                1 | 2 | 7 | 8 | 9 => Some(self.get_parameter_j(0)),
                _ => None,
            };
            let in2 = match opcode {
                1 | 2 | 7 | 8 => Some(self.get_parameter_j(1)),
                _ => None,
            };

            // Compute new value
            let new_out = match out {
                Some(out) => Some({ match opcode {
                    1 | 2 => {
                        let in1 = in1.unwrap();
                        let in2 = in2.unwrap();

                        if opcode == 1 { self.instructions[&in1] + self.instructions[&in2]}
                            else { self.instructions[&in1] * self.instructions[&in2] }},
                    3 => {
                        match self.input.pop_front() {
                            Some(v) => v,
                            None => return,
                        }
                    },
                    4 => {
                        self.output.push_back(self.instructions[&out]);
                        self.instructions[&out] },
                    7 | 8 => {
                        let in1 = in1.unwrap();
                        let in2 = in2.unwrap();

                        if (opcode == 7 && self.instructions[&in1] < self.instructions[&in2])
                            || (opcode == 8 && self.instructions[&in1] == self.instructions[&in2])
                                { 1 }
                            else
                                { 0 }
                    },
                    _ => panic!("Invalid position! (pos: {}, opcode: {})", self.i, opcode),
                }}),
                None => {
                    match opcode {
                        9 => { 
                            let in1 = in1.unwrap();
                            self.relative_base = ((self.relative_base as i64) + self.instructions[&in1]) as usize;
                            None
                        },
                        5 | 6 => { None },
                        _ => panic!("Invalid position! (pos: {}, opcode: {})", self.i, opcode),
                    }
                },
            };

            match new_out {
                Some(new_out) => {
                    self.instructions.entry(out.unwrap()).and_modify(|e| *e = new_out);
                },
                None => {},
            }

            self.i = match opcode {
                1 | 2 | 7 | 8 => self.i + 4,
                3 | 4 | 9 => self.i + 2,
                5 | 6 => {
                    let in1 = self.get_parameter_j(0);
                    let in2 = self.get_parameter_j(1);

                    if (opcode == 5 && self.instructions[&in1] != 0)
                        || (opcode == 6 && self.instructions[&in1] == 0)
                        { self.instructions[&in2] as usize }
                    else
                        { self.i + 3 }
                }
                _ => panic!("Invalid position!"),
            };
        }
    }
}

fn main() {
    use std::io::{self, BufRead};
    let mut rng = rand::thread_rng();

    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap().split(",")
        .map(|x| x.parse::<i64>().unwrap()).enumerate().collect::<HashMap<usize, i64>>();

    let mut res = 0;

    for x in 0..50 {
        for y in 0..50 {
            let mut computer = AmpComputer {
                        i: 0,
                        instructions: numbers.clone(),
                        input: VecDeque::new(),
                        output: VecDeque::new(),
                        relative_base: 0,
                        done: false,
            };
            computer.add_input(x);
            computer.add_input(y);
            computer.compute();
            res += computer.get_next_output().unwrap();
        }
    }

    println!("{}", res);
}
