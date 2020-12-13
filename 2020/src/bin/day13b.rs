use std::io::{self, BufRead};

/// Compute s and t such that a*s + b*t = gcd(a, b)
///
/// Returns (gcd(a, b), (s, t))
///
/// This is an implementation of the [extended Euclidian algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
fn xgcd(a: i64, b: i64) -> (i64, (i64, i64)) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;

        let temp_r = old_r;
        old_r = r;
        r = temp_r - quotient*r;

        let temp_s = old_s;
        old_s = s;
        s = temp_s - quotient*s;

        let temp_t = old_t;
        old_t = t;
        t = temp_t - quotient*t;

    }

    (old_r, (old_s, old_t))
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _current_time:i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let bus_table = lines.next().unwrap().unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, v)| match v.parse() {
            Ok(v) => Some((i as i64, v)),
            Err(_) => None,
        })
        .collect::<Vec<(i64, i64)>>();

    // This is an application of the Chinese remainder theorem
    let a:Vec<i64> = bus_table.iter()
        .map(|&(i, ni)| ni-i)
        .collect();
    let n:Vec<i64> = bus_table.iter()
        .map(|&(_i, ni)| ni)
        .collect();

    let n_prod:i64 = n.iter().product();

    let n_hat:Vec<i64> = n.iter()
        .map(|ni| n_prod/ni)
        .collect();

    let v:Vec<i64> = n.iter().zip(n_hat.iter())
        .map(|(&ni, &ni_hat)| (xgcd(ni, ni_hat).1).1)
        .collect();

    let e:Vec<i64> = v.iter().zip(n_hat.iter())
        .map(|(vi, ni_hat)| vi*ni_hat)
        .collect();

    let mut res:i64 = e.iter().zip(a.iter())
        .map(|(ei, ai)| ei*ai)
        .sum();

    while res < 0 {
        res += n_prod;
    }

    println!("{}", res%n_prod);

}
