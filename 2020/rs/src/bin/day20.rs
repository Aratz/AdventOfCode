extern crate regex;


mod day20 {
    use std::mem;
    use std::collections::{HashMap, VecDeque};

    const SIDE_LENGTH: usize = 10;

    #[derive(Clone, Debug)]
    enum Side {
        N(String),
        W(String),
        S(String),
        E(String),
    }

    impl Side {
        fn align_rotation(&self, new_side: &Side) -> u64 {
            use self::Side::{N, W, S, E};

            match (self, new_side) {
                (s1, s2) if std::mem::discriminant(s1) == std::mem::discriminant(s2) => 180,
                (N(_), E(_)) | (W(_), N(_)) | (S(_), W(_)) | (E(_), S(_)) => 270,
                (N(_), S(_)) | (S(_), N(_)) | (E(_), W(_)) | (W(_), E(_)) => 0,
                (E(_), N(_)) | (N(_), W(_)) | (W(_), S(_)) | (S(_), E(_)) => 90,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Clone, Debug)]
    struct NWSE {
        n: String,
        w: String,
        s: String,
        e: String,
    }

    impl NWSE {
        fn new(n: String, w: String, s: String, e: String) -> Self {
            Self { n, w, s, e }
        }
    }

    impl IntoIterator for NWSE {
        type Item = (Side, String);
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            vec![
                (Side::N(self.n.clone()), self.n),
                (Side::W(self.w.clone()), self.w),
                (Side::S(self.s.clone()), self.s),
                (Side::E(self.e.clone()), self.e),
            ].into_iter()
        }
    }

    #[derive(Clone, Debug)]
    pub struct Tile {
        id: u64,
        content: Vec<Vec<char>>,
        borders: NWSE,
    }

    impl Tile {
        pub fn new(id: u64, content: Vec<Vec<char>>) -> Self {
            let borders = get_borders(&content);
            Self { id, content, borders }
        }

        fn flip_h(&mut self) {
            mem::swap(&mut self.borders.e, &mut self.borders.w);
            self.borders.n = self.borders.n.chars().rev().collect();
            self.borders.w = self.borders.w.chars().rev().collect();
            self.borders.s = self.borders.s.chars().rev().collect();
            self.borders.e = self.borders.e.chars().rev().collect();
            for row in self.content.iter_mut() {
                row.reverse();
            }
        }

        fn flip_v(&mut self) {
            mem::swap(&mut self.borders.n, &mut self.borders.s);
            self.borders.n = self.borders.n.chars().rev().collect();
            self.borders.w = self.borders.w.chars().rev().collect();
            self.borders.s = self.borders.s.chars().rev().collect();
            self.borders.e = self.borders.e.chars().rev().collect();
            self.content.reverse();
        }

        fn rotate90(&mut self) {
            let n = mem::take(&mut self.borders.n);
            let w = mem::replace(&mut self.borders.w, n);
            let s = mem::replace(&mut self.borders.s, w);
            let e = mem::replace(&mut self.borders.e, s);
            self.borders.n = e;

            let mut rotated = self.content.clone();
            for y in 0..SIDE_LENGTH {
                for x in 0..SIDE_LENGTH {
                    rotated[SIDE_LENGTH - x -1][y] = self.content[y][x];
                }
            }

            self.content = rotated
        }

        fn rotate180(&mut self) {
            self.flip_h();
            self.flip_v();
        }

        fn rotate270(&mut self) {
            self.rotate90();
            self.rotate90();
            self.rotate90();
        }
    }

    fn get_borders(content: &[Vec<char>]) -> NWSE {
        let n = content[0].iter().collect();
        let w = content.iter().rev().map(|row| row[0]).collect();
        let s = content[SIDE_LENGTH-1].iter().rev().collect();
        let e = content.iter().map(|row| row[SIDE_LENGTH-1]).collect();

        NWSE::new(n, w, s, e)
    }

    struct AssemblyLine {
        tiles: HashMap<u64, Tile>,
        border_pool: HashMap<String, (Side, u64)>,
        assembled: HashMap<u64, (i32, i32)>,
        queue: VecDeque<u64>,
        done: bool,
    }

