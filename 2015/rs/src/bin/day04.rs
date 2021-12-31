extern crate md5;

mod day04 {
    use md5;

    fn find_hash(key: &str, n_zeroes: usize) -> usize {
        (1..).map(|i| (i, md5::compute(format!("{}{}", key, i.to_string()).as_bytes())))
            .map(|(i, digest)| (i, format!("{:x}", digest)))
            .find(|(_i, hash)| hash.starts_with(&"0".repeat(n_zeroes)))
            .unwrap().0
    }

    pub fn solve_a(input: &str) -> usize {
        find_hash(input, 5)
    }

    pub fn solve_b(input: &str) -> usize {
        find_hash(input, 6)
    }

    #[cfg(test)]
    mod test_day04 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("abcdef"), 609043);
            assert_eq!(solve_a("pqrstuv"), 1048970);
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

    println!("Solution A-part: {}", day04::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day04::solve_b(&buffer.trim()));
}
