fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut numbers = stdin.lock().lines().next().unwrap().unwrap().split(',')
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut i = 0;

    numbers[1] = 12;
    numbers[2] = 2;

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


    println!("{}", numbers[0]);
}
