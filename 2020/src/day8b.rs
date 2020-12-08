use std::io::{self, BufRead};
use std::collections::VecDeque;

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

    let n_inst = instructions.len();

    // Find terminating instructions
    let mut is_pointed_by: Vec<Vec<usize>> = vec![vec![]; n_inst];
    let mut terminating_instructions: Vec<usize> = Vec::new();

    for (i, instruction) in instructions.iter().enumerate() {
        match instruction {
            Operation::Acc(_) | Operation::Nop(_) => {
                if i + 1 < n_inst {
                    is_pointed_by[i + 1].push(i);
                }
                else {
                    terminating_instructions.push(i);
                }
            },
            Operation::Jmp(x) => {
                if (i as i32) + x < n_inst as i32 {
                    is_pointed_by[(x + (i as i32)) as usize].push(i);
                }
                else {
                    terminating_instructions.push(i);
                }
            },
        }
    }

    //Find instruction leading to terminating instructions
    let mut non_looping_instructions = vec![false; n_inst];
    let mut stack = VecDeque::from(terminating_instructions);

    while let Some(ptr) = stack.pop_front() {
        non_looping_instructions[ptr] = true;
        for ptr_by in is_pointed_by[ptr].iter() {
            stack.push_back(*ptr_by);
        }
    }

    let mut ptr: i32 = 0;
    let mut acc: i32 = 0;
    let mut visited: Vec<bool> = vec![false; n_inst];

    let mut switched = false;

    //When executing, find Nop/Jmp switches that would lead to a non looping instruction
    while (ptr as usize) < n_inst {
        if visited[ptr as usize] {
            panic!("Entering infinite loop again at {}", ptr);
        }
        visited[ptr as usize] = true;
        match instructions[ptr as usize] {
            Operation::Acc(x) => {
                acc += x;
                ptr += 1;
            },
            Operation::Jmp(x) => {
                if !switched  && ((ptr + 1) as usize >= n_inst
                    || non_looping_instructions[(ptr + 1) as usize]) {
                    ptr += 1;
                    switched = true;
                }
                else {
                    ptr += x;
                }
            },
            Operation::Nop(x) => {
                if !switched && ((ptr + x) as usize >= n_inst
                    || non_looping_instructions[(ptr + x) as usize]) {
                    ptr += x;
                    switched = true;
                }
                else {
                    ptr += 1;
                }
            },
        }
    }

    println!("{}", acc);

}
