fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut freq_changes = Vec::new();
    for frequency in stdin.lock().lines() {
        freq_changes.push(frequency.unwrap().parse::<i32>().unwrap());
    }

    println!("{}", freq_changes.iter().sum::<i32>());
}
