mod day03 {
    fn get_most_common_bit(report: &[Vec<i32>], pos: usize) -> i32 {
        if 2 * report.iter().filter(|s| s[pos] == 0).count() > report.len()
        { 0 } else { 1 }
    }

    fn get_least_common_bit(report: &[Vec<i32>], pos: usize) -> i32 {
        if 2 * report.iter().filter(|s| s[pos] == 1).count() >= report.len()
        { 0 } else { 1 }
    }

    fn to_decimal(digits: &[i32]) -> i32 {
        digits.iter()
            .enumerate()
            .map(|(i, d)| d * (1 << (digits.len() - 1 - i)))
            .sum()
    }

    fn compute_gamma(report: &[Vec<i32>]) -> i32 {
        (0..report[0].len())
            .map(|i| get_most_common_bit(report, i))
            .enumerate()
            .map(|(i, n)| n * (1 << (report[0].len() - 1 - i)))
            .sum()
    }

    fn compute_epsilon(report: &[Vec<i32>]) -> i32 {
        (1 << (report[0].len() as i32 )) - compute_gamma(report) - 1
    }

    fn compute_o2(report: &[Vec<i32>], i_filter: usize) -> i32 {
        if report.len() == 1 {
            to_decimal(&report[0])
        }
        else {
            let most_common_bit = get_most_common_bit(report, i_filter);
            let filtered_report = report.iter()
                .filter(|s| s[i_filter] == most_common_bit)
                .map(|s| s.to_vec())
                .collect::<Vec<Vec<i32>>>();
            compute_o2(&filtered_report, i_filter + 1)
        }
    }

    fn compute_co2(report: &[Vec<i32>], i_filter: usize) -> i32 {
        if report.len() == 1 {
            to_decimal(&report[0])
        }
        else {
            let least_common_bit = get_least_common_bit(report, i_filter);
            let filtered_report = report.iter()
                .filter(|s| s[i_filter] == least_common_bit)
                .map(|s| s.to_vec())
                .collect::<Vec<Vec<i32>>>();
            compute_co2(&filtered_report, i_filter + 1)
        }
    }

    pub fn solve_a(report: &[Vec<i32>]) -> i32 {
        compute_gamma(report) * compute_epsilon(report)
    }

    pub fn solve_b(report: &[Vec<i32>]) -> i32 {
        compute_o2(report, 0) * compute_co2(report, 0)
    }

    #[cfg(test)]
    mod test_day03 {
        use super::*;

        #[test]
        fn test_gamma() {
            let test_report: Vec<Vec<i32>> = ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
                .iter().map(|s| s.chars()
                         .map(|c| c.to_digit(10).unwrap() as i32)
                         .collect())
                .collect();

            assert_eq!(compute_gamma(&test_report), 22);
        }

        #[test]
        fn test_epsilon() {
            let test_report: Vec<Vec<i32>> = ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
                .iter().map(|s| s.chars()
                         .map(|c| c.to_digit(10).unwrap() as i32)
                         .collect())
                .collect();

            assert_eq!(compute_epsilon(&test_report), 9);
        }

        #[test]
        fn test_o2() {
            let test_report: Vec<Vec<i32>> = ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
                .iter().map(|s| s.chars()
                         .map(|c| c.to_digit(10).unwrap() as i32)
                         .collect())
                .collect();

            assert_eq!(compute_o2(&test_report, 0), 23);
        }

        #[test]
        fn test_co2() {
            let test_report: Vec<Vec<i32>> = ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
                .iter().map(|s| s.chars()
                         .map(|c| c.to_digit(10).unwrap() as i32)
                         .collect())
                .collect();

            assert_eq!(compute_co2(&test_report, 0), 10);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let report: Vec<Vec<i32>> = stdin.lock().lines()
        .map(|l| l.unwrap().chars()
             .map(|c| c.to_digit(10).unwrap() as i32)
             .collect())
        .collect();

    println!("Solution A-part: {}", day03::solve_a(&report));
    println!("Solution B-part: {}", day03::solve_b(&report));
}
