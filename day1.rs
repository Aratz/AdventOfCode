fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let digits = stdin.lock().lines().next()
        .expect("there was no next line")
        .expect("the line could not be read");

    let mut result = 0;
    let mut iterator = digits.chars().peekable();
    while let Some(d) = iterator.next() {
        match iterator.peek() {
            Some(&n) => if n == d { result += d.to_digit(10).unwrap(); },
            None => if d == digits.chars().next().unwrap() { result += d.to_digit(10).unwrap() },
        }
    }

    println!("{}", result);
}
