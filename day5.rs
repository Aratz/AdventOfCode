fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut list = Vec::new();
    for number in stdin.lock().lines() {
        list.push(number.unwrap().parse::<i32>().unwrap());
    }

    let mut current:i32 = 0;
    let mut nsteps = 0;
    while 0 <= current && current < list.len() as i32 {
        nsteps += 1;
        list[current as usize] += 1;
        current += list[current as usize] - 1;
    }

    println!("{}", nsteps);
}
