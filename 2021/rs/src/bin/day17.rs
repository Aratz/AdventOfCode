extern crate regex;

mod day17 {
    use regex::Regex;

    #[inline]
    fn hits_target_y(y0: i32, v0: i32, (y_min, y_max): (i32, i32)) -> bool {
        (0..).map(|n| n*v0 - n*(n-1)/2 + y0)
            .take_while(|&y| y >= y_min)
            .any(|y| y < y_max)
    }

    #[inline]
    fn cumsum(x : i32) -> i32 {
        if x > 0 {
            x*(x+1)/2
        }
        else {
            0
        }
    }

    #[inline]
    fn hits_target(
        (x0, y0): (i32, i32),
        (vx, vy): (i32, i32),
        range: ((i32, i32), (i32, i32))) -> bool {

        let ((x_min, x_max), (y_min, y_max)) = range;
        (0..).map(|n| (x0 + cumsum(vx) - cumsum(vx - n) , n*vy - n*(n-1)/2 + y0))
            .take_while(|&(x, y)| x <= x_max && y >= y_min)
            .any(|(x, y)| y <= y_max && x_min <= x)
    }

    #[inline]
    fn max_height(y0: i32, v0: i32, y_min: i32) -> i32 {
        (0..).map(|n| n*v0 - n*(n-1)/2 + y0)
            .take_while(|&y| y >= y_min)
            .max().unwrap()
    }


    fn find_y(y_range: (i32, i32)) -> i32 {
        let y0 = 0;
        let v_min = 0;
        let v_max = 1000;

        (v_min..v_max).filter(|&v| hits_target_y(y0, v, y_range)).max().unwrap()
    }

    fn find_pairs(x_range: (i32, i32), y_range: (i32, i32)) -> usize {
        let (x0, y0) = (0, 0);
        let (vx_min, vx_max) = (0, x_range.1);
        let (vy_min, vy_max) = (y_range.0, find_y(y_range));

        //dbg!((vx_min..vx_max).flat_map(|vx| (vy_min..vy_max).map(move |vy| (vx, vy)))
        //    .filter(|&(vx, vy)| hits_target((x0, y0), (vx, vy), (x_range, y_range)))
        //    .collect::<Vec<_>>());
        (vx_min..=vx_max).flat_map(|vx| (vy_min..=vy_max).map(move |vy| (vx, vy)))
            .filter(|&(vx, vy)| hits_target((x0, y0), (vx, vy), (x_range, y_range)))
            .count()
    }

    fn parse(input: &str) -> ((i32, i32), (i32, i32)) {
        let re_in = Regex::new(r"target area: x=(?P<x_min>-?\d+)\.\.(?P<x_max>-?\d+), y=(?P<y_min>-?\d+)\.\.(?P<y_max>-?\d+)").unwrap();
        let caps = re_in.captures(input).unwrap();

        (
            (caps["x_min"].parse().unwrap(), caps["x_max"].parse().unwrap()),
            (caps["y_min"].parse().unwrap(), caps["y_max"].parse().unwrap()),
        )
    }

    pub fn solve_a(input: &str) -> i32 {
        let (_x_range, y_range) = parse(input);
        let y0 = 0;

        let v = find_y(y_range);

        assert!(hits_target_y(0, v, y_range));

        max_height(y0, v, y_range.0)
    }

    pub fn solve_b(input: &str) -> usize {
        let (x_range, y_range) = parse(input);

        find_pairs(x_range, y_range)
    }

    #[cfg(test)]
    mod test_day17 {
        use super::*;

        #[test]
        fn test_hits_target_y() {
            assert!(hits_target_y(0, 9, (-10, -5)));
        }

        #[test]
        fn test_hits_target() {
            assert!(hits_target((0, 0), (6, 9), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (6, 0), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (23, -10), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (25, -7), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (8, 0), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (26, -10), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (20, -8), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (25, -6), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (25, -10), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (8, 1), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (24, -10), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (7, 5), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (23, -5), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (27, -10), ((20, 30), (-10, -5))));
            assert!(hits_target((0, 0), (8, -2), ((20, 30), (-10, -5))));
        }

        #[test]
        fn test_max_height() {
            assert_eq!(max_height(0, 9, -10), 45)
        }

        #[test]
        fn test_solve_a() {
            let input = "target area: x=20..30, y=-10..-5";

            assert_eq!(solve_a(&input), 45);
        }

        #[test]
        fn test_solve_b() {
            let input = "target area: x=20..30, y=-10..-5";

            assert_eq!(solve_b(&input), 112);
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

    println!("Solution A-part: {}", day17::solve_a(&buffer));
    println!("Solution B-part: {}", day17::solve_b(&buffer));
}
