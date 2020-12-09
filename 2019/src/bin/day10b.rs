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

    let base: (i32, i32) = (17, 22);

    let mut targets = carta.iter().filter(|&ast| ast != &base).map(|asteroid| {
            let (xx, yy) = (asteroid.0 - base.0, asteroid.1 - base.1);
            let p = xx.gcd(&yy);
            let step = (xx/p, yy/p);

            (
                (1..p).map(|p| (base.0 + p*step.0, base.1 + p*step.1))
                .filter(|c| carta.contains(&c)).count(),
                (-(xx as f64) - 0.01).atan2(yy as f64),
                asteroid,
                )
    }).collect::<Vec<_>>();

    targets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let sol = targets[199].2;
    println!("{}", sol.0*100 + sol.1);

}
