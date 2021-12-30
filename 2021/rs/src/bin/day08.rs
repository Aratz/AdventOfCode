mod day08 {
    use std::collections::HashSet;
    use std::collections::HashMap;

    pub fn solve_a(entries: &[(Vec<String>, Vec<String>)]) -> usize {
        entries.iter()
            .flat_map(|(_patterns, output)| output.iter())
            .filter(|digit| match digit.len() {
                2 | 3 | 4 | 7 => { true },
                _ => { false },
            })
            .count()
    }

    fn is_subset(a: &str, b: &str) -> bool {
        let sub: HashSet<_> = a.chars().collect();
        let sup: HashSet<_> = b.chars().collect();

        sub.is_subset(&sup)
    }

    fn decode(patterns: &[String]) -> HashMap<String, char> {
        let mut mapping: HashMap<String, char> = HashMap::new();
        let mut rev_mapping: HashMap<char, String> = HashMap::new();

        for p in patterns {
            match p.len() {
                2 => { mapping.insert(p.into(), '1'); rev_mapping.insert('1', p.into()); },
                3 => { mapping.insert(p.into(), '7'); rev_mapping.insert('7', p.into()); },
                4 => { mapping.insert(p.into(), '4'); rev_mapping.insert('4', p.into()); },
                7 => { mapping.insert(p.into(), '8'); rev_mapping.insert('8', p.into()); },
                _ => {},
            }
        }

        let nine = patterns.iter()
            .find(|p| p.len() == 6 && is_subset(&rev_mapping[&'4'], p)).unwrap();
        mapping.insert(nine.to_string(), '9');
        rev_mapping.insert('9', nine.to_string());

        let three = patterns.iter()
            .find(|p| p.len() == 5 && is_subset(&rev_mapping[&'7'], p)).unwrap();
        mapping.insert(three.to_string(), '3');
        rev_mapping.insert('3', three.to_string());

        let zero = patterns.iter()
            .find(|p| p.len() == 6 && p != &&rev_mapping[&'9'] && is_subset(&rev_mapping[&'7'], p)).unwrap();
        mapping.insert(zero.to_string(), '0');
        rev_mapping.insert('0', zero.to_string());

        let six = patterns.iter()
            .find(|p| p.len() == 6 && p != &&rev_mapping[&'9'] && p != &&rev_mapping[&'0']).unwrap();
        mapping.insert(six.to_string(), '6');
        rev_mapping.insert('6', six.to_string());

        let five = patterns.iter()
            .find(|p| p.len() == 5 && is_subset(p, &rev_mapping[&'6'])).unwrap();
        mapping.insert(five.to_string(), '5');
        rev_mapping.insert('5', five.to_string());

        let two = patterns.iter()
            .find(|p| p.len() == 5 && p != &&rev_mapping[&'5'] && p != &&rev_mapping[&'3']).unwrap();
        mapping.insert(two.to_string(), '2');
        rev_mapping.insert('2', two.to_string());

        mapping
    }

    pub fn solve_b(entries: &[(Vec<String>, Vec<String>)]) -> usize {
        entries.iter().map(|(patterns, output)| {
            let mapping = decode(patterns);
            output.iter()
                .map(|d| { mapping[d] })
                .collect::<String>()
                .parse::<usize>().unwrap()
        }).sum()
    }
}

fn main () {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let entries = stdin.lock().lines()
        .map(|l| {
            let entry = l.unwrap().split(" | ")
             .map(|s| s.split(' ')
                  .map(|d| {
                      let mut d = d.chars().collect::<Vec<_>>();
                      d.sort_unstable();
                      d.iter().collect::<String>()
                  }).collect::<Vec<String>>())
             .collect::<Vec<_>>();
            (entry[0].clone(), entry[1].clone())
             })
        .collect::<Vec<_>>();

    println!("Solution A-part: {}", day08::solve_a(&entries));
    println!("Solution B-part: {}", day08::solve_b(&entries));
}
