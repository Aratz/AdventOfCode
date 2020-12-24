extern crate regex;

mod day24 {
    use regex::Regex;
    use std::collections::HashMap;

    pub fn solve_a(tiles: &Vec<String>) -> usize {
        let coord_map = vec![
            ("e",  ( 0,  1)),
            ("se", ( 1,  1)),
            ("sw", ( 1,  0)),
            ("w",  ( 0, -1)),
            ("nw", (-1, -1)),
            ("ne", (-1,  0)),
        ].into_iter().collect::<HashMap<&str, (i32, i32)>>();

        let re_dir = Regex::new(r"(e|se|sw|w|nw|ne)").unwrap();

        let fliped_tiles =tiles.iter().map(
            |tile| re_dir.captures_iter(&tile)
                    .map(|dir| coord_map[&dir.get(1).unwrap().as_str()])
                    .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
            ).fold(
                HashMap::new(),
                |mut acc, coord| {
                    acc.entry(coord).and_modify(|e| *e += 1).or_insert(1);
                    acc
                }
            );

        fliped_tiles.values().filter(|&v| v % 2 == 1).count()
    }

    #[cfg(test)]
    mod test_day24 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew\n";

            assert_eq!(solve_a(&input.lines().map(|s| s.into()).collect()), 10);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let input = stdin.lock().lines().map(|s| s.unwrap()).collect();

    println!("Solution A-part: {}", day24::solve_a(&input));
}
