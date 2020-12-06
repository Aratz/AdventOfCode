use std::cmp;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let coord_map = vec![
        ("n", (0, 1)),
        ("ne", (1, 1)),
        ("se", (1, 0)),
        ("s", (0, -1)),
        ("sw", (-1, -1)),
        ("nw", (-1, 0)),
    ].iter()
        .map(|(k, v)| (String::from(*k), *v))
        .collect::<HashMap<String, (i32, i32)>>();


    let current_pos = buffer.split(",")
        .map(|dir| coord_map[dir.trim_end_matches("\n")])
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    let dist = cmp::min(
        current_pos.0.abs() + current_pos.1.abs(),
        (current_pos.0 - current_pos.1).abs()
            + cmp::min(current_pos.0.abs(), current_pos.1.abs()));

    println!("{}", dist);
}
