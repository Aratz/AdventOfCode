fn main() {
    use std::io::{self, BufRead};

    let input = 1;

    let stdin = io::stdin();
    let mut numbers = stdin.lock().lines().next().unwrap().unwrap().split(',')
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut i = 0;

    loop {
        let opcode = numbers[i] % 100;

        if opcode == 99 {break;}


        //let in1 = numbers[i+1];
        //let in2 = numbers[i+2];
        let out = match opcode {
            1 | 2 => numbers[i+3] as usize,
            3 => numbers[i+1] as usize,
            4 => if numbers[i]/100 % 10 == 1 { i + 1 } else { numbers[i+1] as usize },
            _ => panic!("Invalid position!"),
        };

        numbers[out] = match opcode {
            1 | 2 => {
                let in1 = if numbers[i]/100 % 10 == 1 { i + 1 }
                    else { numbers[i+1] as usize };
                let in2 = if numbers[i]/1000 % 10 == 1 { i + 2 }
                    else { numbers[i+2] as usize };

                if opcode == 1 { numbers[in1] + numbers[in2]}
                    else { numbers[in1] * numbers[in2] }},
            3 => { input },
            4 => { println!("Opcode 4: {}", numbers[out]); numbers[out] },
            _ => panic!("Invalid position!"),
        };

        i += match opcode {
            1 | 2 => 4,
            3 | 4 => 2,
            _ => panic!("Invalid position!"),
        };
    }


    println!("{}", numbers[0]);
}
