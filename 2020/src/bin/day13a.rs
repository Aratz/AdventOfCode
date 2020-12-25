use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let current_time:i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let bus_table = lines.next().unwrap().unwrap()
        .split(',')
        .map(|v| v.parse())
        .collect::<Vec<Result<i32, _>>>();

    let res = bus_table.iter()
        .filter_map(|t| match t {
                    Ok(t) => Some((t - (current_time) % t, t)),
                    Err(_) => None,
        }).min().unwrap();

    println!("{:?}", res.0 * res.1);
}
