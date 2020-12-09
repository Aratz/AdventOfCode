use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let max_seat: u32 = stdin.lock().lines()
        .map(|s| s.unwrap().chars().map(|c| match c {
            'R' | 'B' => 1,
            'F' | 'L' => 0,
            _ => panic!("Code Error!"),
        }).rev().enumerate().map(|(i, d)| d<<i).sum()).max().unwrap();

    println!("{:?}", max_seat);

}
