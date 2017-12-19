fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let digits = stdin.lock().lines().next()
        .expect("there was no next line")
        .expect("the line could not be read");

    let digits_circ = String::from(&digits[digits.len()/2..])
        + &String::from(&digits[..&digits.len()/2]);

    let mut result = 0;
    for (d1, d2) in digits.chars().zip(digits_circ.chars()) {
        if d1 == d2 {
            result += d1.to_digit(10).unwrap();
        }
    }
    println!("{}", result);

}
