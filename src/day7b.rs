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

fn amp_computer(start: usize,
                instructions: &mut Vec<i32>,
                input: &mut VecDeque<i32>,
                output: &mut VecDeque<i32>) -> usize {
    let mut i = start;

    loop {
        let opcode = instructions[i] % 100;

        if opcode == 99 {break;}


        let out = match opcode {
            1 | 2 | 7 | 8 => Some(instructions[i+3] as usize),
            3 => Some(instructions[i+1] as usize),
            4 => Some(if instructions[i]/100 % 10 == 1 { i + 1 } else { instructions[i+1] as usize }),
            5 | 6 => None,
            _ => panic!("Invalid position! (pos: {}, opcode: {})", i, opcode),
        };

        match out {
            Some(out) => instructions[out] = match opcode {
                1 | 2 => {
                    let (in1, in2) = get_in(&instructions, i);

                    if opcode == 1 { instructions[in1] + instructions[in2]}
                        else { instructions[in1] * instructions[in2] }},
                3 => {
                    match input.pop_front() {
                        Some(v) => v,
                        None => return i,
                    }
                },
                4 => {
                    //println!("Opcode 4: amp:{}, output: {}", amp, instructions[out]);
                    output.push_back(instructions[out]);
                    instructions[out] },
                7 | 8 => {
                    let (in1, in2) = get_in(&instructions, i);

                    if (opcode == 7 && instructions[in1] < instructions[in2])
                        || (opcode == 8 && instructions[in1] == instructions[in2])
                            { 1 }
                        else
                            { 0 }
                }
                _ => panic!("Invalid position!"),
            },
            None => {},
        };

        i = match opcode {
            1 | 2 | 7 | 8 => i + 4,
            3 | 4 => i + 2,
            5 | 6 => {
                let (in1, in2) = get_in(&instructions, i);

                if (opcode == 5 && instructions[in1] != 0) || (opcode == 6 && instructions[in1] == 0)
                    { instructions[in2] as usize }
                else
                    { i + 3 }
            }
            _ => panic!("Invalid position!"),
        };
    }

    i
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

        let mut starts = (0..5)
            .map(|_i| 0).collect::<Vec<usize>>();
        let mut instructions = (0..5)
            .map(|_i| numbers.clone()).collect::<Vec<_>>();
        let mut inputs = (0..5)
            .map(|_i| VecDeque::new()).collect::<Vec<VecDeque<i32>>>();
        let mut outputs = (0..5)
            .map(|_i| VecDeque::new()).collect::<Vec<VecDeque<i32>>>();

        for amp in 0..5 {
            inputs[amp].push_back(*phase.next().unwrap());
        }

        inputs[0].push_back(sig_in);

        let mut amp = 0;
        while instructions[amp][starts[amp]] != 99 {
            let i = amp_computer(
                starts[amp],
                &mut instructions[amp],
                &mut inputs[amp],
                &mut outputs[amp]);

            while !outputs[amp].is_empty() {
                sig_in = outputs[amp].pop_front().unwrap();
                inputs[(amp + 1) % 5].push_back(sig_in);
            }
            starts[amp] = i;
            amp = (amp + 1) % 5;
        }
        sig_max = max(sig_max, sig_in);
    }
    println!("{}", sig_max);
}
