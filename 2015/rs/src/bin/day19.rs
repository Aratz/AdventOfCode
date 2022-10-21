extern crate regex;

mod day19 {
    use regex::Regex;
    use std::cmp::Reverse;
    use std::collections::{HashSet, BinaryHeap};

    fn parse(input: &str) -> (Vec<(String, String)>, String) {
        let rules_target = input.split("\n\n").collect::<Vec<&str>>();
        let (rules, target) = (rules_target[0], rules_target[1].to_string());

        (
            rules.lines()
                .map(|l| {
                    let io = l.split(" => ").collect::<Vec<&str>>();
                    (io[0].to_string(), io[1].to_string())
                })
                .collect(),
            target)
    }

    pub fn solve_a(input: &str) -> usize {
        let (rules, target) = parse(input);

        let target = &target;

        rules.into_iter()
            .flat_map(|(i, o)| {
                let re_rule = Regex::new(&i).unwrap();
                re_rule.find_iter(&target)
                    .map(move |mtch| {
                        let mut target = target.clone();
                        target.replace_range(mtch.range(), &o);
                        target
                    }).collect::<Vec<String>>()
            })
            .collect::<HashSet<String>>()
            .len()
    }

    pub fn solve_b(input: &str) -> usize {
        let (rules, target) = parse(input);

        let rules = rules.into_iter()
            .map(|(i, o)| (Regex::new(&o).unwrap(), i))
            .collect::<Vec<_>>();

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(target.len()), Reverse(0), target.clone()));

        let mut product_map = HashSet::new();
        product_map.insert(target.clone());

        loop {
            let (_, Reverse(step), product) = queue.pop().unwrap();

            if product == String::from("e") {
                return step;
            }

            for (re_rule, i) in rules.iter() {
                for mtch in re_rule.find_iter(&product) {
                    let mut base = product.clone();
                    base.replace_range(mtch.range(), i);
                    if !product_map.contains(&base) {
                        product_map.insert(base.clone());
                        queue.push((
                                Reverse(base.len() + step + 1),
                                Reverse(step + 1),
                                base));
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod test_day19 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let rules = "H => HO
H => OH
O => HH";

            assert_eq!(solve_a(&format!("{}\n\n{}", rules, "HOH")), 4);
            assert_eq!(solve_a(&format!("{}\n\n{}", rules, "HOHOHO")), 7);
        }

        #[test]
        fn test_solve_b() {
            let rules = "H => HO
H => OH
O => HH
e => H
e => O";

            assert_eq!(solve_b(&format!("{}\n\n{}", rules, "HOH")), 3);
            assert_eq!(solve_b(&format!("{}\n\n{}", rules, "HOHOHO")), 6);
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

    println!("Solution A-part: {}", day19::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day19::solve_b(&buffer.trim()));
}
