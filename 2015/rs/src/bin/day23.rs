mod day23 {
    enum Instruction {
        Hlf(usize),
        Tpl(usize),
        Inc(usize),
        Jmp(i32),
        Jie(usize, i32),
        Jio(usize, i32),
    }

    fn parse(input: &str) -> Vec<Instruction> {
        input.lines()
            .map(|l| {
                let words = l.split(" ").collect::<Vec<_>>();
                let r = words[1].chars().next().unwrap() as usize - 'a' as usize;
                match words[0] {
                    "hlf" => { Instruction::Hlf(r) },
                    "tpl" => { Instruction::Tpl(r) },
                    "inc" => { Instruction::Inc(r) },
                    "jmp" => {
                        let offset = words[1].parse().unwrap();
                        Instruction::Jmp(offset)
                    },
                    "jie" => {
                        let offset = words[2].parse().unwrap();
                        Instruction::Jie(r, offset)
                    },
                    "jio" => {
                        let offset = words[2].parse().unwrap();
                        Instruction::Jio(r, offset)
                    },
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    pub fn solve_ab(input: &str, reg_a: i32) -> i32 {
        let insts = parse(input);
        let mut regs = vec![reg_a, 0];
        let mut i: i32 = 0;
        while 0 <= i && i < insts.len() as i32 {
            match insts[i as usize] {
                Instruction::Hlf(r) => { regs[r] /= 2; },
                Instruction::Tpl(r) => { regs[r] *= 3; },
                Instruction::Inc(r) => { regs[r] += 1; },
                Instruction::Jmp(di) => { i += di - 1; },
                Instruction::Jie(r, di) => { if regs[r] % 2 == 0 { i += di - 1; }},
                Instruction::Jio(r, di) => { if regs[r] == 1 { i += di - 1; }},
            }

            i += 1;
        }

        regs[1]
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

    println!("Solution A-part: {}", day23::solve_ab(&buffer.trim(), 0));
    println!("Solution B-part: {}", day23::solve_ab(&buffer.trim(), 1));
}
