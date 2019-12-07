use std::cmp::max;
use std::collections::VecDeque;

fn get_in(data:&Vec<i32>, pointer:usize) -> (usize, usize) {
    let in1 = if data[pointer]/100 % 10 == 1 { pointer + 1 }
        else { data[pointer+1] as usize };
    let in2 = if data[pointer]/1000 % 10 == 1 { pointer + 2 }
        else { data[pointer+2] as usize };

    (in1, in2)
}

fn next_permutation<T: PartialOrd>(a: &mut [T]) -> bool {
    let (k, l) = {
        let (k, ak) = match a.iter().zip(a.iter().skip(1)).enumerate()
            .filter(|(_i, (a1, a2))| a1 < a2).last() {
            Some((i, (ak, _ak1))) => (i, ak),
            None => return false,
        };

        let (l, _al) = a.iter().enumerate().skip(k+1)
            .filter(|(_i, al)| ak < al).last().unwrap();

        (k, l)
    };

    a.swap(k, l);
    a[k+1..].reverse();

    true
}

struct AmpComputer {
    i: usize,
    instructions: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
}

impl AmpComputer {
    fn current_opcode(&self) -> i32 {
        self.instructions[self.i]
    }

    fn add_input(&mut self, v: i32) {
        self.input.push_back(v);
    }

    fn get_next_output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    /// Run computer until input is empty (returns true) or opcode 99 is
    /// found (return false)
    fn compute(&mut self) -> bool {
        loop {
            let opcode = self.instructions[self.i] % 100;

            if opcode == 99 { return false}


            let out = match opcode {
                1 | 2 | 7 | 8 => Some(self.instructions[self.i+3] as usize),
                3 => Some(self.instructions[self.i+1] as usize),
                4 => Some(if self.instructions[self.i]/100 % 10 == 1 { self.i + 1 }
                          else { self.instructions[self.i+1] as usize }),
                5 | 6 => None,
                _ => panic!("Invalid position! (pos: {}, opcode: {})", self.i, opcode),
            };

            match out {
                Some(out) => self.instructions[out] = match opcode {
                    1 | 2 => {
                        let (in1, in2) = get_in(&self.instructions, self.i);

                        if opcode == 1 { self.instructions[in1] + self.instructions[in2]}
                            else { self.instructions[in1] * self.instructions[in2] }},
                    3 => {
                        match self.input.pop_front() {
                            Some(v) => v,
                            None => return true,
                        }
                    },
                    4 => {
                        self.output.push_back(self.instructions[out]);
                        self.instructions[out] },
                    7 | 8 => {
                        let (in1, in2) = get_in(&self.instructions, self.i);

                        if (opcode == 7 && self.instructions[in1] < self.instructions[in2])
                            || (opcode == 8 && self.instructions[in1] == self.instructions[in2])
                                { 1 }
                            else
                                { 0 }
                    }
                    _ => panic!("Invalid position! (pos: {}, opcode: {})", self.i, opcode),
                },
                None => {},
            };

            self.i = match opcode {
                1 | 2 | 7 | 8 => self.i + 4,
                3 | 4 => self.i + 2,
                5 | 6 => {
                    let (in1, in2) = get_in(&self.instructions, self.i);

                    if (opcode == 5 && self.instructions[in1] != 0)
                        || (opcode == 6 && self.instructions[in1] == 0)
                        { self.instructions[in2] as usize }
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

    let mut orig_inputs = [5, 6, 7, 8, 9];

    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap().split(",")
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut sig_max = 0;

    while next_permutation(&mut orig_inputs[..]) {
        let mut phase = orig_inputs.iter();
        let mut sig_in = 0;

        let mut amp_computers = (0..5)
            .map(|_i| AmpComputer {
                i: 0,
                instructions: numbers.clone(),
                input: VecDeque::new(),
                output: VecDeque::new() })
            .collect::<Vec<_>>();

        for amp in 0..5 {
            amp_computers[amp].add_input(*phase.next().unwrap());
        }

        amp_computers[0].add_input(sig_in);

        let mut amp = 0;
        while amp_computers[amp].current_opcode() != 99 {
            amp_computers[amp].compute();

            while let Some(output) = amp_computers[amp].get_next_output() {
                sig_in = output;
                amp_computers[(amp + 1) % 5].add_input(sig_in);
            }
            amp = (amp + 1) % 5;
        }
        sig_max = max(sig_max, sig_in);
    }
    println!("{}", sig_max);
}
