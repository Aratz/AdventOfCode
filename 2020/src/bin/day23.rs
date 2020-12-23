mod day23 {
    use std::collections::VecDeque;

    pub fn solve_a(mut cups: usize, n: usize) -> usize {
        let mut circle = VecDeque::new();
        let mut staging = VecDeque::new();
        let base = 10;

        //populate queue
        while cups != 0 {
            circle.push_back(cups % base);
            cups /= base;
        }

        let v = circle.pop_back().unwrap();
        circle.push_front(v);

        let base = 9;
        //apply move
        for _ in 0..n {
        //  - compute dest
            let source = *circle.front().unwrap();
            let mut dest = (circle.front().unwrap() - 1 + base - 1) % base + 1;
        //  - pop and stack next three and adjust dest
            for _ in 0..3 {
                staging.push_back(circle.pop_back().unwrap());
            }
            while staging.iter().any(|&v| v == dest) {
                dest = (dest - 1 + base - 1) % base + 1;
            }
        //  - find dest
            while *circle.front().unwrap() != dest {
                let v = circle.pop_back().unwrap();
                circle.push_front(v);
            }
        //  - push back cups
            while let Some(v) = staging.pop_back() {
                circle.push_back(v);
            }
        //  - go back to source
            while *circle.front().unwrap() != source {
                let v = circle.pop_back().unwrap();
                circle.push_front(v);
            }

        // Step forward
            let v = circle.pop_back().unwrap();
            circle.push_front(v);

        }

        while *circle.front().unwrap() != 1 {
            let v = circle.pop_back().unwrap();
            circle.push_front(v);
        }

        circle.pop_front().unwrap();

        circle.iter().enumerate().map(|(i, v)| v*10usize.pow(i as u32)).sum::<usize>()
    }

    #[cfg(test)]
    mod test_day23 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(389125467, 1), 54673289);
            assert_eq!(solve_a(389125467, 10), 92658374);
            assert_eq!(solve_a(389125467, 100), 67384529);
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

    println!("Solution A-part: {}", day23::solve_a(buffer.trim().parse().unwrap(), 100));

}
