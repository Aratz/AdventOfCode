extern crate regex;

mod day18 {
    use regex::Regex;

    fn compute_regexpr(parsed_expr: &mut regex::CaptureMatches) -> Result<u64, String> {
        let mut last_operator = None;
        let first_token = parsed_expr.next();
        let mut acc = match first_token {
            Some(raw_token) => {
                let token = raw_token.get(1).unwrap().as_str();
                match token.parse::<u64>() {
                    Ok(n) => { n },
                    Err(_) => match token {
                        "(" => { compute_regexpr(parsed_expr).unwrap() },
                        ")" | "+" | "*" => { return Err(String::from("Parsing failed")); },
                        _ => unreachable!(),
                    },
                }
            },
            None => { return Err(String::from("Empty expression")); },
        };

        while let Some(raw_token) = parsed_expr.next() {
            let token = raw_token.get(1).unwrap().as_str();
            match token.parse::<u64>() {
                Ok(n) => match last_operator {
                    Some("+") => { acc += n; last_operator = None; },
                    Some("*") => { acc *= n; last_operator = None; },
                    None => { return Err(String::from("Parsing failed (two numbers in a row)")); },
                    _ => unreachable!(),
                },
                Err(_) => match token {
                    "(" => {
                        let n = compute_regexpr(parsed_expr).unwrap();
                        match last_operator {
                            Some("+") => { acc += n; last_operator = None; },
                            Some("*") => { acc *= n; last_operator = None; },
                            None => { return Err(String::from("Parsing failed (two numbers in a row)")); },
                            _ => unreachable!(),
                        }
                    },
                    ")" => { return Ok(acc); }
                    "+" | "*" => match last_operator {
                        Some(_) => { return Err(String::from("Parsing failed (two operators in a row)")); },
                        None => { last_operator = Some(token); },
                    },
                    _ => unreachable!(),
                },
            }
        }

        Ok(acc)
    }

    pub fn compute_expr(expr: &str) -> u64 {
        let re_expr = Regex::new(r"(\d+|\*|\+|\(|\))").unwrap();
        compute_regexpr(&mut re_expr.captures_iter(expr)).unwrap()
    }

    #[cfg(test)]
    mod test_day18 {
        use super::*;

        #[test]
        fn test_compute_expr() {
            assert_eq!(compute_expr("1 + 2 * 3 + 4 * 5 + 6"), 71);
            assert_eq!(compute_expr("1 + (2 * 3) + (4 * (5 + 6))"), 51);
            assert_eq!(compute_expr("2 * 3 + (4 * 5)"), 26);
            assert_eq!(compute_expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
            assert_eq!(compute_expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
            assert_eq!(compute_expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
        }

    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    println!("Solution A-part: {}", stdin.lock().lines().map(
            |line| day18::compute_expr(&line.unwrap())).sum::<u64>());

}
