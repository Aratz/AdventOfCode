extern crate regex;

mod day19 {
    use std::collections::HashMap;
    use regex::Regex;

    #[derive(Clone, Debug)]
    pub enum Rule {
        Simple(String),
        Complex(Vec<Vec<usize>>),
    }

    fn generate_regex(start: usize, mut rules: &mut HashMap<usize, Rule>) {
        let mut origin = rules[&start].clone();
        let reg = match origin {
            Rule::Simple(s) => { s },
            Rule::Complex(subrules) => {
                let vec_reg:Vec<String> = subrules.iter().map(
                    |seq| {
                        let vec_seq:Vec<String> = seq.iter().map(
                            |&rule| {
                                generate_regex(rule, &mut rules);
                                let subrule = rules[&rule].clone();
                                match subrule {
                                    Rule::Simple(s) => { s },
                                    Rule::Complex(_) => unreachable!(),
                                }
                            }).collect();
                        vec_seq.join("")
                    }).collect();
                format!("({})", vec_reg.join("|"))
            },
        };

        origin = Rule::Simple(reg);
        rules.insert(start, origin);
    }

    pub fn solve_a(rules: &mut HashMap<usize, Rule>, messages: &Vec<String>) -> usize {
        generate_regex(0, rules);

        let reg_string = match &rules[&0] {
            Rule::Simple(s) => { s },
            Rule::Complex(_) => unreachable!(),
        };

        let reg_rules = Regex::new(&format!(r"^{}$", reg_string)).unwrap();

        messages.iter().filter(|msg| reg_rules.is_match(msg)).count()
    }

    #[cfg(test)]
    mod test_day19 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let mut rules = vec![
                (0, Rule::Complex(vec![vec![4, 1, 5]])),
                (1, Rule::Complex(vec![vec![2,3], vec![3, 2]])),
                (2, Rule::Complex(vec![vec![4, 4], vec![5, 5]])),
                (3, Rule::Complex(vec![vec![4, 5], vec![5, 4]])),
                (4, Rule::Simple("a".into())),
                (5, Rule::Simple("b".into())),
            ].into_iter().collect();

            let messages = vec![
                "ababbb".into(),
                "bababa".into(),
                "abbbab".into(),
                "aaabbb".into(),
                "aaaabbb".into(),
            ];

            assert_eq!(solve_a(&mut rules, &messages), 2);
        }
    }
}

fn main() {
    use regex::Regex;
    use std::io::{self, Read};

    let reg_rule = Regex::new(r#"(?P<id>\d+): ((?P<complex>(\d+ ?)+(\| (\d+ ?)+)?)|"(?P<simple>[ab])")"#).unwrap();
    let reg_msg = Regex::new(r"([ab]{2,})").unwrap();

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let mut rules = reg_rule.captures_iter(&buffer).map(
            |c| (
                c.name("id").unwrap().as_str().parse().unwrap(),
                match c.name("simple") {
                    Some(s) => { day19::Rule::Simple(s.as_str().into()) },
                    None => {
                        day19::Rule::Complex(c.name("complex").unwrap().as_str()
                            .split(" | ").map(
                                |subrules| subrules.trim().split(" ").map(
                                    |rule| rule.parse().unwrap()).collect()
                                ).collect())
                    },
                })
        ).collect();

    let messages = reg_msg.captures_iter(&buffer).map(
        |c| c.get(1).unwrap().as_str().into()
        ).collect();

    println!("Solution A-part: {}", day19::solve_a(&mut rules, &messages));
}
