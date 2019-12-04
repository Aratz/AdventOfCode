fn two_digits(n: &str) -> bool {
    let n_vec = n.chars().collect::<Vec<_>>();
    let mut i = 1;
    let mut count = 1;
    while i < n_vec.len() {
        let new_count = if n_vec[i - 1] == n_vec[i] {
            count + 1
        }
        else {
            if count == 2 {
                return true;
            }

            1
        };
        count = new_count;
        i += 1;
    }
    if count == 2 { true } else { false }
}
fn never_decrease(n: &str) -> bool {
    let mut n_sorted = n.chars().collect::<Vec<_>>();
    n_sorted.sort();
    n_sorted.into_iter().collect::<String>() == n
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let (a, b) = {
        let range = stdin.lock().lines().next().unwrap().unwrap().split("-")
        .map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        (range[0], range[1])
    };

    let sol = (a..b).map(|x| x.to_string())
        .filter(|x| two_digits(&x))
        .filter(|x| never_decrease(&x))
        .count();

    println!("{}", sol)
}
