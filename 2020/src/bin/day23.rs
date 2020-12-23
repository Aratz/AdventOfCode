mod day23 {
    use std::collections::{VecDeque, HashMap};

    fn iter_circle(mut cups: usize, n_moves: usize, n_cups: usize) -> HashMap<usize, usize> {
        let mut circle: HashMap<usize, usize> = HashMap::new();
        let mut staging = VecDeque::new();
        let base = 10;

        //populate queue
        let mut last_digit = 10;
        let first_digit = cups % base;
        while cups != 0 {
            circle.insert(cups % base, last_digit);
            last_digit = cups % base;
            cups /= base;
        }
        if n_cups == 9 {
            circle.insert(first_digit, last_digit);
        }
        else {
            circle.insert(n_cups, last_digit);
        }

        for i in 10..n_cups {
            circle.insert(i, i + 1);
        }


        let base = n_cups;
        let mut source = last_digit;
        //apply move
        for _ in 0..n_moves {
        //  - compute dest
            let mut dest = (source - 1 + base - 1) % base + 1;
        //  - pop and stack next three and adjust dest
            for _ in 0..3 {
                //source.next = last
                staging.push_back(circle[&source]);
                let source_next = circle[&circle[&source]];
                circle.insert(source, source_next);
            }
            while staging.iter().any(|&v| v == dest) {
                dest = (dest - 1 + base - 1) % base + 1;
            }
        //  - push back cups
            let mut dest_last = circle[&dest];
            let mut prev = dest;
            while let Some(v) = staging.pop_front() {
                circle.insert(prev, v);
                prev = v;
            }
            circle.insert(prev, dest_last);

        // Step forward
            source = circle[&source];
        }

        //let mut dbg_vec = Vec::new();
        //let mut source = 3;
        //for _ in 0..20 {
        //    source = circle[&source];
        //    dbg_vec.push(source);
        //}
        //dbg!(&dbg_vec);

        circle
    }

    pub fn solve_a(cups: usize, n_moves: usize) -> usize {
        let circle = iter_circle(cups, n_moves, 9);
        let mut res = Vec::new();
        let mut source = circle[&1];
        while source != 1 {
            res.push(source);
            source = circle[&source];
        }
        res.iter().rev().enumerate().map(|(i, v)| v*10usize.pow(i as u32)).sum::<usize>()
    }

    pub fn solve_b(cups: usize, n_moves: usize, n_cups: usize) -> usize {
        let circle = iter_circle(cups, n_moves, n_cups);
        circle[&1] * circle[&circle[&1]]
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

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(389125467, 10_000_000, 1_000_000), 149245887792);
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

    let start_cfg = buffer.trim().parse().unwrap();

    println!("Solution A-part: {}", day23::solve_a(start_cfg, 100));
    println!("Solution B-part: {}", day23::solve_b(start_cfg, 10_000_000, 1_000_000));

}
