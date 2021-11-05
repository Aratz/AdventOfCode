mod day12 {
    const NREG: usize = 4;

    enum Instruction {
        CpyVal(i32, usize),
        CpyReg(usize, usize),
        Inc(usize),
        Dec(usize),
        JnzVal(i32, i32),
        JnzReg(usize, i32),
    }

    fn reg2ind(reg: &str) -> usize {
        reg.chars().next().unwrap() as usize - 'a' as usize
    }

    impl Instruction {
        fn new(s: &str) -> Self {
            let words = s.split(" ").collect::<Vec<_>>();
            match words[0] {
                "cpy" => {
                    match words[1].parse::<i32>() {
                        Ok(v) => Instruction::CpyVal(v, reg2ind(words[2])),
                        Err(_) => Instruction::CpyReg(reg2ind(words[1]), reg2ind(words[2])),
                    }
                },
                "inc" => { Instruction::Inc(reg2ind(words[1])) },
                "dec" => { Instruction::Dec(reg2ind(words[1])) },
                "jnz" => {
                    match words[1].parse::<i32>() {
                        Ok(v) => Instruction::JnzVal(v, words[2].parse().unwrap()),
                        Err(_) => Instruction::JnzReg(reg2ind(words[1]), words[2].parse().unwrap()),
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    fn execute(instructions: &[Instruction]) -> [i32; NREG] {
        let mut regs = [0; NREG];
        let mut i = 0;
        while i < instructions.len() {
            match instructions[i] {
                Instruction::CpyVal(x, y) => { regs[y] = x; },
                Instruction::CpyReg(x, y) => { regs[y] = regs[x]; },
                Instruction::Inc(x) => { regs[x] += 1; },
                Instruction::Dec(x) => { regs[x] -= 1; },
                Instruction::JnzVal(x, y) => { if x != 0 { i = ((i as i32) +  y - 1) as usize; }},
                Instruction::JnzReg(x, y) => { if regs[x] != 0 { i = ((i as i32) + y - 1) as usize; }},
            }
            i += 1;
        }

        regs
    }

    pub fn solve_a(lines: &[String]) -> i32 {
        let regs = execute(&lines.iter().map(|s| Instruction::new(s)).collect::<Vec<_>>());
        regs[0]
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<String> = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect();

    println!("Solution A-part: {}", day12::solve_a(&lines));
}
