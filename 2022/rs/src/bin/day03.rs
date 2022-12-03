mod day03 {
    use std::collections::HashSet;

    fn find_common<'a>(s1: &'a [i32], s2: &'a [i32]) -> Vec<i32> {
        let s1_set: HashSet<i32> = HashSet::from_iter(s1.iter().cloned());
        s2.iter()
            .filter(|n| s1_set.contains(n))
            .map(|n| *n)
            .collect()
    }

    pub fn solve_a(input: &str) -> i32 {
        input.lines()
            .map(|l| {
                let n_item = l.len();
                let items = l.chars().map(|c| match c {
                    'a'..='z' => { c as i32 - 'a' as i32 + 1 },
                    'A'..='Z' => { c as i32 - 'A' as i32 + 27 },
                    _ => unreachable!(),
                }).collect::<Vec<_>>();
                find_common(&items[..n_item/2], &items[n_item/2..])[0]
            })
            .sum()
    }

    pub fn solve_b(input: &str) -> i32 {
        input.lines()
            .map(|l| l.chars().map(|c| match c {
                'a'..='z' => { c as i32 - 'a' as i32 + 1 },
                'A'..='Z' => { c as i32 - 'A' as i32 + 27 },
                _ => unreachable!(),
            }).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|elfes| find_common(&find_common(&elfes[0], &elfes[1]), &elfes[2])[0])
            .sum()
    }

    #[cfg(test)]
    mod test_day03 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

            assert_eq!(solve_a(&input), 157);
        }

        #[test]
        fn test_solve_b() {
            let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

            assert_eq!(solve_b(&input), 70);
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

    println!("Solution A-part: {}", day03::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day03::solve_b(&buffer.trim()));
}