    impl AssemblyLine {
        fn new(tiles: &[Tile]) -> Self {
            let tiles: HashMap<u64, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();
            let queue = VecDeque::from(tiles.keys().copied().collect::<Vec<u64>>());
            Self {
                tiles,
                border_pool: HashMap::new(),
                assembled: HashMap::new(),
                queue,
                done: false,
            }
        }

        fn find_match(&mut self, new_tile_id: u64) -> bool {
            let new_tile = self.tiles.get_mut(&new_tile_id).unwrap();
            for (new_side, border) in new_tile.borders.clone().into_iter() {
                if let Some((fixed_side, id)) = self.border_pool
                            .get(&border.chars().rev().collect::<String>()) {
                            //Set rotation, set coordinates, add to pool, then continue
                        // Set rotation
                        match fixed_side.align_rotation(&new_side) {
                            0 => {},
                            90 => { new_tile.rotate90(); },
                            180 => { new_tile.rotate180(); },
                            270 => { new_tile.rotate270(); },
                            _ => unreachable!(),
                        }

                        // Set coordinates
                        // if rot 90 or 270: no flip ?
                        // Not necessary to flip if rotation has been done?
                        let (fixed_x, fixed_y) = self.assembled[&id];
                        match fixed_side {
                            Side::N(_) => {
                                self.assembled.insert(new_tile.id, (fixed_x, fixed_y + 1));
                            },
                            Side::W(_) => {
                                self.assembled.insert(new_tile.id, (fixed_x - 1, fixed_y));
                            },
                            Side::S(_) => {
                                self.assembled.insert(new_tile.id, (fixed_x, fixed_y - 1));
                            },
                            Side::E(_) => {
                                self.assembled.insert(new_tile.id, (fixed_x + 1, fixed_y));
                            },
                        };

                        // add to pool
                        for (s, border) in new_tile.borders.clone().into_iter() {
                            self.border_pool.insert(border, (s, new_tile.id));
                        }

                        return true
                }
            }

            false
        }

        fn assemble(&mut self) {
            if self.done {
                return
            }

            let first_tile = &self.tiles[&self.queue.pop_front().unwrap()];
            self.assembled.insert(first_tile.id, (0, 0));
            for (s, border) in first_tile.borders.clone().into_iter() {
                self.border_pool.insert(border, (s, first_tile.id));
            }

            while let Some(new_tile_id) = self.queue.pop_front() {
                if !self.find_match(new_tile_id) {
                    self.tiles.get_mut(&new_tile_id).unwrap().flip_h();
                    if !self.find_match(new_tile_id) {
                        self.tiles.get_mut(&new_tile_id).unwrap().flip_h();
                        self.tiles.get_mut(&new_tile_id).unwrap().flip_v();
                        if !self.find_match(new_tile_id) {
                            self.tiles.get_mut(&new_tile_id).unwrap().flip_v();
                            self.queue.push_back(new_tile_id);
                        }
                    }
                }
            }

            self.done = true;
        }

        fn get_full_picture(&mut self) -> Vec<Vec<char>> {
            self.assemble();

            let image = &self.assembled;
            let min_x = image.values().map(|v| v.0).min().unwrap();
            let max_x = image.values().map(|v| v.0).max().unwrap();
            let min_y = image.values().map(|v| v.1).min().unwrap();
            let max_y = image.values().map(|v| v.1).max().unwrap();


            // assemble full picture
            let rev_map: HashMap<(i32, i32), u64> = image.iter().map(|(k, v)| (*v, *k)).collect();
            let mut full_picture: Vec<Vec<char>> = Vec::new();

            for y in min_y..=max_y {
                for i in (1..SIDE_LENGTH-1).rev() {
                    full_picture.push(Vec::new());
                    for x in min_x..=max_x {
                        full_picture.last_mut().unwrap().append(
                            &mut self.tiles[&rev_map[&(x, y)]].content[i][1..SIDE_LENGTH-1].to_vec());
                    }
                }
            }

            full_picture
        }
    }

