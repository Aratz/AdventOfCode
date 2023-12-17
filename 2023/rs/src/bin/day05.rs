mod day05 {
    use std::cmp::{min, max};

    fn parse(input: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
        let sections = input.split("\n\n").collect::<Vec<_>>();

        let seeds = sections[0]
            .split(": ").skip(1).next().unwrap()
            .split(' ').map(|n| n.parse().unwrap())
            .collect();

        let conv_maps = sections[1..].iter()
            .map(
                |sec| {
                    let mut c_map: Vec<_> = sec.lines().skip(1)
                        .map(
                            |l|
                            l.split(' ').map(|n| n.parse().unwrap())
                                .collect::<Vec<i64>>())
                        .map(|dsl| (dsl[0], dsl[1], dsl[2]))
                        .collect();
                    c_map.sort_by_key(|dsl| dsl.1);
                    c_map
                })
            .collect();

        (seeds, conv_maps)
    }

    pub fn solve_a(input: &str) -> i64 {
        let (seeds, conv_maps) = parse(input);

        seeds.into_iter()
            .map(
                |s| {
                    conv_maps.iter().fold(s,
                        |s_acc, c_map|
                        match c_map.iter()
                            .find(
                                |dsl|
                                dsl.1 <= s_acc && s_acc < dsl.1 + dsl.2) {
                            Some(dsl) => { dsl.0 + (s_acc - dsl.1) },
                            None => { s_acc },
                        })
                })
            .min().unwrap()
    }

    pub fn solve_b(input: &str) -> i64 {
        let (seeds, conv_maps) = parse(input);

        let s_pairs = seeds.chunks(2)
            .map(|v_pair| (v_pair[0], v_pair[1]))
            .collect::<Vec<(i64, i64)>>();

        conv_maps.iter().fold(s_pairs,
            |pairs_acc, c_map|
            pairs_acc.into_iter()
                .flat_map(
                    |(mut s, mut range)| {
                        let mut new_pairs = Vec::new();
                        for dsl in c_map {
                            if s < dsl.1 {
                                new_pairs.push((s, min(range, dsl.1 - s)));
                                s = dsl.1;
                                range = max(0, range - dsl.1);
                                if range == 0 { break; }
                            }
                            if s < dsl.1 + dsl.2 {
                                new_pairs.push(
                                    (dsl.0 + (s - dsl.1), min(range, dsl.2)));
                                s = dsl.1 + dsl.2;
                                range = max(0, range - dsl.2);
                                if range == 0 { break; }
                            }
                        }
                        if range > 0 {
                            new_pairs.push((s, range));
                        }

                        new_pairs
                    }
                )
                .collect()
            ).into_iter()
                .map(|(seed, _range)| seed)
                .min().unwrap()
    }

    #[cfg(test)]
    mod test_day05 {
        use super::*;

        static INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 35);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 46);
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

    println!("Solution A-part: {}", day05::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day05::solve_b(&buffer.trim()));
}
