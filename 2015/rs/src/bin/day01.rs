mod day01 {
    pub fn solve_a(input: &str) -> i32 {
        input.chars()
            .map(|c| if c == '(' { 1 } else { -1 })
            .sum::<i32>()
    }

    pub fn solve_b(input: &str) -> usize {
        input.chars()
            .map(|c| if c == '(' { 1 } else { -1 })
            .scan(0, |state, x| {
                *state = *state + x;
                Some(*state)
            }).enumerate()
            .find(|&(_, floor)| floor == -1)
            .unwrap().0 + 1
    }

    #[cfg(test)]
    mod test_day01 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("(())"), 0);
            assert_eq!(solve_a("(()(()("), 3);
            assert_eq!(solve_a("))((((("), 3);
            assert_eq!(solve_a("())"), -1);
            assert_eq!(solve_a(")())())"), -3);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(")"), 1);
            assert_eq!(solve_b("()())"), 5);
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

    println!("Solution A-part: {}", day01::solve_a(&buffer));
    println!("Solution B-part: {}", day01::solve_b(&buffer));
}
