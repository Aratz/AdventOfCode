extern crate regex;

mod day19 {
    use std::collections::HashMap;
    use regex::Regex;

    const N_MATCH: usize = 6;

    #[derive(Clone, Debug)]
    pub enum Rule {
        Simple(String),
        Complex(Vec<Vec<usize>>),
    }

    fn generate_regex_b(start: usize, mut rules: &mut HashMap<usize, Rule>) {
        let mut origin = rules[&start].clone();
        let reg = match start { 
            8 => {
                generate_regex_b(42, &mut rules);
                let rule42 = rules[&42].clone();
                match rule42 {
                    Rule::Simple(s) => { format!("({})+", s) },
                    Rule::Complex(_) => unreachable!(),
                }
            }
            11 => {
                generate_regex_b(42, &mut rules);
                let rule42 = match rules[&42].clone() {
                    Rule::Simple(s) => { s },
                    Rule::Complex(_) => unreachable!(),
                };
                generate_regex_b(31, &mut rules);
                let rule31 = match rules[&31].clone() {
                    Rule::Simple(s) => { s },
                    Rule::Complex(_) => unreachable!(),
                };

                let vec_reg: Vec<String> = (1..=N_MATCH).map(|i| format!(
                        r"({}){{{}}}({}){{{}}}", rule42, i, rule31, i))
                    .collect();
                format!("({})", vec_reg.join("|"))
            }
            _ => match origin {
                Rule::Simple(s) => { s },
                Rule::Complex(subrules) => {
                    let vec_reg:Vec<String> = subrules.iter().map(
                        |seq| {
                            let vec_seq:Vec<String> = seq.iter().map(
                                |&rule| {
                                    generate_regex_b(rule, &mut rules);
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
            }
        };

        origin = Rule::Simple(reg);
        rules.insert(start, origin);
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

    pub fn solve_a(rules: &mut HashMap<usize, Rule>, messages: &[String]) -> usize {
        generate_regex(0, rules);

        let reg_string = match &rules[&0] {
            Rule::Simple(s) => { s },
            Rule::Complex(_) => unreachable!(),
        };

        let reg_rules = Regex::new(&format!(r"^{}$", reg_string)).unwrap();

        messages.iter().filter(|msg| reg_rules.is_match(msg)).count()
    }

    pub fn solve_b(rules: &mut HashMap<usize, Rule>, messages: &[String]) -> usize {
        generate_regex_b(0, rules);

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
    use std::collections::HashMap;

    let reg_rule = Regex::new(r#"(?P<id>\d+): ((?P<complex>(\d+ ?)+(\| (\d+ ?)+)?)|"(?P<simple>[ab])")"#).unwrap();
    let reg_msg = Regex::new(r"([ab]{2,})").unwrap();

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let rules: HashMap<usize, day19::Rule> = reg_rule.captures_iter(&buffer).map(
            |c| (
                c.name("id").unwrap().as_str().parse().unwrap(),
                match c.name("simple") {
                    Some(s) => { day19::Rule::Simple(s.as_str().into()) },
                    None => {
                        day19::Rule::Complex(c.name("complex").unwrap().as_str()
                            .split(" | ").map(
                                |subrules| subrules.trim().split(' ').map(
                                    |rule| rule.parse().unwrap()).collect()
                                ).collect())
                    },
                })
        ).collect();

    let messages: Vec<String> = reg_msg.captures_iter(&buffer).map(
        |c| c.get(1).unwrap().as_str().into()
        ).collect();

    let mut rules_a = rules.clone(); 
    let mut rules_b = rules;

    println!("Solution A-part: {}", day19::solve_a(&mut rules_a, &messages));
    println!("Solution B-part: {}", day19::solve_b(&mut rules_b, &messages));
}
