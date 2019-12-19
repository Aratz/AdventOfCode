use std::collections::{VecDeque, HashMap};

struct AmpComputer {
    i: usize,
    instructions: HashMap<usize, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative_base: usize,
    done: bool,
}

#[allow(dead_code)]
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

            if let Some(out) = out { self.instructions.entry(out).or_insert(0); }

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
                            else { 0 }
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

            if let Some(new_out) = new_out {
                self.instructions.entry(out.unwrap()).and_modify(|e| *e = new_out);
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
                    else { self.i + 3 }
                }
                _ => panic!("Invalid position!"),
            };
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap().split(',')
        .map(|x| x.parse::<i64>().unwrap()).enumerate().collect::<HashMap<usize, i64>>();


    let mut computer = AmpComputer {
                i: 0,
                instructions: numbers,
                input: VecDeque::new(),
                output: VecDeque::new(),
                relative_base: 0,
                done: false,
    };

    let mut hull: HashMap<(i64, i64), i64> = HashMap::new();


    let mut pos = (0, 0);
    let mut dir = 0;

    hull.insert(pos, 0);
    computer.add_input(0);

    while !computer.done {
        computer.compute();
        let color = computer.get_next_output().unwrap();
        let turn = computer.get_next_output().unwrap();

        hull.insert(pos, color);

        dir = (4 + dir + 2*turn - 1) % 4;

        pos = match dir {
            0 => (pos.0, pos.1 + 1),
            1 => (pos.0 + 1, pos.1),
            2 => (pos.0, pos.1 - 1),
            3 => (pos.0 - 1, pos.1),
            _ => panic!("Direction error: {}", dir),
        };

        computer.add_input(match hull.get(&pos) {
            Some(color) => *color,
            None => 0,
        });

    }

    println!("Panels covered: {}", hull.len());
}
