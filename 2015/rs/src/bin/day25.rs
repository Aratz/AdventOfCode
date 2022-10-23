extern crate regex;

mod day25 {
    use regex::Regex;

    fn parse(input: &str) -> (u128, u128) {
        let re_input = Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (?P<row>\d+), column (?P<col>\d+).").unwrap();

        let cap = re_input.captures(input).unwrap();

        (cap["row"].parse().unwrap(), cap["col"].parse().unwrap())
    }

    pub fn solve_a(input: &str) -> u128 {
        let target = parse(input);

        let mut pos = (1, 1);
        let mut code = 20151125;

        while pos != target {
            if pos.0 == 1 {
                pos.0 = pos.1 + 1;
                pos.1 = 1;
            }
            else {
                pos.0 -= 1;
                pos.1 += 1;
            }

            code = (code * 252533) % 33554393;
        }

        code
    }

    #[cfg(test)]
    mod test_day25 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "To continue, please consult the code grid in the manual.  Enter the code at row 6, column 6.";

            assert_eq!(solve_a(&input), 27995004);
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

    println!("Solution A-part: {}", day25::solve_a(&buffer.trim()));
}
