mod day20 {
    fn are_overlapping(int1: &(u64, u64), int2: &(u64, u64)) -> bool {
        int2.0 + 1 <= int1.0 && int1.0 <= int2.1 + 1
            || int1.0 + 1 <= int2.0 && int2.0 <= int1.1 + 1
            || int2.0 <= int1.1 + 1 && int1.1 + 1 <= int2.1
            || int1.0 <= int2.1 + 1 && int2.1 + 1 <= int1.1
    }

    /// Merge _overlapping_ intervals
    fn merge_overlap(ints: &[(u64, u64)]) -> (u64, u64) {
        (
            ints.iter().map(|int| int.0).min().unwrap(),
            ints.iter().map(|int| int.1).max().unwrap()
        )
    }

    fn merge(ints: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let mut merged_ints = Vec::new();
        for new_int in ints {
            let (mut overlap, mut non_overlap): (Vec<(u64, u64)>, Vec<(u64, u64)>) = merged_ints.iter()
                .partition(|int| are_overlapping(int, new_int));
            overlap.push(*new_int);
            non_overlap.push(merge_overlap(&overlap));
            merged_ints = non_overlap;
        }

        merged_ints.sort();

        merged_ints
    }

    pub fn solve_a(ints: &[(u64, u64)]) -> u64 {
        let merged_ints = merge(ints);
        merged_ints.iter().map(|int| int.1).min().unwrap() + 1
    }

    pub fn solve_b(ints: &[(u64, u64)]) -> u64 {
        let merged_ints = merge(ints);
        merged_ints.iter()
            .zip(merged_ints.iter().skip(1))
            .map(|(int1, int2)| int2.0 - int1.1 - 1)
            .sum()
    }


    #[cfg(test)]
    mod test_day20 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(&vec![(5, 8), (0, 2), (4, 7)]), 3);
            assert_eq!(solve_a(&vec![(5, 8), (0, 3), (4, 7)]), 9);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(&vec![(5, 8), (0, 2), (4, 7), (10, 20)]), 2);
            assert_eq!(solve_b(&vec![(5, 8), (0, 3), (4, 7), (10, 20)]), 1);
        }
    }
}


fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let intervals: Vec<(u64, u64)> = stdin.lock().lines()
        .map(|l| {
            let bounds = l.unwrap().split("-")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>();
            (bounds[0], bounds[1])
        })
        .collect();

    println!("Solution A-part: {}", day20::solve_a(&intervals));
    println!("Solution B-part: {}", day20::solve_b(&intervals));
}
