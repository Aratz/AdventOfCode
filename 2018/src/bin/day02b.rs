fn distance(a: &String, b: &String) -> usize {
    a.chars().zip(b.chars()).filter(|&(c_a, c_b)| c_a != c_b).count()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut ids = Vec::new();
    for id in stdin.lock().lines() {
        ids.push(id.unwrap());
    }

    for id_1 in ids.iter() {
        for id_2 in ids.iter() {
            if distance(id_1, id_2) == 1 {
                println!("{}", id_1.chars().zip(id_2.chars()).filter(|&(c_a, c_b)| c_a == c_b)
                    .map(|(c_a, _)| c_a).collect::<String>());
                return ();
            }
        }
    }
}
