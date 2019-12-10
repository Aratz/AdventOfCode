extern crate num_integer;

use std::io::{self, BufRead};
use std::collections::HashSet;
use num_integer::Integer;

fn main() {
    let stdin = io::stdin();

    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let carta: HashSet<(i32, i32)> = lines.iter().enumerate()
        .flat_map(|(y, l)| l.chars().enumerate()
                  .filter_map(move |(x, c)| if c == '#' { Some((x as i32, y as i32)) } else { None }))
        .collect();

    let base = carta.iter().map(|candidate|
             (
                 carta.iter().filter(|&ast| ast != candidate)
                 .filter_map(|asteroid| 
                     {
                         let (xx, yy) = (asteroid.0 - candidate.0, asteroid.1 - candidate.1);
                         let p = xx.gcd(&yy);
                         let step = (xx/p, yy/p);

                         match (1..p).map(|p| (candidate.0 + p*step.0, candidate.1 + p*step.1))
                             .filter(|c| carta.contains(&c)).count() {
                                 0 => Some(asteroid),
                                 _ => None,
                             }
                     }).count(),
                     candidate,
                     )
            ).max().unwrap();

    println!("Best possible location at {}, {} ({} asteroids detected)",
        (base.1).0, (base.1).1, base.0);
}
