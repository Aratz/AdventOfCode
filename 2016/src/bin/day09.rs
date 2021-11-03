extern crate regex;

mod day09 {
    use regex::Regex;

    fn decompress(stream: &str) -> String {
        let mut dec_s = String::new();

        let reg_marker = Regex::new(r"\((?P<len>\d+)x(?P<times>\d+)\)").unwrap();

        let mut last_i = 0;

        for c in reg_marker.captures_iter(stream) {
            let marker = c.get(0).unwrap();
            let len: usize = c["len"].parse().unwrap();
            let times: usize = c["times"].parse().unwrap();

            if marker.start() < last_i {
                continue;
            }

            dec_s.push_str(&stream[last_i..marker.start()]);

            let substring = &stream[marker.end()..marker.end()+len];
            for _ in 0..times {
                dec_s.push_str(substring);
            }

            last_i = marker.end()+len
        }
        dec_s.push_str(&stream[last_i..]);

        dec_s
    }

    pub fn solve_a(stream: &str) -> usize {
        decompress(stream).len()
    }

    #[cfg(test)]
    mod test_day09 {
        use super::*;

        #[test]
        fn test_decompress() {
            assert_eq!(decompress("ADVENT"), "ADVENT");
            assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
            assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
            assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
            assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
            assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
            assert_eq!(decompress("X(10x2)(3x3)ABCYZ"), "X(3x3)ABCYZ(3x3)ABCYZ");
        }
    }

}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: String = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect();

    println!("Solution A-part: {}", day09::solve_a(&lines));
}
