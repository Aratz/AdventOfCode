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

    println!("Solution A-part: {}", day17::solve_a(step))

}
