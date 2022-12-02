mod day02 {
    pub fn solve_a(input: &str) -> i32 {
        input.lines()
            .map(|l| {
                let mut chars = l.chars();
                let opp = (chars.next().unwrap() as u8 - 'A' as u8) as i32;
                chars.next();
                let me = (chars.next().unwrap() as u8 - 'X' as u8) as i32;
                (me, (opp - me + 3) % 3)
            })
            .map(|(me, outcome)| match outcome {
                0 => { 3 },
                1 => { 0 },
                2 => { 6 },
                _ => unreachable!(),
            } + me + 1)
            .sum()
    }

    pub fn solve_b(input: &str) -> i32 {
        input.lines()
            .map(|l| {
                let mut chars = l.chars();
                let opp = (chars.next().unwrap() as u8 - 'A' as u8) as i32;
                chars.next();
                let outcome = (chars.next().unwrap() as u8 - 'X' as u8) as i32;
                (
                    match outcome {
                        0 => { (opp + 2) % 3 },
                        1 => { opp },
                        2 => { (opp + 1) % 3 },
                        _ => unreachable!(),
                    },
                    outcome)
            })
            .map(|(me, outcome)| match outcome {
                0 => { 0 },
                1 => { 3 },
                2 => { 6 },
                _ => unreachable!(),
            } + me + 1)
            .sum()
    }

    #[cfg(test)]
    mod test_day02 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "A Y
B X
C Z";
            assert_eq!(solve_a(&input), 15);
        }

        #[test]
        fn test_solve_b() {
            let input = "A Y
B X
C Z";
            assert_eq!(solve_b(&input), 12);

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

    println!("Solution A-part: {}", day02::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day02::solve_b(&buffer.trim()));
}
