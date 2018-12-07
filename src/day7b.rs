fn duration(c:char) -> u32 {
    c as u32 - 'A' as u32 + 1 + 60
}

fn main() {
    use std::collections::{HashMap, HashSet};
    use std::io::{self, BufRead};

    let mut h:HashMap<char, (HashSet<char>, HashSet<char>)> = HashMap::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let first_letters = line.unwrap().split(" ").map(|s| s.chars().next().unwrap())
            .collect::<Vec<char>>();
        {
            let node = h.entry(first_letters[7]).or_insert((HashSet::new(), HashSet::new()));
            (*node).0.insert(first_letters[1]);
        }

        {
            let node = h.entry(first_letters[1]).or_insert((HashSet::new(), HashSet::new()));
            (*node).1.insert(first_letters[7]);
        }
    }

    let h2 = h.clone();
    let mut S = h.keys().filter(|&k| h.get(&k).unwrap().0.is_empty())
        .map(|c| *c).collect::<Vec<char>>();
    let mut order = String::from("");
    S.sort_by(|a, b| b.cmp(a));

    while ! S.is_empty() {
        let head = S.pop().unwrap();
        order.push(head);
        let outer = h.get(&head).unwrap().1.clone();
        for child in outer {
            h.entry(child).and_modify(|e| { (*e).0.remove(&head); });

            if h.get(&child).unwrap().0.is_empty() {
                S.push(child);
            }
        }
        S.sort_by(|a, b| b.cmp(a));
    }
    println!("{}", order);

    let n_workers = 5;

    let mut levels:Vec<HashSet<char>> = vec![HashSet::new(); order.len()];
    for c in order.chars().rev() {
        let mut max_level = match levels.iter().enumerate()
            .filter(|&(_, nodes)| nodes.iter().any(|n| h2.get(n).unwrap().0.contains(&c)))
            .map(|(i, _)| i).max() {
                Some(x) => x + 1,
                None => 0,
            };
        while levels[max_level].len() >= n_workers {
            max_level += 1;
        }

        levels[max_level].insert(c);
    }
    println!("{}", levels.iter()
             .map(|nodes| match nodes.iter().map(|&c| duration(c)).max() {
                Some(x) => x,
                None => 0,
             })
             .sum::<u32>());
}
