extern crate regex;

mod day21 {
    use regex::Regex;
    use std::collections::HashMap;

    enum Monkey {
        Val(i128),
        Op(((String, String), String)),
        Me(i128),
    }

    fn parse(input: &str) -> HashMap<String, Monkey> {
        let re_monkey = Regex::new(r"(?P<name>\w+): (?P<op>\d+|\w+ [+\-*/] \w+)").unwrap();

        re_monkey.captures_iter(input)
            .map(|capt| (
                    capt["name"].to_string(),
                    match capt["op"].parse::<i128>() {
                        Ok(v) => { 
                            if &capt["name"] == "humn" { Monkey::Me(v) }
                            else { Monkey::Val(v) }
                        },
                        Err(_) => {
                            let ops = capt["op"].split(' ').collect::<Vec<_>>();
                            Monkey::Op(((ops[0].to_string(), ops[2].to_string()), ops[1].to_string()))
                        }
                    }))
            .collect()
    }

    fn yell(target: &str, mut monkeys: &mut HashMap<String, Monkey>, rev: bool) -> Option<i128> {
        match &monkeys[target] {
            Monkey::Val(v) => { Some(*v) },
            Monkey::Me(v) => { if rev { None } else { Some(*v) }},
            Monkey::Op(((m1, m2), op)) => {
                let (m1, m2) = (m1.to_string(), m2.to_string());
                let op = op.to_string();

                let v1 = yell(&m1, &mut monkeys, rev);
                let v2 = yell(&m2, &mut monkeys, rev);

                match (v1, v2) {
                    (Some(v1), Some(v2)) => {
                        let res = match op.as_str() {
                            "+" => v1 + v2,
                            "-" => v1 - v2,
                            "*" => v1 * v2,
                            "/" => v1 / v2,
                            _ => unreachable!(),
                        };
                        monkeys.insert(target.to_string(), Monkey::Val(res));
                        Some(res)
                    }
                    _ => None,
                }
            }
        }
    }

    fn inverse_yell(monkey: &str, target: Vec<i128>, mut monkeys: &mut HashMap<String, Monkey>) -> Vec<i128> {
        match &monkeys[monkey] {
            Monkey::Val(v) => { panic!() },
            Monkey::Me(v) => target.to_vec(),
            Monkey::Op(((m1, m2), op)) => {
                let (mut m1, mut m2) = (m1.to_string(), m2.to_string());
                let op = op.to_string();

                let mut v1 = yell(&m1, &mut monkeys, true);
                let mut v2 = yell(&m2, &mut monkeys, true);
                if !v2.is_some() {
                    (m1, m2) = (m2, m1);
                }

                let target: Vec<i128> = match (v1, op.as_str(), v2) {
                    (Some(v), "+", None) => target.into_iter().map(|x| x - v).collect(),
                    (None, "+", Some(v)) => target.into_iter().map(|x| x - v).collect(),
                    (Some(v), "-", None) => target.into_iter().map(|x| v - x).collect(),
                    (None, "-", Some(v)) => target.into_iter().map(|x| x + v).collect(),
                    (Some(v), "*", None) => target.into_iter().filter_map(
                        |x| if x % v == 0 { Some(x / v) } else { None }).collect(),
                    (None, "*", Some(v)) => target.into_iter().filter_map(
                        |x| if x % v == 0 { Some(x / v) } else { None }).collect(),
                    (None, "/", Some(v)) => target.into_iter().flat_map(|x| (x*v..(x+1)*v)).collect(),
                    (Some(v), "/", None) => target.into_iter().flat_map(|x| (x*v..(x+1)*v)).collect(),
                    _ => unreachable!(),
                };

                inverse_yell(&m1, target, &mut monkeys)
            }
        }
    }

    pub fn solve_a(input: &str) -> i128 {
        let mut monkeys = parse(input);

        yell("root", &mut monkeys, false).unwrap()
    }

    pub fn solve_b(input: &str) -> i128 {
        let mut monkeys = parse(input);

        if let Monkey::Op(((m1, m2), _)) = &monkeys["root"] {
            let (mut m1, mut m2) = (m1.to_string(), m2.to_string());

            let mut v1 = yell(&m1, &mut monkeys, true);
            let mut v2 = yell(&m2, &mut monkeys, true);
            if !v2.is_some() {
                (v1, v2) = (v2, v1);
                (m1, m2) = (m2, m1);
            }

            let res_vec = inverse_yell(&m1, vec![v2.unwrap()], &mut monkeys);
            let res = res_vec[0];

            let mut monkeys = parse(input);
            monkeys.insert("humn".to_string(), Monkey::Val(res));
            let chk1 = yell(&m1, &mut monkeys, false);
            let chk2 = yell(&m2, &mut monkeys, false);

            assert_eq!(chk1.unwrap(), chk2.unwrap());

            res
        }
        else { panic!("Root is not a regular operation") }
    }

    #[cfg(test)]
    mod test_day21 {
        use super::*;

        static INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 152);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 301);
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

    println!("Solution A-part: {}", day21::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day21::solve_b(&buffer.trim()));
}
