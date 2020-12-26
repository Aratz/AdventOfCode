mod day14 {
    const ROUNDS: usize = 64;
    const LENGTH: usize = 256;
    const SUFFIX: [usize; 5] = [17, 31, 73, 47, 23];

    fn knot_hash(s: &str) -> Vec<usize> {
        let mut string: Vec<usize> = (0..256).collect();
        let mut input_lengths: Vec<usize> = s.chars().map(|x| (x as u8) as usize).collect();
        input_lengths.extend(SUFFIX.iter());

        let mut skip_size = 0;
        let mut shift = 0;

        for _ in 0 .. ROUNDS {
            for &in_len in &input_lengths {
                string = string.iter().cycle().take(in_len).copied().collect::<Vec<_>>()
                    .into_iter().rev().collect::<Vec<_>>()
                    .into_iter().chain(
                        string.iter().cycle().skip(in_len).take(LENGTH - in_len).copied())
                    .collect();

                string = string.into_iter().cycle().skip(in_len + skip_size)
                    .take(LENGTH).collect();

                shift += in_len + skip_size;
                skip_size += 1;
            }
        }

        string = string.into_iter().cycle().skip(LENGTH - (shift % LENGTH))
            .take(LENGTH).collect();

        (0..16).map(|i| string[16*i..16*(i+1)].iter().fold(0, |acc, &x| acc ^ x))
            .collect()
    }

    pub fn solve_a(s: &str) -> usize {
        (0..128).map(|row| {
            let hash = knot_hash(&format!("{}-{}", s, row));
            hash.into_iter().map(
                |n| (0..8).map(|bit| if n & (1<<bit) != 0 { 1 } else { 0}).sum::<usize>())
                .sum::<usize>()
        }).sum()
    }

    #[cfg(test)]
    mod test_day14 {
        use super::*;

        #[test]
        fn test_knot() {
            assert_eq!(knot_hash(""),
                vec![
                    0xa2, 0x58, 0x2a, 0x3a,
                    0x0e, 0x66, 0xe6, 0xe8,
                    0x6e, 0x38, 0x12, 0xdc,
                    0xb6, 0x72, 0xa2, 0x72,
                ]);
            assert_eq!(knot_hash("AoC 2017"),
                vec![
                    0x33, 0xef, 0xeb, 0x34,
                    0xea, 0x91, 0x90, 0x2b,
                    0xb2, 0xf5, 0x9c, 0x99,
                    0x20, 0xca, 0xa6, 0xcd,
                ]);
        }

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a("flqrgnkx"), 8108);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let key_string = stdin.lock().lines().next().unwrap().unwrap();

    println!("Solution A-part: {}", day14::solve_a(&key_string));
}
