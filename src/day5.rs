fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let chain = stdin.lock().lines().next().unwrap().unwrap();

    println!("{}", chain.chars().fold(String::from(""), |mut acc, c| {
        let l = acc.chars().last();
        match l {
            None => acc.push(c),
            Some(l) => if l.to_lowercase().to_string() == c.to_lowercase().to_string()
                && (l.is_lowercase() && c.is_uppercase()
                    || l.is_uppercase() && c.is_lowercase()) {
                    acc.pop();
                }
                else {
                    acc.push(c);
                },

        }
        acc
    }).chars().count());
}
