extern crate regex;

mod day16 {
    use regex::Regex;
    use std::collections::HashMap;

    fn parse(input: &str) -> HashMap<usize, HashMap<String, i32>> {
        let re_aunt = Regex::new(r"Sue (?P<id>\d+):(?P<features>(?: \w+: \d+,?)+)").unwrap();
        let re_features = Regex::new(r"(?P<key>\w+): (?P<value>\d+)").unwrap();

        re_aunt.captures_iter(input)
            .map(|capt| (
                capt["id"].parse().unwrap(),
                re_features.captures_iter(&capt["features"])
                    .map(|capt| (
                        capt["key"].to_string(),
                        capt["value"].parse().unwrap()))
                    .collect()))
            .collect()
    }

    fn get_my_aunt() -> HashMap<String, i32> {
        vec![
            (String::from("children"), 3),
            (String::from("cats"), 7),
            (String::from("samoyeds"), 2),
            (String::from("pomeranians"), 3),
            (String::from("akitas"), 0),
            (String::from("vizslas"), 0),
            (String::from("goldfish"), 5),
            (String::from("trees"), 3),
            (String::from("cars"), 2),
            (String::from("perfumes"), 1),
        ].into_iter().collect()
    }

    pub fn solve_a(input: &str) -> usize {
        let my_aunt = get_my_aunt();
        let aunts = parse(input);

        aunts.into_iter()
            .find(|(_id, features)|
                features.into_iter()
                    .all(|(key, value)| my_aunt[key] == *value))
            .unwrap().0
    }

    pub fn solve_b(input: &str) -> usize {
        let my_aunt = get_my_aunt();
        let aunts = parse(input);

        aunts.into_iter()
            .find(|(_id, features)|
                features.into_iter()
                    .all(|(key, value)| match key.as_str() {
                        "cats" | "trees" => { my_aunt[key] < *value },
                        "pomeranians" | "goldfish" => { my_aunt[key] > *value },
                        _ => { my_aunt[key] == *value },
                    }))
            .unwrap().0
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

    println!("Solution A-part: {}", day16::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day16::solve_b(&buffer.trim()));
}
