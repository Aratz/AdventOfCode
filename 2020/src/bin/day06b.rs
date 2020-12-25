extern crate regex;

use std::io::{self, Read};
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re = Regex::new(r"([a-z]+(\s|$))+").unwrap();

    let res: usize = re.captures_iter(&buffer)
        .map(|c| c.iter().next().unwrap().unwrap().as_str()) // Extract match
        .map(|forms| forms
             .split('\n')
             .map(|form| form.chars().collect::<HashSet<_>>())
             .collect::<Vec<_>>().iter().rev().skip(1)
             .fold(
                 ('a'..='z').collect::<HashSet<_>>(),
                 |a, b| a.intersection(&b).copied().collect::<HashSet<_>>()
                 )
             .len()
             )
        .sum();

    println!("{}", res);
}
