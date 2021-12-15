mod day12 {
    const NREG: usize = 4;

    #[derive(Clone, Copy, Debug)]
    enum Variable {
        Value(i32),
        Pointer(usize),
    }

    impl Variable {
        fn new(s: &str) -> Self {
            match s.parse::<i32>() {
                Ok(v) => Variable::Value(v),
                Err(_) => Variable::Pointer(reg2ind(s)),
            }
        }

        fn eval(&self, regs: &[i32; NREG]) -> i32 {
            match self {
                Variable::Value(v) => *v,
                Variable::Pointer(p) => regs[*p],
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum Instruction {
        Inc(Variable),
        Dec(Variable),
        Tgl(Variable),
        Cpy(Variable, Variable),
        Jnz(Variable, Variable),
    }

    fn reg2ind(reg: &str) -> usize {
        reg.chars().next().unwrap() as usize - 'a' as usize
    }

    impl Instruction {
        fn new(s: &str) -> Self {
            let words = s.split(" ").collect::<Vec<_>>();
            match words[0] {
                "inc" => Instruction::Inc(Variable::new(words[1])),
                "tgl" => Instruction::Tgl(Variable::new(words[1])),
                "dec" => Instruction::Dec(Variable::new(words[1])),
                "cpy" => Instruction::Cpy(Variable::new(words[1]), Variable::new(words[2])),
                "jnz" => Instruction::Jnz(Variable::new(words[1]), Variable::new(words[2])),
                _ => unreachable!(),
            }
        }
    }

    fn execute(instructions: &[Instruction], mut regs: [i32; NREG]) -> [i32; NREG] {
        let mut instructions = instructions.to_vec();
        let mut i = 0;

        while i < instructions.len() {
            match instructions[i] {
                Instruction::Inc(Variable::Pointer(p)) => { regs[p] += 1; },
                Instruction::Dec(Variable::Pointer(p)) => { regs[p] -= 1; },
                Instruction::Tgl(Variable::Pointer(p)) => {
                    let i_inst = i as i32 + regs[p];
                    if !(0 <= i_inst && i_inst < instructions.len() as i32) {
                        i += 1;
                        continue;
                    }
                    let i_inst = i_inst as usize;

                    instructions[i_inst] = match instructions[i_inst] {
                        Instruction::Inc(var) => { Instruction::Dec(var) },
                        Instruction::Dec(var) => { Instruction::Inc(var) },
                        Instruction::Tgl(var) => { Instruction::Inc(var) },
                        Instruction::Cpy(var1, var2) => { Instruction::Jnz(var1, var2) },
                        Instruction::Jnz(var1, var2) => { Instruction::Cpy(var1, var2) },
                    }
                },
                Instruction::Cpy(var, Variable::Pointer(p)) => { regs[p] = var.eval(&regs); },
                Instruction::Jnz(x, y) => {
                    if x.eval(&regs) != 0 {
                        i = ((i as i32) +  y.eval(&regs) - 1) as usize;
                    }
                },
                _ => {},
            }
            i += 1;
        }

        regs
    }

    pub fn solve(lines: &[String], init: [i32; NREG]) -> i32 {
        let regs = execute(
            &lines.iter().map(|s| Instruction::new(s)).collect::<Vec<_>>(),
            init);
        regs[0]
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<String> = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect();

    println!("Solution A-part: {}", day12::solve(&lines, [7, 0, 0, 0]));
    println!("Solution B-part: {}", day12::solve(&lines, [12, 0, 1, 0]));
}
