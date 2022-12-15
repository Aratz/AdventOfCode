extern crate regex;

mod day15 {
    use regex::Regex;
    use std::collections::HashSet;

    fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
        let re_sensor = Regex::new(r"Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>-?\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>-?\d+)").unwrap();
        re_sensor.captures_iter(input)
            .map(|capt| (
                    (capt["s_x"].parse().unwrap(), capt["s_y"].parse().unwrap()),
                    (capt["b_x"].parse().unwrap(), capt["b_y"].parse().unwrap())))
            .collect()
    }

    #[inline]
    fn dist(a: &(i64, i64), b: &(i64, i64)) -> u64 {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }

    pub fn solve_a(input: &str, y: i64) -> usize {
        let sensors = parse(input);
        let min_x = sensors.iter().map(|(s, b)| s.0 - dist(s, b) as i64).min().unwrap();
        let max_x = sensors.iter().map(|(s, b)| s.0 + dist(s, b) as i64).max().unwrap();

        let beacons: HashSet<_> = sensors.iter().map(|(_s, b)| b).collect();

        (min_x..=max_x)
            .filter(|&x| !beacons.contains(&(x, y)))
            .filter(|&x| sensors.iter().any(|(s, b)| dist(s, &(x, y)) <= dist(s, b)))
            .count()
    }

    pub fn solve_b(input: &str, (max_x, max_y): (i64, i64)) -> i64 {
        let sensors = parse(input);

        let target = sensors.iter()
            .flat_map(|(s, b)| {
                let range = dist(s, b) as i64 + 1;
                (0..=range).flat_map(move |dx| {
                    let dy = range - dx;
                    vec![(dx, dy), (-dx, dy), (dx, -dy), (-dx, -dy)]
                        .into_iter()
                        .map(|(dx, dy)| (s.0 + dx, s.1 + dy))
                })
            })
            .filter(|&(x, y)| 0 <= x && x <= max_x && 0 <= y && y <= max_y)
            .find(|pos| sensors.iter().all(|(s, b)| dist(s, pos) > dist(s, b)))
            .unwrap();

            target.0 * 4000000 + target.1
    }

    #[cfg(test)]
    mod test_day15 {
        use super::*;

        static INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(&INPUT, 10), 26);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(&INPUT, (20, 20)), 56000011);
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

    println!("Solution A-part: {}", day15::solve_a(&buffer.trim(), 2000000));
    println!("Solution B-part: {}", day15::solve_b(&buffer.trim(), (4000000, 4000000)));
}
