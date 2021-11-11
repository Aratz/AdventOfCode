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

    #[cfg(test)]
    mod test_day19 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(5), 3);
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
}
