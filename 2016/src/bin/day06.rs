mod day06 {
    use std::collections::HashMap;
    use std::hash::Hash;

    fn most_common<T: Eq + Hash + Copy>(items: &[T]) -> T {
        let mut counts: HashMap<T, usize> = HashMap::new();

        for &v in items {
            counts.entry(v).and_modify(|e| { *e += 1 }).or_insert(1);
        }

        *counts.iter().max_by_key(|(_k, count)| *count).unwrap().0
    }

    pub fn solve_a(codes: &Vec<Vec<char>>) -> String {
        let code_len = codes[0].len();
        let codes_t = (0..code_len).map(
            |i| codes.iter().map(|s| s[i]).collect::<Vec<char>>())
            .collect::<Vec<_>>();

        codes_t.iter().map(|letters| most_common(&letters)).collect()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let codes: Vec<_> = stdin.lock().lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    println!("Solution A-part: {}", day06::solve_a(&codes));
}
