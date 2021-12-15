extern crate regex;

mod day18 {
    use regex::Regex;

    fn compute_regexpr_a(parsed_expr: &mut regex::CaptureMatches) -> Result<u64, String> {
        let mut last_operator = None;
        let first_token = parsed_expr.next();
        let mut acc = match first_token {
            Some(raw_token) => {
                let token = raw_token.get(1).unwrap().as_str();
                match token.parse::<u64>() {
                    Ok(n) => { n },
                    Err(_) => match token {
                        "(" => { compute_regexpr_a(parsed_expr).unwrap() },
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
                        let n = compute_regexpr_a(parsed_expr).unwrap();
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

    pub fn compute_expr_a(expr: &str) -> u64 {
        let re_expr = Regex::new(r"(\d+|\*|\+|\(|\))").unwrap();
        compute_regexpr_a(&mut re_expr.captures_iter(expr)).unwrap()
    }

    fn compute_regexpr_b(parsed_expr: &mut regex::CaptureMatches) -> Result<u64, String> {
        let mut acc = match parsed_expr.next() {
            Some(raw_token) => {
                let token = raw_token.get(1).unwrap().as_str();
                match token.parse::<u64>() {
                    Ok(n) => { n },
                    Err(_) => match token {
                        "(" => { compute_regexpr_b(parsed_expr).unwrap() },
                        ")" | "+" | "*" => { return Err(String::from("Malformed expression")); },
                        _ => unreachable!(),
                    },
                }
            },
            None => { return Err(String::from("Empty expression")); },
        };

        match parsed_expr.next() {
            Some(raw_token) => {
                let token =raw_token.get(1).unwrap().as_str();
                match token {
                    "+" => {},
                    "*" => { return Ok(acc * compute_regexpr_b(parsed_expr)?);},
                    ")" => { return Ok(acc); },
                    _ => { return Err(format!("Malformed expression (two numbers in a row({}))", token)); },
                }
            },
            None => { return Ok(acc); },
        };

        while let Some(raw_token) = parsed_expr.next() {
            let token = raw_token.get(1).unwrap().as_str();
            match token.parse::<u64>() {
                Ok(n) => { acc += n; },
                Err(_) => match token {
                    "(" => {
                        let n = compute_regexpr_b(parsed_expr)?;
                        acc += n;
                    },
                    ")" => { return Ok(acc); }
                    "*" => { return Ok(acc * compute_regexpr_b(parsed_expr)?);}
                    "+" => { },
                    _ => unreachable!(),
                },
            }
        }

        Ok(acc)
    }

    pub fn compute_expr_b(expr: &str) -> u64 {
        let re_expr = Regex::new(r"(\d+|\*|\+|\(|\))").unwrap();
        compute_regexpr_b(&mut re_expr.captures_iter(expr)).unwrap()
    }


    #[cfg(test)]
    mod test_day18 {
        use super::*;

        #[test]
        fn test_compute_expr_a() {
            assert_eq!(compute_expr_a("1 + 2 * 3 + 4 * 5 + 6"), 71);
            assert_eq!(compute_expr_a("1 + (2 * 3) + (4 * (5 + 6))"), 51);
            assert_eq!(compute_expr_a("2 * 3 + (4 * 5)"), 26);
            assert_eq!(compute_expr_a("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
            assert_eq!(compute_expr_a("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
            assert_eq!(compute_expr_a("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
        }

        #[test]
        fn test_compute_expr_b() {
            assert_eq!(compute_expr_b("1 + 2 * 3 + 4 * 5 + 6"), 231);
            assert_eq!(compute_expr_b("1 + (2 * 3) + (4 * (5 + 6))"), 51);
            assert_eq!(compute_expr_b("2 * 3 + (4 * 5)"), 46);
            assert_eq!(compute_expr_b("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
            assert_eq!(compute_expr_b("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
            assert_eq!(compute_expr_b("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<_> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    println!("Solution A-part: {}", lines.iter().map(
            |line| day18::compute_expr_a(&line)).sum::<u64>());
    println!("Solution B-part: {}", lines.iter().map(
            |line| day18::compute_expr_b(&line)).sum::<u64>());

}