    pub fn solve_a(tiles: &[Tile]) -> u64 {
        let mut assembly_line = AssemblyLine::new(tiles);
        assembly_line.assemble();

        let image = assembly_line.assembled;

        let min_x = image.values().map(|v| v.0).min().unwrap();
        let max_x = image.values().map(|v| v.0).max().unwrap();
        let min_y = image.values().map(|v| v.1).min().unwrap();
        let max_y = image.values().map(|v| v.1).max().unwrap();

        image.keys().filter(
            |k| image[k] == (min_x, min_y)
             || image[k] == (min_x, max_y)
             || image[k] == (max_x, min_y)
             || image[k] == (max_x, max_y)).product::<u64>()
    }

    fn rotate90(pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let w = pattern[0].len();

        (0..w).rev().map(|i| pattern.iter().map(|row| row[i]).collect()).collect()
    }

    fn flip_h(mut pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
        for row in pattern.iter_mut() {
            row.reverse();
        }

        pattern
    }

    fn flip_v(mut pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
        pattern.reverse();

        pattern
    }


    pub fn solve_b(tiles: &[Tile]) -> usize {
        let mut assembly_line = AssemblyLine::new(tiles);
        let full_picture = assembly_line.get_full_picture();
        let pattern = vec![
            "                  # ".chars().collect(),
            "#    ##    ##    ###".chars().collect(),
            " #  #  #  #  #  #   ".chars().collect(),
        ];

        let patterns = vec![
            pattern.clone(),
            rotate90(pattern.clone()),
            rotate90(rotate90(pattern.clone())),
            rotate90(rotate90(rotate90(pattern.clone()))),
            flip_h(pattern.clone()),
            rotate90(flip_h(pattern.clone())),
            rotate90(rotate90(flip_h(pattern.clone()))),
            rotate90(rotate90(rotate90(flip_h(pattern.clone())))),
            flip_v(pattern.clone()),
            rotate90(flip_v(pattern.clone())),
            rotate90(rotate90(flip_v(pattern.clone()))),
            rotate90(rotate90(rotate90(flip_v(pattern.clone())))),
        ];

        // Search for 2D pattern and nullify
        let n_match = patterns.iter().map(|pattern| {
            let h_pic = full_picture.len();
            let w_pic = full_picture[0].len();

            let h_pat = pattern.len();
            let w_pat = pattern[0].len();

            let origins: Vec<(usize, usize)> = (0..h_pic - h_pat + 1).flat_map(|i_pic|
                (0..w_pic - w_pat + 1).map(move |j_pic| (i_pic, j_pic))).collect();

            origins.iter().filter(
                |(i_pic, j_pic)| {
                    let pat_px: Vec<(usize, usize)> = (0..h_pat).flat_map(|i_pat|
                        (0..w_pat).map(move |j_pat| (i_pat, j_pat))).collect();

                    pat_px.iter().filter(
                        |(i_pat, j_pat)| pattern[*i_pat][*j_pat] == ' '
                            || pattern[*i_pat][*j_pat] == full_picture[i_pic+i_pat][j_pic+j_pat]
                            ).count() == h_pat * w_pat
                }).count()
        }).max().unwrap();

        let total_pic: usize = full_picture.iter().map(|row| row.iter().filter(|&&c| c=='#').count()).sum();
        let total_path: usize = pattern.iter().map(|row| row.iter().filter(|&&c| c=='#').count()).sum();

        total_pic - n_match * total_path
    }
}

fn main() {
    use regex::Regex;
    use std::io::{self, Read};

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re_tile = Regex::new(r"Tile (?P<id>\d+):\n(?P<tile>([#\.]{10}\n?){10})").unwrap();

    let tiles: Vec<_> = re_tile.captures_iter(&buffer).map(
        |c| day20::Tile::new(
            c.name("id").unwrap().as_str().parse().unwrap(),
            c.name("tile").unwrap().as_str().lines().map(|l| l.chars().collect()).collect(),
            )).collect();

    println!("Solution A-part: {}", day20::solve_a(&tiles));
    println!("Solution B-part: {}", day20::solve_b(&tiles));
}
