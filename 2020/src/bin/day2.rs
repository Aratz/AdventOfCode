extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug)]
struct PasswordPolicy<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let ppolicies = lines.iter()
        .map(|l| re.captures_iter(l)
                   .map(|c| PasswordPolicy {
                       min: c.get(1).unwrap().as_str().parse::<_>().unwrap(),
                       max: c.get(2).unwrap().as_str().parse::<_>().unwrap(),
                       letter: c.get(3).unwrap().as_str().chars().next().unwrap(),
                       password: c.get(4).unwrap().as_str(),
                   })
                   .next().unwrap())
        .collect::<Vec<_>>();
    let res = ppolicies.iter().filter(|pp| {
        let count = pp.password.matches(pp.letter).count();
        pp.min <= count && count <= pp.max
    }).count();

    println!("{}", res);
}
