extern crate regex;

mod day24 {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};

    struct Floor(HashSet<(i32, i32)>);

    const NEIGHBORS: [(i32, i32); 7]= [
        ( 0,  0),
        ( 0,  1),
        ( 1,  1),
        ( 1,  0),
        ( 0, -1),
        (-1, -1),
        (-1,  0),
    ];

    impl Floor {
        fn n_tiles(&self) -> usize {
            self.0.len()
        }

        fn update(&mut self) {
            self.0 = self.0.iter()
                .flat_map(
                    |(x, y)| NEIGHBORS.iter().map(move |(dx, dy)| (x + dx, y + dy))
                    )
                .filter(|coords| {
                    let n = self.n_black_neighbors(coords);
                    let is_black = self.0.contains(coords);
                    if !is_black && n == 2 { true }
                    else if is_black && (n == 0 || n > 2) { false }
                    else { is_black }
                }).collect();
        }

        fn n_black_neighbors(&self, coords: &(i32, i32)) -> usize {
            let (x, y) = coords;
            NEIGHBORS.iter().filter(|(dx, dy)| dx != &0 || dy != &0)
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(|coords| self.0.contains(coords))
                .count()
        }
    }

    fn seed_tiles(tiles: &[String]) -> Floor {
        let coord_map = vec![
            ("e",  ( 0,  1)),
            ("se", ( 1,  1)),
            ("sw", ( 1,  0)),
            ("w",  ( 0, -1)),
            ("nw", (-1, -1)),
            ("ne", (-1,  0)),
        ].into_iter().collect::<HashMap<&str, (i32, i32)>>();

        let re_dir = Regex::new(r"(e|se|sw|w|nw|ne)").unwrap();

        Floor(tiles.iter().map(
            |tile| re_dir.captures_iter(&tile)
                    .map(|dir| coord_map[&dir.get(1).unwrap().as_str()])
                    .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
            ).fold(
                HashMap::<(i32, i32), bool>::new(),
                |mut acc, coord| {
                    acc.entry(coord).and_modify(|e| *e = !*e).or_insert(true);
                    acc
                }
            ).iter().filter(|&(_k, &v)| v).map(|(k, _v)| *k).collect())
    }

    pub fn solve_a(tiles: &[String]) -> usize {
        let flipped_tiles = seed_tiles(tiles);

        flipped_tiles.n_tiles()
    }

    pub fn solve_b(tiles: &[String], n_days: usize) -> usize {
        let mut flipped_tiles = seed_tiles(tiles);

        for _ in 0..n_days {
            flipped_tiles.update();
        }

        flipped_tiles.n_tiles()
    }

    #[cfg(test)]
    mod test_day24 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew\n";

            assert_eq!(solve_a(&input.lines().map(|s| s.into()).collect()), 10);
        }

        #[test]
        fn test_solve_b() {
            let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew\n";

            assert_eq!(solve_b(&input.lines().map(|s| s.into()).collect(), 1), 15);
            assert_eq!(solve_b(&input.lines().map(|s| s.into()).collect(), 2), 12);
            assert_eq!(solve_b(&input.lines().map(|s| s.into()).collect(), 100), 2208);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|s| s.unwrap()).collect();

    println!("Solution A-part: {}", day24::solve_a(&input));
    println!("Solution B-part: {}", day24::solve_b(&input, 100));
}
