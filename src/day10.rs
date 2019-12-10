extern crate num_integer;

use std::io::{self, BufRead};
use std::collections::HashSet;
use num_integer::Integer;
use std::cmp::max;

fn main() {
    let stdin = io::stdin();

    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let carta: HashSet<(i32, i32)> = lines.iter().enumerate()
        .flat_map(|(y, l)| l.chars().enumerate()
                  .filter_map(move |(x, c)| if c == '#' { Some((x as i32, y as i32)) } else { None }))
        .collect();

    let mut max_count = 0;

    for candidate in carta.iter() {
        let mut count = 0;

        for asteroid in carta.iter() {
            if candidate == asteroid { continue }

            let (xx, yy) = (asteroid.0 - candidate.0, asteroid.1 - candidate.1);

            let p = xx.gcd(&yy);

            if p == 1 {
                count += 1;
                continue;
            }

            let step = (xx/p, yy/p);

            count += match (1..p).map(|p| (candidate.0 + p*step.0, candidate.1 + p*step.1))
                .filter(|c| carta.contains(&c)).next() {
                    Some(_) => 0,
                    None => 1
                }
        }

        max_count = max(max_count, count);
    }

    println!("{}", max_count);
}
