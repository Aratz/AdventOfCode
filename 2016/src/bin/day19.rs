mod day19 {
    pub fn solve_a(n_elf: usize) -> usize {
        let mut next_elves: Vec<Option<usize>> = (0..n_elf).map(|elf| Some((elf + 1)%n_elf)).collect();

        let mut current_elf = 0;
        while Some(current_elf) != next_elves[current_elf] {
            let next_elf = next_elves[current_elf].unwrap();
            let next_elf2 = next_elves[next_elf].unwrap();

            next_elves[current_elf] = Some(next_elf2);
            next_elves[next_elf] = None;

            current_elf = next_elf2;
        }

        current_elf + 1
    }

    pub fn solve_b(mut n_elf: usize) -> usize {
        let mut next_elves: Vec<Option<usize>> = (0..n_elf).map(|elf| Some((elf + 1)%n_elf)).collect();

        let mut current_elf = n_elf/2 - 1;

        if n_elf % 2 == 1 {
            let next_elf = next_elves[current_elf].unwrap();
            let next_elf2 = next_elves[next_elf].unwrap();

            next_elves[current_elf] = Some(next_elf2);
            next_elves[next_elf] = None;

            current_elf = next_elf2;
            n_elf -= 1;
        }

        while Some(current_elf) != next_elves[current_elf] && n_elf > 2 {
            let next_elf = next_elves[current_elf].unwrap();
            let next_elf2 = next_elves[next_elf].unwrap();
            let next_elf3 = next_elves[next_elf2].unwrap();

            next_elves[current_elf] = Some(next_elf3);
            next_elves[next_elf] = None;
            next_elves[next_elf2] = None;

            current_elf = next_elf3;
            n_elf -= 2;
        }

        current_elf + 1
    }


    #[cfg(test)]
    mod test_day19 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(5), 3);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(5), 2);
            assert_eq!(solve_b(6), 3);
            assert_eq!(solve_b(11), 2);
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

    println!("Solution A-part: {}", day19::solve_a(buffer.trim().parse().unwrap()));
    println!("Solution B-part: {}", day19::solve_b(buffer.trim().parse().unwrap()));
}
