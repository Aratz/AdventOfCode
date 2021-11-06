extern crate md5;
extern crate fancy_regex;

mod day14 {
    use md5;
    use fancy_regex::Regex;
    use std::collections::HashMap;

    fn find_nc(hash: &str, n: usize) -> Vec<char> {
        let reg = Regex::new(&format!(r"(.)\1{{{}}}", n - 1)).unwrap();

        reg.find_iter(hash)
            .map(|m| m.unwrap().as_str().chars().next().unwrap())
            .collect()
    }

    fn gen_hash(salt: &str, i: usize, n: usize) -> String {
        let mut hash = format!("{}{}", salt, i.to_string());
        for _ in 0..n {
            hash = format!("{:x}", md5::compute(hash.as_bytes()));
        }

        hash
    }

    fn find_keys(salt: &str, n: usize) -> Vec<usize> {
        let mut five_c: HashMap<char, Vec<usize>> = HashMap::new();
        let mut keys: Vec<usize> = Vec::new();
        let mut hashes: Vec<String> = Vec::new();

        let hash = gen_hash(salt, 0, n);
        hashes.push(hash);

        for i in 1..=1000 {
            let hash = gen_hash(salt, i, n);
            for c in find_nc(&hash, 5) {
                five_c.entry(c).or_default().push(i);
            }
            hashes.push(hash);
        }

        for i in 0.. {
            let hash = &hashes[i];

            if let Some(c) = find_nc(hash, 3).get(0) {
                if let Some(idx) = five_c.get(&c) {
                    if let Some(_) = idx.iter().find(|&j| i < *j && *j <= i + 1000) {
                        keys.push(i);
                    }
                }
            }

            {
                let hash = gen_hash(salt, i + 1001, n);
                for c in find_nc(&hash, 5) {
                    five_c.entry(c).or_default().push(i + 1001);
                }
                hashes.push(hash);
            }

            if keys.len() == 64 { break; }
        }

        keys
    }

    pub fn solve_a(salt: &str) -> usize {
        find_keys(salt, 1)[63]
    }

    pub fn solve_b(salt: &str) -> usize {
        find_keys(salt, 2017)[63]
    }

    #[cfg(test)]
    mod test_day14 {
        use super::*;

        #[test]
        fn test_gen_hash() {
            assert_eq!(gen_hash("abc", 0, 2017), "a107ff634856bb300138cac6568c0f24");
        }
    }
}

fn main() {
    use std::io::{self, Read};

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    println!("Solution A-part: {}", day14::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day14::solve_b(&buffer.trim()));
}
