fn main() {
    use std::io::{self, BufRead};
    use std::collections::hash_map::{HashMap, Entry};

    let stdin = io::stdin();

    let mut ancestors = HashMap::new();
    let mut descendants:HashMap<String, Vec<String>> = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut words = line.split(" -> ");
        let parent = words.next().unwrap().split(" ").next().unwrap();

        if let Some(children) = words.next() {
            for child in children.split(", "){
                ancestors.insert(String::from(child), String::from(parent));

                match descendants.entry(String::from(parent)) {
                    Entry::Vacant(e) => { e.insert(vec![String::from(child)]);},
                    Entry::Occupied(mut e) => { e.get_mut().push(String::from(child)); },
                }
            }
        }
    }

    let mut program = ancestors.iter().next().unwrap().1;
    while let Some(ancestor) = ancestors.get(&program[..]) {
        program = ancestor;
    }

    println!("{}", program);
}
