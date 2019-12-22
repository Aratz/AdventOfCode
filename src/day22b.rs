#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io::{self, BufRead};
use std::collections::HashMap;

const N: i64 = 119315717514047;
const I: i64 = 2020;

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

fn quick_exp(a: i64, n: i64) -> i64 {
    let mut res = 1;
    let mut a = a;
    let mut n = n;

    while n > 0 {
        if n % 2 == 1 {
            res = mult_noverflow(res, a);
        }
        a = mult_noverflow(a, a);
        n /= 2;
    }

    res
}
fn mult_noverflow(a: i64, b: i64) -> i64 {
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

fn fast_p(i: i64, l: i64, data: &mut HashMap<(i64, i64), i64>, process: &[Technique]) {
    if data.contains_key(&(i, l)) {println!("HIT!"); return };

    let mut j = i;

    if l == 1 {
        for p in process.iter() {
            match p {
                Technique::DealStack => { j = N - j - 1; },
                Technique::Cut(n) => { j = (N + j + n)%N; },
                Technique::DealInc(n) => { j = mult_noverflow(j, quick_exp(*n, N-2)); },
            }
        }
    }
    else {
        fast_p(j, l/2, data, process);
        j = data[&(j, l/2)];
        fast_p(j, l/2, data, process);
        j = data[&(j, l/2)];
        if l % 2 == 1 {
            fast_p(j, 1, data, process);
            j = data[&(j, 1)];
        }
    }
    data.insert((i, l), j);
}


fn main() {
    let stdin = io::stdin();

    let mut process: Vec<Technique> = stdin.lock().lines()
        .map(|l| parse_action(&l.unwrap())).collect();

    process.reverse();

    let mut i = I;
    let l = 101741582076661;

    let mut data: HashMap<(i64, i64), i64> = HashMap::new();

    fast_p(i, l, &mut data, &process);
    println!("{}", data[&(i, l)]);
}
