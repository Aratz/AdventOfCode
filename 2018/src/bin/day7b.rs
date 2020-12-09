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

    let mut time:HashMap<char, u32> = HashMap::new();
    for c in order.chars() {
        let anc = match h2.get(&c).unwrap().0.iter()
            .map(|n| time.get(n).unwrap()).max() {
                Some(x) => *x,
                None => 0,
            };
        let t = time.entry(c).or_insert(0);
        *t = anc + duration(c);

        println!("{} {}", c, *t);
    }

}
