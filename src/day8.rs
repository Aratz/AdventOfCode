fn main() {
    use std::io::{self, BufRead};
    use std::collections::HashMap;

    let stdin = io::stdin();
    let pass = stdin.lock().lines().next().unwrap().unwrap();

    let (w, h) = (25, 6);

    let mut max_layer: Option<HashMap<char, u32>> = None;

    let mut pass_it = pass.chars().enumerate();

    let mut c_count: HashMap<char, u32> = HashMap::new();
    while let Some((i, c)) = pass_it.next() {
        if i % (w * h) == 0 && i != 0 {
            max_layer = match max_layer {
                Some(max_count) => if c_count[&'0'] < max_count[&'0']
                    { Some(c_count) } else { Some(max_count) },
                None =>  Some(c_count),
            };
            c_count = HashMap::new();
        }

        c_count.entry(c).and_modify(|e| { *e += 1 }).or_insert(1);
    }

    let max_layer = max_layer.unwrap();
    println!("{}", max_layer[&'1'] * max_layer[&'2']);
}
