use std::cmp;
use std::io::{self, Read};
use std::collections::HashMap;

fn dist(pos: (i32, i32)) -> i32 {
    cmp::min(
        pos.0.abs() + pos.1.abs(),
        (pos.0 - pos.1).abs() + cmp::min(pos.0.abs(), pos.1.abs())
        )
}

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


    let max_dist = buffer.split(",")
        .map(|dir| coord_map[dir.trim_end_matches("\n")])
        .scan(
            (0, 0),
            |state, b| {
                *state = (state.0 + b.0, state.1 + b.1);
                Some(*state)
            })
        .map(|pos| dist(pos))
        .max().unwrap();

    println!("{}", max_dist);
}
