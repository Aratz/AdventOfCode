extern crate regex;

mod day04 {
    use regex::Regex;
    use std::collections::HashMap;

    struct Room {
        name: Vec<String>,
        id: i32,
        checksum: String,
    }

    impl Room {
        fn compute_checksum(&self) -> String {
            let mut letter_count: HashMap<char, i32> = HashMap::new();
            for ch in self.name.join("").chars() {
                letter_count.entry(ch)
                    .and_modify(|e| { *e += 1 })
                    .or_insert(1);
            }
            let mut sorted_count: Vec<_> = letter_count.iter().collect();
            sorted_count.sort_unstable_by_key(|(&ch, &count)| (-count, ch));
            sorted_count.iter()
                .map(|(ch, _count)| ch.to_string())
                .take(5)
                .collect()
        }

        fn is_real(&self) -> bool {
            self.compute_checksum() == self.checksum
        }
    }

    pub fn solve_a(lines: &str) -> i32 {
        let reg_room = Regex::new(r"(?P<name>([a-z]*-)*)(?P<id>[0-9]*)\[(?P<checksum>[a-z]{5})\]").unwrap();
        reg_room.captures_iter(lines)
            .map(|c| Room {
                name: {
                    let mut letters: Vec<String> = c.name("name").unwrap().as_str()
                        .split("-").map(|s| s.into())
                        .collect();
                    letters.pop();
                    letters
                },
                id: c.name("id").unwrap().as_str().parse().unwrap(),
                checksum: c.name("checksum").unwrap().as_str().into(),
            })
            .filter_map(|room| if room.is_real() { Some(room.id) } else { None })
            .sum()
    }

    #[cfg(test)]
    mod test_day04 {
        use super::*;

        #[test]
        fn test_checksum() {
            assert_eq!(
                Room {
                    name:vec![
                        "aaaaa".to_string(),
                        "bbb".to_string(),
                        "z".to_string(),
                        "y".to_string(),
                        "x".to_string()],
                    id:123,
                    checksum:"abxyz".to_string()
                }.compute_checksum(),
                "abxyz"
                );
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

    println!("Solution A-part: {}", day04::solve_a(&buffer));
}
