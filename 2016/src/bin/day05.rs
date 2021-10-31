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

    pub fn solve_b(door_id: &str) -> String {
        let mut password = ['X'; 8];
        let mut password_findr = (0..)
            .map(|i| md5::compute(format!("{}{}", door_id, i.to_string()).as_bytes()))
            .map(|digest| format!("{:x}", digest))
            .filter(|hash| &hash[..5] == "00000")
            .map(|hash| (
                    hash.chars().nth(5).unwrap().to_digit(16).unwrap() as usize,
                    hash.chars().nth(6).unwrap())
                )
            .filter(|(pos, _ch)| pos < &8);

        for _ in 0..8 {
            loop {
                let (pos, ch) = password_findr.next().unwrap();
                if password[pos] == 'X' {
                    password[pos] = ch;
                    break;
                }
            }
        }

        password.iter().collect()
    }

    #[cfg(test)]
    mod test_day05 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("abc"), "18f47a30");
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b("abc"), "05ace8e3");
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
    println!("Solution B-part: {}", day05::solve_b(&buffer.trim()));
}
