fn main() {
    use std::io::{self, BufRead};
    use std::collections::HashMap;

    let stdin = io::stdin();

    let mut ancestors = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut words = line.split(" -> ");
        let parent = words.next().unwrap().split(" ").next().unwrap();

        if let Some(children) = words.next() {
            for child in children.split(", "){
                ancestors.insert(String::from(child), String::from(parent));
            }
        }
    }

    let mut program = ancestors.iter().next().unwrap().1;
    while let Some(ancestor) = ancestors.get(&program[..]) {
        program = ancestor;
    }

    println!("{}", program);
}
