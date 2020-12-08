#[macro_use] extern crate lazy_static;
extern crate regex;

const M:i128 = 119315717514047;
const K: i128 = 101741582076661;


use regex::Regex;
use std::io::{self, BufRead};

fn mult_noverflow(a: i128, b: i128, m: i128) -> i128 {
    let mut res = 0;
    let mut a = a;
    let mut b = b;

    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m;
        }
        a = (a *2 ) % m;
        b = b / 2;
    }

    res
}

fn compose(f: (i128, i128), g: (i128, i128)) -> (i128, i128) {
    (mult_noverflow(f.0, g.0, M), (mult_noverflow(f.0, g.1, M) + f.1) % M)
}

fn quick_exp(mut f_exp: (i128, i128), k: i128) -> (i128, i128) { // OK
    let mut f = (1, 0);
    let mut exp = 0;

    while (1 << exp) <= k {
        if K & (1 << exp) > 0 {
            f = compose(f_exp, f);
        }
        f_exp = compose(f_exp, f_exp);
        exp += 1;
    }

    f
}

fn xgcd(a: i128, b: i128) -> (i128, (i128, i128)) { // OK
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;

        let temp_r = old_r;
        old_r = r;
        r = temp_r - mult_noverflow(quotient, r, b);

        let temp_s = old_s;
        old_s = s;
        s = (temp_s - mult_noverflow(quotient, s, b) +b)%b ;

        let temp_t = old_t;
        old_t = t;
        t = (temp_t - mult_noverflow(quotient, t, b) +b)%b;

    }

    (old_r, (old_s, old_t))
}

fn parse_action(action: &str) -> (i128, i128) {
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
                0 => return (M-1, M-1), // OK
                1 => return (
                    1,
                    (M + cap.get(1).unwrap().as_str().parse::<i128>().unwrap()) % M), // OK
                2 => {
                    let n = cap.get(1).unwrap().as_str().parse::<i128>().unwrap();
                    return (((xgcd(n, M).1).0 + M) % M, 0) // OK
                },
                _ => continue,
            }
        }
    }

    panic!("No match found!")
}

fn main() {
    let stdin = io::stdin();

    let process: Vec<(i128, i128)> = stdin.lock().lines()
        .map(|l| parse_action(&l.unwrap())).collect();

    let f_exp = process.iter().fold((1, 0), |acc, &x| {
        compose(acc, x)
    });
    let i = 2020;
    let f = quick_exp(f_exp, K);

    println!("{:?}", (mult_noverflow(i, f.0, M) + f.1) % M);
}
