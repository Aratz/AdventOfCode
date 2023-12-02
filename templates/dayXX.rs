mod dayXX {
    pub fn solve_a(input: &str) -> i32 {
        0
    }

    pub fn solve_b(input: &str) -> i32 {
        0
    }

    #[cfg(test)]
    mod test_dayXX {
        use super::*;

        static INPUT: &str = "";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 0);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 0);
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

    println!("Solution A-part: {}", dayXX::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", dayXX::solve_b(&buffer.trim()));
}
