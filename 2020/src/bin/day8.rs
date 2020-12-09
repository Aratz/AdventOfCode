use std::io::{self, BufRead};

enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn main() {
    let stdin = io::stdin();

    let instructions = stdin.lock().lines()
        .map(|instruction| {
            let raw_instruction = instruction.unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            match raw_instruction[0].as_str() {
                "acc" => Operation::Acc(raw_instruction[1].parse().unwrap()),
                "jmp" => Operation::Jmp(raw_instruction[1].parse().unwrap()),
                "nop" => Operation::Nop(raw_instruction[1].parse().unwrap()),
                _ => { panic!("Unknown instruction!"); },
            }
        })
        .collect::<Vec<_>>();

    let mut ptr: i32 = 0;
    let mut acc: i32 = 0;
    let mut visited: Vec<bool> = vec![false; instructions.len()];

    while !visited[ptr as usize] {
        visited[ptr as usize] = true;
        match instructions[ptr as usize] {
            Operation::Acc(x) => {
                acc += x;
                ptr += 1;
            }
            Operation::Jmp(x) => {
                ptr += x;
            }
            Operation::Nop(_) => {
                ptr += 1;
            }
        }
    }

    println!("{}", acc);

}
