extern crate md5;

mod day05 {
    use md5;

    pub fn solve_a(door_id: &str) -> String {
        (0..).map(|i| md5::compute(format!("{}{}", door_id, i.to_string()).as_bytes()))
            .map(|digest| format!("{:x}", digest))
            .filter(|hash| &hash[..5] == "00000")
            .take(8)
            .map(|hash| hash.chars().nth(5).unwrap())
            .collect()
    }

    #[cfg(test)]
    mod test_day05 {
        use super::*;

        #[test]
        fn test_solve_ab() {
            assert_eq!(solve_a("abc"), "18f47a30");
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

    println!("Solution A-part: {}", day05::solve_a(&buffer.trim()));
}
