use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut seats = stdin.lock().lines()
        .map(|s| s.unwrap().chars().map(|c| match c {
            'R' | 'B' => 1,
            'F' | 'L' => 0,
            _ => panic!("Code Error!"),
        }).rev().enumerate().map(|(i, d)| d<<i).sum()).collect::<Vec<u32>>();

    seats.sort_unstable();

    let seats = seats;

    let my_seat = seats.iter().zip(seats.iter().skip(1)).find(|(&s0, &s1)| s0 != s1 - 1).unwrap();

    println!("{:?}", my_seat.0+1);

}
