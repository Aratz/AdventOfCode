mod day17 {
    pub fn solve_a(step: usize) -> usize{
        let n_ins = 2017;
        let mut spinlock = vec![0; n_ins + 1];
        let mut pos = 0;

        for n in 1..=n_ins {
            for _ in 0..step {
                pos = spinlock[pos];
            }

            spinlock[n] = spinlock[pos];
            spinlock[pos] = n;

            pos = n;
        }

        spinlock[2017]
    }

    pub fn solve_b(step: usize) -> usize {
        let n_ins = 50_000_000;
        let mut next = 0;

        let mut pos = 0;
        let mut spin_size = 1;

        for n in 1..=n_ins {
            pos = (pos + step) % spin_size;

            if pos == 0 {
                next = n;
            }

            spin_size += 1;
            pos += 1;
        }

        next
    }

    #[cfg(test)]
    mod test_day17 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(3), 638);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let step = stdin.lock().lines().next().unwrap().unwrap()
        .parse().unwrap();

    println!("Solution A-part: {}", day17::solve_a(step));
    println!("Solution B-part: {}", day17::solve_b(step));
}
