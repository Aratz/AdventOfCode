mod day02 {
    use std::cmp::{min, max};

    fn find_key(start: i32, seq: &str) -> i32 {
        let (x, y) = ((start - 1) % 3, (start - 1) / 3);
        let (x, y) = seq.chars().fold(
            (x, y), |(x, y), c| match c {
                'U' => (x, max(0, y - 1)),
                'D' => (x, min(2, y + 1)),
                'R' => (min(2, x + 1), y),
                'L' => (max(0, x - 1), y),
                _ => panic!("Instruction error!"),
            });
        3*y + x + 1
    }

    pub fn solve_a(insts: &Vec<String>) -> String {
        let mut sol = String::new();
        let mut start = 5;

        for inst in insts.iter() {
            start = find_key(start, inst);
            sol.push_str(&start.to_string());
        }

        sol
    }


    #[cfg(test)]
    mod test_day02 {
        use super::*;

        #[test]
        fn test_find_key() {
            assert_eq!(find_key(5, "ULL"), 1);
            assert_eq!(find_key(1, "RRDDD"), 9);
            assert_eq!(find_key(9, "LURDL"), 8);
            assert_eq!(find_key(8, "UUUUD"), 5);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<_> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    println!("Solution A-part: {}", day02::solve_a(&lines));
}
