extern crate regex;
extern crate evalexpr;

mod day11 {
    use regex::Regex;
    use evalexpr::{HashMapContext, ContextWithMutableVariables, Context, eval_with_context_mut};

    struct Monkey {
        items: Vec<i64>,
        op: String,
        test: i64,
        target: (usize, usize),
    }

    fn parse(input: &str) -> Vec<Monkey> {
        let re_monkey = Regex::new(r"Monkey \d:\s+Starting items: (?P<items>(?:\d+(?:, )?)+)\s+Operation: (?P<op>.*)\s+Test: divisible by (?P<test>\d+)\s+If true: throw to monkey (?P<target_true>\d+)\s+If false: throw to monkey (?P<target_false>\d+)").unwrap();

        re_monkey.captures_iter(input)
            .map(|capt| {
                Monkey {
                    items: capt["items"]
                        .split(", ")
                        .map(|val| val.parse().unwrap())
                        .collect(),
                    op: capt["op"].to_string(),
                    test: capt["test"].parse().unwrap(),
                    target: (
                        capt["target_true"].parse().unwrap(),
                        capt["target_false"].parse().unwrap()
                        ),
                }
            }).collect()
    }

    pub fn solve_ab(input: &str, hard: bool) -> usize {
        let mut monkeys = parse(input);
        let mut activity = vec![0; monkeys.len()];

        let modulo: i64 = monkeys.iter().map(|m| m.test).product();

        for _round in 0..(if !hard { 20 } else { 10000 }) {
            for i_monkey in 0..monkeys.len() {
                activity[i_monkey] += monkeys[i_monkey].items.len();
                let targets: Vec<_> = monkeys[i_monkey].items.iter()
                    .map(|item| {
                         let mut context = HashMapContext::new();
                         context.set_value("old".into(), (*item).into()).unwrap();
                         eval_with_context_mut(&monkeys[i_monkey].op, &mut context).unwrap();
                         let mut item = context.get_value("new").unwrap().as_int().unwrap();
                         if !hard {
                            item /= 3;
                         }
                         else {
                            item %= modulo;
                         }

                         let target = if item % monkeys[i_monkey].test == 0 {
                             monkeys[i_monkey].target.0
                         } else {
                             monkeys[i_monkey].target.1
                         };

                         (target, item)
                    }).collect();
                monkeys[i_monkey].items.clear();
                for (target, item) in targets.into_iter() {
                    monkeys[target].items.push(item);
                }
            }
        }

        activity.select_nth_unstable(monkeys.len() - 2);
        activity[activity.len() - 1] * activity[activity.len() - 2]
    }

    #[cfg(test)]
    mod test_day11 {
        use super::*;

        static INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_ab(&INPUT, false), 10605);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_ab(&INPUT, true), 2713310158);
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

    println!("Solution A-part: {}", day11::solve_ab(&buffer.trim(), false));
    println!("Solution B-part: {}", day11::solve_ab(&buffer.trim(), true));
}
