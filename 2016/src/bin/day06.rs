mod day06 {
    use std::collections::HashMap;
    use std::hash::Hash;

    fn count_items<T: Eq + Hash>(items: &[T]) -> HashMap<&T, usize> {
        let mut counts: HashMap<&T, usize> = HashMap::new();

        for v in items {
            counts.entry(v).and_modify(|e| { *e += 1 }).or_insert(1);
        }

        counts
    }

    fn least_common<T: Eq + Hash>(items: &[T]) -> &T {
        let counts = count_items(&items);
        *counts.iter().min_by_key(|(_k, count)| *count).unwrap().0
    }

    fn most_common<T: Eq + Hash>(items: &[T]) -> &T {
        let counts = count_items(&items);
        *counts.iter().max_by_key(|(_k, count)| *count).unwrap().0
    }

    fn transpose<T: Copy>(items: &[Vec<T>]) -> Vec<Vec<T>> {
        let item_len = items[0].len();
        (0..item_len).map(
            |i| items.iter().map(|s| s[i]).collect::<Vec<T>>())
            .collect()
    }

    pub fn solve_a(codes: &Vec<Vec<char>>) -> String {
        let codes_t = transpose(codes);

        codes_t.iter().map(|letters| most_common(&letters)).collect()
    }

    pub fn solve_b(codes: &Vec<Vec<char>>) -> String {
        let codes_t = transpose(codes);

        codes_t.iter().map(|letters| least_common(&letters)).collect()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let codes: Vec<_> = stdin.lock().lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    println!("Solution A-part: {}", day06::solve_a(&codes));
    println!("Solution B-part: {}", day06::solve_b(&codes));
}
