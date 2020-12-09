#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Technique {
    DealStack,
    Cut(i64),
    DealInc(i64),
}

fn parse_action(action: &str) -> Technique {
    lazy_static! {
        static ref TECH: [Regex; 3] = [
            Regex::new(r"^deal into new stack$").unwrap(),
            Regex::new(r"^cut (-?\d+)$").unwrap(),
            Regex::new(r"^deal with increment (\d+)$").unwrap(),
            ];
    }

    for (i, re) in TECH.iter().enumerate() {
        if let Some(cap) = re.captures(action) {
            match i {
                0 => return Technique::DealStack,
                1 => return Technique::Cut(cap.get(1).unwrap().as_str().parse().unwrap()),
                2 => return Technique::DealInc(cap.get(1).unwrap().as_str().parse().unwrap()),
                _ => continue,
            }
        }
    }

    panic!("No match found!")
}

fn main() {
    const N: i64 = 119315717514047;

    let stdin = io::stdin();

    let process: Vec<Technique> = stdin.lock().lines()
        .map(|l| parse_action(&l.unwrap())).collect();

    let mut cards: Vec<_> = (0..N).collect();

    for p in process {
        match p {
            Technique::DealStack => { cards.reverse(); },
            Technique::Cut(n) => {
                let new_stack = cards.iter()
                    .cycle().skip(((N+n)%N) as usize)
                    .take(N as usize).map(|&v| v).collect();
                cards = new_stack;
            },
            Technique::DealInc(n) => { 
                let mut new_stack = cards.clone();
                for (c, i) in cards.iter().zip((0..N as usize).cycle().step_by(n as usize)) {
                    new_stack[i] = *c;
                }
                cards = new_stack;
            },
        }
    }

    for (i, &c) in cards.iter().enumerate() {
        if c == 2019 {
            println!("{:?}", i);
            return;
        }
    }
}
