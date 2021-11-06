mod day13 {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    fn is_open(x: i32, y: i32, designer_code: i32) -> bool {
        let sum = x*x + 3*x + 2*x*y + y + y*y + designer_code;
        let n_bits = (sum as f32).log2().floor() as i32 + 1;
        let n_on = (0..n_bits).filter(|i| sum & (1 << i) > 0).count();

        x >= 0 && y >= 0 && n_on % 2 == 0
    }

    fn shortest_path(start: (i32, i32), end: Option<(i32, i32)>,
                     designer_code: i32, max_dist: Option<i32>) -> (Option<i32>, usize) {
        let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();
        let mut visited: HashMap<(i32, i32), i32> = HashMap::new();

        let max_dist = max_dist.unwrap_or(i32::MAX);

        queue.push_back((start, 0));

        while let Some(((x, y), dist)) = queue.pop_front() {
            if visited.contains_key(&(x, y))
                || !is_open(x, y, designer_code)
                || dist > max_dist { continue; }

            for dxy in vec![-1, 1] {
                queue.push_back(((x + dxy, y), dist + 1));
                queue.push_back(((x, y + dxy), dist + 1));
            }

            visited.insert((x, y), dist);
            if Some((x, y)) == end { break; }
        }

        (end.map(|end| visited[&end]), visited.len())
    }

    pub fn solve_a(designer_code: i32) -> i32 {
        shortest_path((1, 1), Some((31, 39)), designer_code, None).0.unwrap()
    }

    pub fn solve_b(designer_code: i32) -> usize {
        shortest_path((1, 1), None, designer_code, Some(50)).1
    }

    #[cfg(test)]
    mod test_day13 {
        use super::*;

        #[test]
        fn test_is_open() {
            assert!(is_open(0, 0, 10));
            assert!(!is_open(1, 0, 10));
            assert!(is_open(2, 0, 10));
            assert!(!is_open(3, 0, 10));
            assert!(!is_open(4, 0, 10));
            assert!(!is_open(5, 0, 10));

            assert!(is_open(0, 1, 10));
            assert!(!is_open(0, 2, 10));

            assert!(is_open(1, 1, 10));

            assert!(is_open(7, 4, 10));
        }

        #[test]
        fn test_shortest_path() {
            assert_eq!(shortest_path((1, 1), Some((7, 4)), 10, None).0.unwrap(), 11);
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

    println!("Solution A-part: {}", day13::solve_a(buffer.trim().parse().unwrap()));
    println!("Solution A-part: {}", day13::solve_b(buffer.trim().parse().unwrap()));
}
