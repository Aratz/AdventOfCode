extern crate md5;

mod day17 {
    use md5;
    use std::cmp::max;
    use std::collections::VecDeque;

    fn pathfinder(password: &str) -> (Result<String, &'static str>, usize) {
        let start = (0, 3);
        let end = (3, 0);
        let directions = vec!["U", "D", "L", "R"];

        let mut shortest_path = Err("Path not found");
        let mut longest_path = 0;

        let mut queue: VecDeque<((i32, i32), String)> = VecDeque::new();
        queue.push_back((start, String::from("")));

        while let Some(((x, y), path)) = queue.pop_front() {
            if (x, y) == end {
                if shortest_path.is_err() {
                    shortest_path = Ok(path);
                }
                else {
                    longest_path = max(longest_path, path.len());
                }
                continue;
            }

            let hash = format!(
                "{:x}", 
                md5::compute(format!("{}{}", password, path.to_string()).as_bytes()));
            for (i, (dx, dy)) in vec![(0, 1), (0, -1), (-1, 0), (1, 0)].iter().enumerate() {
                if hash.chars().nth(i).unwrap() > 'a'
                    && 0 <= x + dx && x + dx < 4
                    && 0 <= y + dy && y + dy < 4 {
                    queue.push_back((
                            (x + dx, y + dy),
                            format!("{}{}", path, directions[i])));
                }
            }
        }

        (shortest_path, longest_path)
    }

    pub fn solve_a(password: &str) -> String {
        pathfinder(password).0.unwrap()
    }

    pub fn solve_b(password: &str) -> usize {
        pathfinder(password).1
    }

    #[cfg(test)]
    mod test_day17 {
        use super::*;

        #[test]
        fn test_pathfinder_short() {
            assert_eq!(pathfinder("ihgpwlah").0, Ok(String::from("DDRRRD")));
            assert_eq!(pathfinder("kglvqrro").0, Ok(String::from("DDUDRLRRUDRD")));
            assert_eq!(pathfinder("ulqzkmiv").0, Ok(String::from("DRURDRUDDLLDLUURRDULRLDUUDDDRR")));
        }

        #[test]
        fn test_pathfinder_long() {
            assert_eq!(pathfinder("ihgpwlah").1, 370);
            assert_eq!(pathfinder("kglvqrro").1, 492);
            assert_eq!(pathfinder("ulqzkmiv").1, 830);
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

    println!("Solution A-part: {}", day17::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day17::solve_b(&buffer.trim()));
}
