extern crate regex;

mod day22 {
    use std::collections::HashSet;
    use std::cmp::{min, max};
    use regex::Regex;

    fn parse(input: &str) -> Vec<(((i32, i32), (i32, i32), (i32, i32)), bool)> {
        let re_parse = Regex::new(r"(?P<onoff>\w+) x=(?P<xmin>-?\d+)..(?P<xmax>-?\d+),y=(?P<ymin>-?\d+)..(?P<ymax>-?\d+),z=(?P<zmin>-?\d+)..(?P<zmax>-?\d+)").unwrap();

    re_parse.captures_iter(&input)
        .map(|capt| ((
                    (capt["xmin"].parse().unwrap(), capt["xmax"].parse().unwrap()),
                    (capt["ymin"].parse().unwrap(), capt["ymax"].parse().unwrap()),
                    (capt["zmin"].parse().unwrap(), capt["zmax"].parse().unwrap()),
                ),
                &capt["onoff"] == "on"))
        .collect()
    }

    pub fn solve_a(input: &str) -> usize {
        let steps = parse(input);

        let mut cubes = HashSet::new();

        for (((xmin, xmax), (ymin, ymax), (zmin, zmax)), onoff) in steps {
            for x in max(-50, xmin)..=min(50, xmax) {
                for y in max(-50, ymin)..=min(50, ymax) {
                    for z in max(-50, zmin)..=min(50, zmax) {
                        if onoff {
                            cubes.insert((x, y, z));
                        }
                        else {
                            cubes.remove(&(x, y, z));
                        }
                    }
                }
            }
        }

        cubes.len()
    }

    //new solve
    //keep track of non overlapping full on cubes
    //cases:
    //new on:
    //  no intersect -> 2 cubes
    //  full intersect -> 1 cube
    //  corner intersect -> 7 + 1 cube
    //  edge intersect
    //      -> include -> 4 + 1 cube
    //      -> exclude -> 2 + 1 cube
    //  face intersect ->
    //      -> include -> 1 + 1 cube
    //      -> exclude -> 1 + 1 cube
    //
    //new off:
    //  no intersect -> 1 cube
    //  full intersect -> 0 cubes
    //  corner intersect -> 7 cubes
    //  edge intersect
    //      -> include -> 4 cubes
    //      -> exclude -> 2 cubes
    //  face intersect ->
    //      -> include -> 9 cubes
    //      -> exclude -> 1 cube

    #[cfg(test)]
    mod test_day21 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

            assert_eq!(solve_a(&input), 590784);
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

    println!("Solution A-part: {}", day22::solve_a(&buffer));
}
