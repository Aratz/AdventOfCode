fn get_in(data:&[i32], pointer:usize) -> (usize, usize) {
    let in1 = if data[pointer]/100 % 10 == 1 { pointer + 1 }
        else { data[pointer+1] as usize };
    let in2 = if data[pointer]/1000 % 10 == 1 { pointer + 2 }
        else { data[pointer+2] as usize };

    (in1, in2)
}

fn main() {
    use std::io::{self, BufRead};

    let input = 5;

    let stdin = io::stdin();
    let mut numbers = stdin.lock().lines().next().unwrap().unwrap().split(',')
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut i = 0;

    loop {
        let opcode = numbers[i] % 100;

        if opcode == 99 { break; }

        let out = match opcode {
            1 | 2 | 7 | 8 => Some(numbers[i+3] as usize),
            3 => Some(numbers[i+1] as usize),
            4 => Some(if numbers[i]/100 % 10 == 1 { i + 1 } else { numbers[i+1] as usize }),
            5 | 6 => None,
            _ => panic!("Invalid position!"),
        };

        if let Some(out) = out {
            numbers[out] = match opcode {
                1 | 2 => {
                    let (in1, in2) = get_in(&numbers, i);

                    if opcode == 1 { numbers[in1] + numbers[in2]}
                        else { numbers[in1] * numbers[in2] }},
                3 => { input },
                4 => { println!("Opcode 4({}): {}", i, numbers[out]); numbers[out] },
                7 | 8 => {
                    let (in1, in2) = get_in(&numbers, i);

                    if (opcode == 7 && numbers[in1] < numbers[in2])
                        || (opcode == 8 && numbers[in1] == numbers[in2])
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
                let (in1, in2) = get_in(&numbers, i);

                if (opcode == 5 && numbers[in1] != 0) || (opcode == 6 && numbers[in1] == 0)
                    { numbers[in2] as usize }
                else { i + 3 }
            }
            _ => panic!("Invalid position!"),
        };
    }
}
