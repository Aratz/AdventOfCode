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

    fn gen_hash(salt: &str, i: usize) -> String {
        format!("{:x}", md5::compute(format!("{}{}", salt, i.to_string()).as_bytes()))
    }

    fn find_keys(salt: &str) -> Vec<usize> {
        let mut five_c: HashMap<char, Vec<usize>> = HashMap::new();
        let mut keys: Vec<usize> = Vec::new();

        for i in 1..=1000 {
            let hash = gen_hash(salt, i);
            for c in find_nc(&hash, 5) {
                five_c.entry(c).or_default().push(i);
            }
        }

        for i in 0.. {
            let hash = gen_hash(salt, i);

            if let Some(c) = find_nc(&hash, 3).get(0) {
                if let Some(idx) = five_c.get(&c) {
                    if let Some(_) = idx.iter().find(|&j| i < *j && *j <= i + 1000) {
                        keys.push(i);
                    }
                }
            }

            {
                let hash = gen_hash(salt, i + 1001);
                for c in find_nc(&hash, 5) {
                    five_c.entry(c).or_default().push(i + 1001);
                }
            }

            if keys.len() == 64 { break; }
        }

        keys
    }

    pub fn solve_a(salt: &str) -> usize {
        find_keys(salt)[63]
    }

    #[cfg(test)]
    mod test_day14 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let keys = find_keys("abc");

            assert_eq!(keys[0], 39);
            assert_eq!(keys[1], 92);
            assert_eq!(keys[63], 22728);
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
}
