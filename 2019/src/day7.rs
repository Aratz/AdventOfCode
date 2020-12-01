use std::cmp::max;

fn get_in(data:&[i32], pointer:usize) -> (usize, usize) {
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

fn amp_computer(mut instructions: Vec<i32>, input_vec: Vec<i32>) -> i32{
    let mut i = 0;
    let mut input = input_vec.iter();
    let mut sig_in = 0;


    loop {
        let opcode = instructions[i] % 100;

        if opcode == 99 {break;}


        let out = match opcode {
            1 | 2 | 7 | 8 => Some(instructions[i+3] as usize),
            3 => Some(instructions[i+1] as usize),
            4 => Some(if instructions[i]/100 % 10 == 1 { i + 1 } else { instructions[i+1] as usize }),
            5 | 6 => None,
            _ => panic!("Invalid position!"),
        };

        if let Some(out) = out {
            instructions[out] = match opcode {
                1 | 2 => {
                    let (in1, in2) = get_in(&instructions, i);

                    if opcode == 1 { instructions[in1] + instructions[in2]}
                        else { instructions[in1] * instructions[in2] }},
                3 => { *input.next().unwrap() },
                4 => {
                    //println!("Opcode 4: amp:{}, output: {}", amp, instructions[out]);
                    sig_in = instructions[out];
                    instructions[out] },
                7 | 8 => {
                    let (in1, in2) = get_in(&instructions, i);

                    if (opcode == 7 && instructions[in1] < instructions[in2])
                        || (opcode == 8 && instructions[in1] == instructions[in2])
                            { 1 }
                        else { 0 }
                }
                _ => panic!("Invalid position!"),
            };
        };

        i = match opcode {
            1 | 2 | 7 | 8 => i + 4,
            3 | 4 => i + 2,
            5 | 6 => {
                let (in1, in2) = get_in(&instructions, i);

                if (opcode == 5 && instructions[in1] != 0) || (opcode == 6 && instructions[in1] == 0)
                    { instructions[in2] as usize }
                else { i + 3 }
            }
            _ => panic!("Invalid position!"),
        };
    }

    sig_in
}

fn main() {
    use std::io::{self, BufRead};

    let mut orig_inputs = [0, 1, 2, 3, 4];

    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap().split(',')
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut sig_max = 0;

    while next_permutation(&mut orig_inputs[..]) {
        let mut phase = orig_inputs.iter();
        let mut sig_in = 0;

        for _amp in 0..5 {
            sig_in = amp_computer(numbers.clone(), vec![*phase.next().unwrap(), sig_in]);
        }
        sig_max = max(sig_max, sig_in);
    }
    println!("{}", sig_max);
}
