mod day25 {
    use std::cmp::max;

    pub fn solve_a(input: &str) -> String {
        let mut sum = input.lines()
            .map(|s| s.chars()
                 .rev()
                 .map(|c| match c {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => unreachable!(),
                }).collect::<Vec<i32>>())
            .fold(vec![], |acc, x| {
                let mut res = vec![0; max(acc.len(), x.len())];
                for i in 0..acc.len() {
                    res[i] += acc[i];
                }
                for i in 0..x.len() {
                    res[i] += x[i];
                }

                res
            });

        let mut max_digit = sum.len();
        let mut i = 0;

        while i < max_digit {
            if sum[i] > 2 {
                if i + 1 == max_digit {
                    sum.push(0);
                    max_digit += 1;
                }

                let x = (sum[i] + 2) / 5;
                sum[i] -= 5 * x;
                sum[i + 1] += x;
            }
            if sum[i] < -2 {
                if i + 1 == max_digit {
                    sum.push(0);
                    max_digit += 1;
                }

                let x = (sum[i] - 2) / -5;
                sum[i] += 5 * x;
                sum[i + 1] -= x;
            }

            i += 1;
        }

        sum.into_iter().rev()
            .map(|digit| match digit {
                -2 => '=',
                -1 => '-',
                d => char::from_digit(d as u32, 10).unwrap(),
            }).collect()
    }

    #[cfg(test)]
    mod test_day25 {
        use super::*;

        static INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), String::from("2=-1=0"));
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

    println!("Solution A-part: {}", day25::solve_a(&buffer.trim()));
}
