mod day02 {
    fn find_key(
            keypad: &Vec<Vec<char>>,
            start: (usize, usize),
            seq: &str) -> (usize, usize) {
        let (y, x) = start;
        seq.chars().fold(
            (y, x), |(y, x), c| {
                let new_coord = match c {
                    'U' => (y - 1, x),
                    'D' => (y + 1, x),
                    'R' => (y, x + 1),
                    'L' => (y, x - 1),
                    _ => panic!("Instruction error!"),
                };

                if keypad[new_coord.0][new_coord.1] != 'X' { new_coord }
                else { (y, x) }
            })
    }

    pub fn solve_a(insts: &Vec<String>) -> String {
        let mut sol = String::new();
        let mut start = (2, 2);
        let keypad = vec![
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', '1', '2', '3', 'X'],
            vec!['X', '4', '5', '6', 'X'],
            vec!['X', '7', '8', '9', 'X'],
            vec!['X', 'X', 'X', 'X', 'X'],
        ];

        for inst in insts.iter() {
            start = find_key(&keypad, start, inst);
            sol.push(keypad[start.0][start.1]);
        }

        sol
    }

    pub fn solve_b(insts: &Vec<String>) -> String {
        let mut sol = String::new();
        let mut start = (3, 1);
        let keypad = vec![
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', '1', 'X', 'X', 'X'],
            vec!['X', 'X', '2', '3', '4', 'X', 'X'],
            vec!['X', '5', '6', '7', '8', '9', 'X'],
            vec!['X', 'X', 'A', 'B', 'C', 'X', 'X'],
            vec!['X', 'X', 'X', 'D', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X'],
        ];

        for inst in insts.iter() {
            start = find_key(&keypad, start, inst);
            sol.push(keypad[start.0][start.1]);
        }

        sol
    }


    #[cfg(test)]
    mod test_day02 {
        use super::*;

        #[test]
        fn test_find_key_a() {
            let keypad = vec![
                vec!['X', 'X', 'X', 'X', 'X'],
                vec!['X', '1', '2', '3', 'X'],
                vec!['X', '4', '5', '6', 'X'],
                vec!['X', '7', '8', '9', 'X'],
                vec!['X', 'X', 'X', 'X', 'X'],
            ];

            assert_eq!(find_key(&keypad, (2, 2), "ULL"), (1, 1));
            assert_eq!(find_key(&keypad, (1, 1), "RRDDD"), (3, 3));
            assert_eq!(find_key(&keypad, (3, 3), "LURDL"), (3, 2));
            assert_eq!(find_key(&keypad, (3, 2), "UUUUD"), (2, 2));
        }

        #[test]
        fn test_find_key_b() {
            let keypad = vec![
                vec!['X', 'X', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', '1', 'X', 'X', 'X'],
                vec!['X', 'X', '2', '3', '4', 'X', 'X'],
                vec!['X', '5', '6', '7', '8', '9', 'X'],
                vec!['X', 'X', 'A', 'B', 'C', 'X', 'X'],
                vec!['X', 'X', 'X', 'D', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'X', 'X'],
            ];

            assert_eq!(find_key(&keypad, (3, 1), "ULL"), (3, 1));
            assert_eq!(find_key(&keypad, (3, 1), "RRDDD"), (5, 3));
            assert_eq!(find_key(&keypad, (5, 3), "LURDL"), (4, 3));
            assert_eq!(find_key(&keypad, (4, 3), "UUUUD"), (2, 3));
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<_> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    println!("Solution A-part: {}", day02::solve_a(&lines));
    println!("Solution B-part: {}", day02::solve_b(&lines));
}
