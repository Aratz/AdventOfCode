extern crate regex;

mod day08 {
    use regex::Regex;

    fn parse(input: &str) -> String {
        let re_quote = Regex::new(r#"\\""#).unwrap();
        let item = re_quote.replace_all(input, r#"""#);

        let re_single_backslash = Regex::new(r"\\\\").unwrap();
        let item = re_single_backslash.replace_all(&item, r"\");

        let re_hex = Regex::new(r"\\x[[:xdigit:]]{2}").unwrap();
        let item = re_hex.replace_all(&item, "X");

        item.to_string()
    }

    fn deparse(input: &str) -> String {
        let re_quote = Regex::new(r#"""#).unwrap();
        let item = re_quote.replace_all(input, r#"ε""#);

        let re_single_backslash = Regex::new(r"\\").unwrap();
        let item = re_single_backslash.replace_all(&item, r"ε\");

        let re_escape = Regex::new(r"ε").unwrap();
        let item = re_escape.replace_all(&item, r"\");

        format!("\"{}\"", item.to_string())
    }

    #[inline]
    fn diff(input: &str, a: bool) -> usize {
        if a {
            input.len() + 2 - parse(input).len()
        }
        else {
            deparse(input).len() - input.len()
        }
    }

    #[inline]
    pub fn solve_ab(input: &str, a: bool) -> usize {
        input.lines().map(|s| diff(s, a)).sum()
    }

    #[cfg(test)]
    mod test_day08 {
        use super::*;

        #[test]
        fn test_parse() {
            assert_eq!(parse(r#"a\"aa\"\x27\\"#), String::from("a\"aa\"X\\"));
            assert_eq!(parse(r#"\\\xab"#), String::from(r"\X"));
        }

        #[test]
        fn test_deparse() {
            assert_eq!(deparse(r#""""#), String::from(r#""\"\"""#));
            assert_eq!(deparse(r#""aaa\"aaa""#), String::from(r#""\"aaa\\\"aaa\"""#));
            assert_eq!(deparse(r#""\x27""#), String::from(r#""\"\\x27\"""#));
        }

        #[test]
        fn test_diff() {
            assert_eq!(diff(r#""""#, true), 2);
            assert_eq!(diff(r#""abc""#, true), 5 - 3);
            assert_eq!(diff(r#""aaa\"aaa""#, true), 10 - 7);
            assert_eq!(diff(r#""\x27""#, true), 6 - 1);

            assert_eq!(diff(r#""""#, false), 6 - 2);
            assert_eq!(diff(r#""abc""#, false), 9 - 5);
            assert_eq!(diff(r#""aaa\"aaa""#, false), 16 - 10);
            assert_eq!(diff(r#""\x27""#, false), 11 - 6);
        }

        #[test]
        fn test_solve_a() {
            let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
            assert_eq!(solve_ab(input, true), 12);
        }

        #[test]
        fn test_solve_b() {
            let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
            assert_eq!(solve_ab(input, false), 19);
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

    println!("Solution A-part: {}", day08::solve_ab(&buffer.trim(), true));
    println!("Solution B-part: {}", day08::solve_ab(&buffer.trim(), false));
}
