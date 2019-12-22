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

fn quick_exp(a: i64, n: i64, N: i64) -> i64 {
    let mut res = 1;
    let mut a = a;
    let mut n = n;

    while n > 0 {
        if n % 2 == 1 {
            res = mult_noverflow(res, a, N);
        }
        a = mult_noverflow(a, a, N);
        n /= 2;
    }

    res
}
fn mult_noverflow(a: i64, b: i64, N: i64) -> i64 {
    let mut res = 0;
    let mut a = a;
    let mut b = b;

    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % N;
        }
        a = (a *2 ) % N;
        b = b / 2;
    }

    res
}

fn main() {
    const N: i64 = 119315717514047;

    const I: i64 = 2020;

    let stdin = io::stdin();

    let mut process: Vec<Technique> = stdin.lock().lines()
        .map(|l| parse_action(&l.unwrap())).collect();

    process.reverse();

    let mut i = I;

    for k in (0..101741582076661 as i64) {
        for p in process.iter() {
            match p {
                Technique::DealStack => { i = N - i - 1; },
                Technique::Cut(n) => { i = (N + i + n)%N; },
                Technique::DealInc(n) => { i = mult_noverflow(i, quick_exp(*n, N-2, N), N); },
            }
        }
    }
    println!("{}", i);
}
