mod day01 {
    pub fn solve_a(input: &str) -> i32 {
        input.split("\n\n")
            .map(|pack| pack
                 .split("\n")
                 .map(|item| item.parse::<i32>().unwrap())
                 .sum())
            .max().unwrap()
    }

    pub fn solve_b(input: &str) -> i32 {
        let mut packs = input.split("\n\n")
            .map(|pack| pack
                 .split("\n")
                 .map(|item| item.parse::<i32>().unwrap())
                 .sum())
            .collect::<Vec<_>>();
        packs.sort();

        packs[packs.len()-3..].iter().sum()
    }

    #[cfg(test)]
    mod test_day01 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

            assert_eq!(solve_a(&input), 24000);
        }

        #[test]
        fn test_solve_b() {
            let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

            assert_eq!(solve_b(&input), 45000);


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

    println!("Solution A-part: {}", day01::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day01::solve_b(&buffer.trim()));
}
