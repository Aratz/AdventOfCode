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

    fn lazy_decompress(stream: &str) -> usize {
        let mut dec_len = 0;

        let reg_marker = Regex::new(r"\((?P<len>\d+)x(?P<times>\d+)\)").unwrap();

        let mut last_i = 0;

        for c in reg_marker.captures_iter(stream) {
            let marker = c.get(0).unwrap();
            let len: usize = c["len"].parse().unwrap();
            let times: usize = c["times"].parse().unwrap();

            if marker.start() < last_i {
                continue;
            }

            dec_len += marker.start() - last_i;

            let substring = &stream[marker.end()..marker.end()+len];
            dec_len += lazy_decompress(substring) * times;

            last_i = marker.end()+len
        }
        dec_len += stream.len() - last_i;

        dec_len
    }

    pub fn solve_a(stream: &str) -> usize {
        decompress(stream).len()
    }

    pub fn solve_b(stream: &str) -> usize {
        lazy_decompress(stream)
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

        #[test]
        fn test_lazy_decompress() {
            assert_eq!(lazy_decompress("ADVENT"), 6);
            assert_eq!(lazy_decompress("A(1x5)BC"), 7);
            assert_eq!(lazy_decompress("(3x3)XYZ"), 9);
            assert_eq!(lazy_decompress("X(8x2)(3x3)ABCY"), 20);
            assert_eq!(lazy_decompress("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
            assert_eq!(lazy_decompress("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
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
    println!("Solution B-part: {}", day09::solve_b(&lines));
}
