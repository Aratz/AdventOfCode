mod day10 {
    fn scheduler(input: &str) -> Vec<i32> {
        let mut clock = 1;
        let mut time_reg = vec![1; 240 + 2];

        for line in input.lines() {
            let words: Vec<_> = line.split(' ').collect();
            match words[0] {
                "addx" => {
                    time_reg[clock + 1] = time_reg[clock];
                    time_reg[clock + 2] = time_reg[clock] + words[1].parse::<i32>().unwrap();
                    clock += 2;
                },
                "noop" => {
                    time_reg[clock + 1] = time_reg[clock];
                    clock += 1;
                },
                _ => unreachable!(),
            }
        }

        time_reg
    }

    pub fn solve_a(input: &str) -> i32 {
        let schedule = scheduler(input);
        schedule.into_iter()
            .enumerate()
            .skip(20).step_by(40)
            .map(|(clock, regx)| (clock as i32)*regx)
            .sum()
    }

    pub fn solve_b(input: &str) {
        let schedule = scheduler(input);
        let mut crt = vec![vec![' '; 40]; 6];
        for clock in 1..=240 {
            if schedule[clock].abs_diff((clock as i32 - 1) % 40) <= 1 {
                crt[(clock - 1) / 40][(clock - 1) % 40] = '▮';
            }
        }

        for line in crt.into_iter() {
            println!("{}", line.into_iter().collect::<String>());
        }
    }

    #[cfg(test)]
    mod test_day10 {
        use super::*;

        static INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(&INPUT), 13140);
        }
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

    println!("Solution A-part: {}", day10::solve_a(&buffer.trim()));
    println!("Solution B-part:"); day10::solve_b(&buffer.trim());
}
