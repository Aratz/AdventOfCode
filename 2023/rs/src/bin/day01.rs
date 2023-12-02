extern crate regex;

mod day01 {
    use regex::Regex;

    fn parse_a(input: &str) -> Vec<u32> {
        input.lines()
            .map(|l| {
                let first_dg = l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .next().unwrap();
                let last_dg = l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .last().unwrap();
                10*first_dg + last_dg
            }).collect()
    }

    pub fn solve_a(input: &str) -> u32 {
        parse_a(input).into_iter().sum()
    }

    fn to_digit(input: &str) -> u32 {
        match input.parse() {
            Ok(digit) => { digit },
            Err(_) => {
                match input {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn parse_b(input: &str) -> Vec<u32> {
        let letters = r"one|two|three|four|five|six|seven|eight|nine";
        let re_digit = Regex::new(&format!("{}|{}", "[0-9]", letters)).unwrap();
        let re_digit_rev = Regex::new(
            &format!(
                "{}|{}", "[0-9]",
                letters.chars().rev().collect::<String>().as_str())).unwrap();

        input.lines()
            .map(|l| {
                let first_dg = to_digit(re_digit.find(l).unwrap().as_str());
                let last_dg = to_digit(
                    re_digit_rev
                    .find(l.chars().rev().collect::<String>().as_str()).unwrap().as_str()
                    .chars().rev().collect::<String>().as_str());
                10*first_dg + last_dg
            }).collect()
    }

    pub fn solve_b(input: &str) -> u32 {
        parse_b(input).into_iter().sum()
    }

    #[cfg(test)]
    mod test_day01 {
        use super::*;

        static INPUT_A: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        static INPUT_B: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT_A), 142);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT_B), 281);
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
