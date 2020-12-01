fn main() {
    use std::io::{self, BufRead};
    use std::collections::HashMap;

    let stdin = io::stdin();


    let mut two = 0;
    let mut three = 0;

    for id in stdin.lock().lines(){
        let mut letter_count = HashMap::new();

        for c in id.unwrap().chars(){
            let count = letter_count.entry(c).or_insert(0);
            *count += 1;
        }

        if letter_count.iter().any(|(_, n)| *n==2) {
            two += 1;
        }

        if letter_count.iter().any(|(_, n)| *n==3) {
            three += 1;
        }

    }

    println!("{}", two * three);
}
