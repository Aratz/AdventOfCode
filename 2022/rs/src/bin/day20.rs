mod day20 {
    use std::collections::HashMap;

    fn parse(input: &str) -> (Vec<i64>, HashMap<usize, (usize, usize)>) {
        let numbers: Vec<i64> = input.lines()
            .map(|v| v.parse().unwrap())
            .collect();

        let mut cycle = numbers.iter().enumerate()
            .zip(numbers.iter().enumerate().skip(1).zip(numbers.iter().enumerate().skip(2)))
            .map(|((i, _), ((j, _), (k, _)))| (j, (i, k)))
            .collect::<HashMap<usize, (usize, usize)>>();

        cycle.insert(
            0,
            (numbers.len() - 1, 1));
        cycle.insert(
            numbers.len() - 1,
            (numbers.len() - 2, 0));

        (numbers, cycle)
    }
    pub fn solve_ab(input: &str, hard: bool) -> i64 {
        let (numbers, mut cycle) = parse(input);
        let numbers = if hard {
            numbers.into_iter().map(|v| v * 811589153).collect()
        }
        else {
            numbers
        };

        for _ in 0..(if hard { 10 } else { 1 }) {
            for (i, &v) in numbers.iter().enumerate() {
                let (mut i_prev, mut i_next) = cycle[&i];
                cycle.entry(i_prev).and_modify(|val| val.1 = i_next);
                cycle.entry(i_next).and_modify(|val| val.0 = i_prev);
                cycle.remove(&i);

                if v > 0 {
                    for _ in 0..(v % (numbers.len() - 1) as i64) {
                        i_prev = i_next;
                        i_next = cycle[&i_next].1;
                    }
                }
                else if v < 0 {
                    for _ in 0..-(v % (numbers.len() - 1) as i64) {
                        i_next = i_prev;
                        i_prev = cycle[&i_prev].0;
                    }
                }

                cycle.entry(i_prev).and_modify(|val| val.1 = i);
                cycle.entry(i_next).and_modify(|val| val.0 = i);
                cycle.insert(i, (i_prev, i_next));

            }
        }

        let mut i = numbers.iter().enumerate().find(|(_, v)| **v == 0).unwrap().0;
        let mut sum = 0;
        for n in 0..3000 + 1 {
            if n != 0 && n % 1000 == 0 {
                sum += numbers[i];
            }
            i = cycle[&i].1;
        }

        sum
    }

    #[cfg(test)]
    mod test_day20 {
        use super::*;

        static INPUT: &str = "1
2
-3
3
-2
0
4";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_ab(INPUT, false), 3);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_ab(INPUT, true), 1623178306);
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

    println!("Solution A-part: {}", day20::solve_ab(&buffer.trim(), false));
    println!("Solution B-part: {}", day20::solve_ab(&buffer.trim(), true));
}
