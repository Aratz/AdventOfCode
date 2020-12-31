mod day18 {
    use std::collections::{HashMap, VecDeque};

    #[derive(Clone, Debug)]
    pub enum Value {
        Reg(char),
        Num(i64),
    }

    impl Value {
        pub fn unwrap(&self, registers: &HashMap<char, i64>) -> i64 {
            match self {
                Value::Reg(c) => registers[c],
                Value::Num(v) => *v,
            }
        }

        pub fn wrap(value: &str) -> Self {
            match value.parse::<i64>() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(value.chars().next().unwrap()),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Inst {
        Snd(Value),
        Set(char, Value),
        Add(char, Value),
        Mul(char, Value),
        Mod(char, Value),
        Rcv(char),
        Jgz(Value, Value),

    }

    pub fn solve_a(insts: &[Inst]) -> i64 {
        let mut i: i64 = 0;
        let mut last_played = 0;
        let mut registers: HashMap::<char, i64> = HashMap::new();

        loop {
            match &insts[i as usize] {
                Inst::Snd(x) => {
                    last_played = x.unwrap(&registers);
                },
                Inst::Set(x, y) => {
                    registers.insert(*x, y.unwrap(&registers));
                },
                Inst::Add(x, y) => {
                    let y = y.unwrap(&registers);
                    registers.entry(*x).and_modify(|e| *e += y).or_insert(y);
                },
                Inst::Mul(x, y) => {
                    let y = y.unwrap(&registers);
                    registers.entry(*x).and_modify(|e| *e *= y).or_insert(0);
                },
                Inst::Mod(x, y) => {
                    let y = y.unwrap(&registers);
                    registers.entry(*x).and_modify(|e| *e %= y).or_insert(0);
                },
                Inst::Rcv(x) => {
                    if registers[&x] != 0 {
                        return last_played;
                    }
                },
                Inst::Jgz(x, y) => {
                    let x = x.unwrap(&registers);
                    let y = y.unwrap(&registers);
                    if x > 0 {
                        i += y - 1;
                    }
                },
            }
            i += 1;

            if i < 0 || i >= insts.len() as i64 { break; }
        }

        return -1;
    }

    struct Computer {
        pos: i64,
        insts: Vec<Inst>,
        registers: HashMap<char, i64>,
        queue: VecDeque<i64>,
        n_send: i64,
        done: bool,
        is_waiting: bool,
    }

    impl Computer {
        fn new(id: i64, insts: &[Inst]) -> Self {
            let mut registers = HashMap::new();
            registers.insert('p', id);

            Computer {
                pos: 0,
                insts: insts.to_vec(),
                registers,
                queue: VecDeque::new(),
                n_send: 0,
                done: false,
                is_waiting: false,
            }
        }

        fn step(&mut self) -> Option<i64> {
            if self.done { return None };

            match &self.insts[self.pos as usize] {
                Inst::Snd(x) => {
                    self.pos += 1;
                    if self.pos < 0 || self.pos >= self.insts.len() as i64 { self.done = true; }

                    self.n_send += 1;

                    return Some(x.unwrap(&self.registers));
                },
                Inst::Rcv(x) => {
                    if let Some(v) = self.queue.pop_front() {
                        self.registers.insert(*x, v);
                        self.is_waiting = false;
                    }
                    else {
                        self.is_waiting = true;
                        return None;
                    }
                },
                Inst::Set(x, y) => {
                    self.registers.insert(*x, y.unwrap(&self.registers));
                },
                Inst::Add(x, y) => {
                    let y = y.unwrap(&self.registers);
                    self.registers.entry(*x).and_modify(|e| *e += y).or_insert(y);
                },
                Inst::Mul(x, y) => {
                    let y = y.unwrap(&self.registers);
                    self.registers.entry(*x).and_modify(|e| *e *= y).or_insert(0);
                },
                Inst::Mod(x, y) => {
                    let y = y.unwrap(&self.registers);
                    self.registers.entry(*x).and_modify(|e| *e %= y).or_insert(0);
                },
                Inst::Jgz(x, y) => {
                    let x = x.unwrap(&self.registers);
                    let y = y.unwrap(&self.registers);
                    if x > 0 {
                        self.pos += y - 1;
                    }
                },
            }
            self.pos += 1;

            if self.pos < 0 || self.pos >= self.insts.len() as i64 { self.done = true; }

            return None;
        }
    }

    pub fn solve_b(insts: &[Inst]) -> i64 {
        let mut computers = vec![
            Computer::new(0, insts),
            Computer::new(1, insts),
        ];

        while !computers.iter().all(|c| c.done)
            && !computers.iter().all(|c| c.is_waiting) {
            for i in 0..2 {
                if let Some(snd) = computers[i].step() {
                    computers[1 - i].queue.push_back(snd);
                }
            }
        }

        computers[1].n_send
    }

    #[cfg(test)]
    mod test_day18 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let insts = vec![
                Inst::Set('a', Value::Num(1)),
                Inst::Add('a', Value::Num(2)),
                Inst::Mul('a', Value::Reg('a')),
                Inst::Mod('a', Value::Num(5)),
                Inst::Snd(Value::Reg('a')),
                Inst::Set('a', Value::Num(0)),
                Inst::Rcv(Value::Reg('a')),
                Inst::Jgz(Value::Reg('a'), Value::Num(-1)),
                Inst::Set('a', Value::Num(1)),
                Inst::Jgz(Value::Reg('a'), Value::Num(-2)),
            ];

            assert_eq!(solve_a(&insts), 4);
        }
    }
}


fn main() {
    use std::io::{self, BufRead};
    use day18::{Inst, Value};

    let stdin = io::stdin();

    let insts: Vec<Inst> = stdin.lock().lines().map(
        |l| {
            let raw_inst: Vec<String> = l.unwrap().split(' ').map(|s| s.into()).collect();
            match raw_inst[0].as_str() {
                "snd" => Inst::Snd(Value::wrap(&raw_inst[1])),
                "set" => Inst::Set(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                "add" => Inst::Add(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                "mul" => Inst::Mul(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                "mod" => Inst::Mod(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                "rcv" => Inst::Rcv(raw_inst[1].chars().next().unwrap()),
                "jgz" => Inst::Jgz(Value::wrap(&raw_inst[1]), Value::wrap(&raw_inst[2])),
                _ => unreachable!()
            }
        }).collect();

    println!("Solve A-part: {}", day18::solve_a(&insts));
    println!("Solve A-part: {}", day18::solve_b(&insts));
}
