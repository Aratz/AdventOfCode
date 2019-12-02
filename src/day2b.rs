fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap().split(",")
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    for noun in 0..100 {
        for verb in 0..100 {

            let mut numbers = numbers.to_vec();

            let mut i = 0;

            numbers[1] = noun;
            numbers[2] = verb;

            loop {
                if numbers[i] == 99 {break;}

                let in1 = numbers[i+1];
                let in2 = numbers[i+2];
                let out = numbers[i+3];
                numbers[out] = match numbers[i] {
                    1 => numbers[in1] + numbers[in2],
                    2 => numbers[in1] * numbers[in2],
                    _ => panic!("Invalid position!"),
                };

                i += 4;
            }

            if numbers[0] == 19690720 {
                println!("{}", 100*noun + verb);
                std::process::exit(0);
            }
        }
    }
}
