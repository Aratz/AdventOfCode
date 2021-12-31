mod dayXX {
    pub fn solve_a(input: &str) -> {

    }

    #[cfg(test)]
    mod test_dayXX {
        use super::*;

        #[test]
        fn test_solve_a() {

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
