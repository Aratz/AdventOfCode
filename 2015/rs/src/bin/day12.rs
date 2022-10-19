extern crate regex;
extern crate serde_json;

mod day12 {
    use regex::Regex;
    use serde_json::Value;

    pub fn solve_a(input: &str) -> i64 {
        let re_num = Regex::new(r"-?\d+").unwrap();

        re_num.captures_iter(input)
            .map(|capt| capt[0].parse::<i64>().unwrap())
            .sum()
    }

    fn sum_filter_red(doc: &Value) -> i64 {
        match doc {
            Value::Number(v) => v.as_i64().unwrap(),
            Value::Array(vec) => {
                vec.iter()
                    .map(|v| sum_filter_red(v))
                    .sum()
            }
            Value::Object(map)
                if !map.contains_key("red")
                    && !map.values().any(|v| *v == Value::String("red".to_string())) => {
                map.values()
                    .map(|v| sum_filter_red(v))
                    .sum()
            }
            _ => { 0 },
        }
    }

    pub fn solve_b(input: &str) -> i64 {
        let doc = serde_json::from_str(input).unwrap();
        sum_filter_red(&doc)
    }

    #[cfg(test)]
    mod test_day12 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("[1,2,3]"), 6);
            assert_eq!(solve_a("{\"a\":2,\"b\":4}"), 6);
            assert_eq!(solve_a("{\"a\":{\"b\":4},\"c\":-1}"), 3);
            assert_eq!(solve_a("[[[3]]]"), 3);
            assert_eq!(solve_a("{\"a\":[-1,1]}"), 0);
            assert_eq!(solve_a("[-1,{\"a\":1}]"), 0);
            assert_eq!(solve_a("{}"), 0);
            assert_eq!(solve_a("[]"), 0);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b("[1,2,3]"), 6);
            assert_eq!(solve_b("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
            assert_eq!(solve_b("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
            assert_eq!(solve_b("[1,\"red\",5]"), 6);
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

    println!("Solution A-part: {}", day12::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day12::solve_b(&buffer.trim()));
}
