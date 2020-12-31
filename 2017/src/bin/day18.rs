mod day18 {
    use std::collections::HashMap;

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

    pub enum Inst {
        Snd(Value),
        Set(char, Value),
        Add(char, Value),
        Mul(char, Value),
        Mod(char, Value),
        Rcv(Value),
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
                    let x = x.unwrap(&registers);

                    if x != 0 {
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
                "rcv" => Inst::Rcv(Value::wrap(&raw_inst[1])),
                "jgz" => Inst::Jgz(Value::wrap(&raw_inst[1]), Value::wrap(&raw_inst[2])),
                _ => unreachable!()
            }
        }).collect();

    println!("Solve A-part: {}", day18::solve_a(&insts));
}
