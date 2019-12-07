use std::cmp::max;

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
            Some((i, (ak, ak1))) => (i, ak),
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

fn main() {
    use std::io::{self, BufRead};

    let mut orig_inputs = [0, 1, 2, 3, 4];

    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap().split(",")
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut sig_max = 0;

    while next_permutation(&mut orig_inputs[..]) {
        let mut phase = orig_inputs.iter();
        let mut sig_in = 0;

        for amp in 0..5 {
            let mut i = 0;
            let mut numbers = numbers.clone();
            let input_vec = vec![*phase.next().unwrap(), sig_in];
            let mut input = input_vec.iter();


            loop {
                let opcode = numbers[i] % 100;

                if opcode == 99 {break;}


                let out = match opcode {
                    1 | 2 | 7 | 8 => Some(numbers[i+3] as usize),
                    3 => Some(numbers[i+1] as usize),
                    4 => Some(if numbers[i]/100 % 10 == 1 { i + 1 } else { numbers[i+1] as usize }),
                    5 | 6 => None,
                    _ => panic!("Invalid position!"),
                };

                match out {
                    Some(out) => numbers[out] = match opcode {
                        1 | 2 => {
                            let (in1, in2) = get_in(&numbers, i);

                            if opcode == 1 { numbers[in1] + numbers[in2]}
                                else { numbers[in1] * numbers[in2] }},
                        3 => { *input.next().unwrap() },
                        4 => {
                            //println!("Opcode 4: amp:{}, output: {}", amp, numbers[out]);
                            sig_in = numbers[out];
                            numbers[out] },
                        7 | 8 => {
                            let (in1, in2) = get_in(&numbers, i);

                            if (opcode == 7 && numbers[in1] < numbers[in2])
                                || (opcode == 8 && numbers[in1] == numbers[in2])
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
                        let (in1, in2) = get_in(&numbers, i);

                        if (opcode == 5 && numbers[in1] != 0) || (opcode == 6 && numbers[in1] == 0)
                            { numbers[in2] as usize }
                        else
                            { i + 3 }
                    }
                    _ => panic!("Invalid position!"),
                };
            }
        }
        sig_max = max(sig_max, sig_in);
    }
    println!("{}", sig_max);
}
