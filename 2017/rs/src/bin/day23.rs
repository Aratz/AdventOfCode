mod day23 {
    use std::collections::{HashMap, VecDeque};

    #[derive(Clone, Debug)]
    enum Value {
        Reg(char),
        Num(i64),
    }

    impl Value {
        fn unwrap(&self, registers: &HashMap<char, i64>) -> i64 {
            match self {
                Value::Reg(c) => registers[c],
                Value::Num(v) => *v,
            }
        }

        fn wrap(value: &str) -> Self {
            match value.parse::<i64>() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(value.chars().next().unwrap()),
            }
        }
    }

    #[derive(Clone, Debug)]
    enum Inst {
        Snd(Value),
        Set(char, Value),
        Add(char, Value),
        Sub(char, Value),
        Mul(char, Value),
        Mod(char, Value),
        Rcv(char),
        Jnz(Value, Value),

    }

    struct Computer {
        pos: i64,
        insts: Vec<Inst>,
        registers: HashMap<char, i64>,
        queue: VecDeque<i64>,
        n_send: i64,
        done: bool,
        is_waiting: bool,
        n_mul: usize,
    }

    impl Computer {
        fn new(insts: &[Inst]) -> Self {
            let mut registers = HashMap::new();
            for reg in 'a'..='h' {
                registers.insert(reg, 0);
            }

            Computer {
                pos: 0,
                insts: insts.to_vec(),
                registers,
                queue: VecDeque::new(),
                n_send: 0,
                done: false,
                is_waiting: false,
                n_mul: 0,
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
                Inst::Sub(x, y) => {
                    let y = y.unwrap(&self.registers);
                    self.registers.entry(*x).and_modify(|e| *e -= y).or_insert(y);
                },
                Inst::Mul(x, y) => {
                    let y = y.unwrap(&self.registers);
                    self.registers.entry(*x).and_modify(|e| *e *= y).or_insert(0);
                    self.n_mul += 1;
                },
                Inst::Mod(x, y) => {
                    let y = y.unwrap(&self.registers);
                    self.registers.entry(*x).and_modify(|e| *e %= y).or_insert(0);
                },
                Inst::Jnz(x, y) => {
                    let x = x.unwrap(&self.registers);
                    let y = y.unwrap(&self.registers);
                    if x != 0 {
                        self.pos += y - 1;
                    }
                },
            }
            self.pos += 1;

            if self.pos < 0 || self.pos >= self.insts.len() as i64 { self.done = true; }

            return None;
        }
    }

    fn parse(input: &[String]) -> Vec<Inst> {
        input.iter().map(
            |l| {
                let raw_inst: Vec<String> = l.split(' ').map(|s| s.into()).collect();
                match raw_inst[0].as_str() {
                    "snd" => Inst::Snd(Value::wrap(&raw_inst[1])),
                    "set" => Inst::Set(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                    "add" => Inst::Add(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                    "sub" => Inst::Sub(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                    "mul" => Inst::Mul(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                    "mod" => Inst::Mod(raw_inst[1].chars().next().unwrap(), Value::wrap(&raw_inst[2])),
                    "rcv" => Inst::Rcv(raw_inst[1].chars().next().unwrap()),
                    "jnz" => Inst::Jnz(Value::wrap(&raw_inst[1]), Value::wrap(&raw_inst[2])),
                    _ => unreachable!()
                }
            }).collect()
    }

    pub fn solve_a(input: &[String]) -> usize {
        let insts = parse(input);
        let mut computer = Computer::new(&insts);
        while !computer.done {
            computer.step();
        }

        computer.n_mul
    }

    fn get_primes(i_max: i64) -> Vec<i64> {
        let mut nums: Vec<_> = (2..=i_max).collect();
        let mut primes = Vec::new();

        while !nums.is_empty() {
            let new_prime = nums[0];
            primes.push(new_prime);
            nums = nums.into_iter().filter(|n| n % new_prime != 0).collect();
        }

        primes
    }

    pub fn solve_b(range: (i64, i64), step: i64) -> usize {
        let primes = get_primes(range.1);

    1001 - primes.into_iter().filter(|&n| n >= range.0 && (n - range.0) % step == 0).count()
    }

    #[cfg(test)]
    mod test_day23 {
        use super::*;

        #[test]
        fn test_primes() {
            assert_eq!(get_primes(23), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
        }

    }
}


fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();


    println!("Solve A-part: {}", day23::solve_a(&input));
    println!("Solve B-part: {}", day23::solve_b((106700, 123700), 17));
}
