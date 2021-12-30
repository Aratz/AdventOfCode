mod day14 {
    use std::collections::HashMap;

    type Template = HashMap<(char, char), usize>;
    type Rules = HashMap<(char, char), char>;

    struct Input {
        template: Template,
        rules: Rules,
        bounds: (char, char),
    }

    fn apply(
        template: &Template,
        rules: &Rules)
        -> Template {

        let mut new_chain = HashMap::new();

        for (&(a, b), &count) in template.iter() {
            let new_element = rules[&(a, b)];
            *new_chain.entry((a, new_element)).or_default() += count;
            *new_chain.entry((new_element, b)).or_default() += count;
        }

        new_chain
    }

    fn parse_input(s: &str)
        -> Input {
        let mut lines = s.lines();

        let template_raw: Vec<char> = lines.next().unwrap().chars().collect();

        let mut template = HashMap::new();

        for (&a, &b) in template_raw.iter().zip(template_raw.iter().skip(1)) {
            *template.entry((a, b)).or_default() += 1;
        }

        let rules: Rules = lines.skip(1)
            .map(|l| {
                let rule = l.split(" -> ")
                    .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
                ((rule[0][0], rule[0][1]), rule[1][0])
            }).collect();

        Input { template, rules, bounds: (template_raw[0], template_raw[template_raw.len() - 1]) }
    }

    fn count_elements(
        polymer: &Template,
        bounds: (char, char))
        -> HashMap<char, usize> {

        let mut counts: HashMap<char, usize> = HashMap::new();

        for (&(a, b), &count) in polymer {
            *counts.entry(a).or_default() += count;
            *counts.entry(b).or_default() += count;
        }

        *counts.entry(bounds.0).or_default() += 1;
        *counts.entry(bounds.1).or_default() += 1;

        counts.into_iter().map(|(a, count)| (a, count/2)).collect()
    }

    pub fn solve(input: &str, max_step: usize) -> usize {
        let Input { mut template, rules, bounds } = parse_input(input);

        for _ in 0..max_step {
            template = apply(&template, &rules);
        }

        let counts = count_elements(&template, bounds);

        counts.values().max().unwrap() - counts.values().min().unwrap()
    }

    #[cfg(test)]
    mod test_day14 {
        use super::*;

        #[test]
        fn test_solve() {
            let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
            assert_eq!(solve(&input, 10), 1588);
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

    println!("Solution A-part: {}", day14::solve(&buffer, 10));
    println!("Solution B-part: {}", day14::solve(&buffer, 40));
}
