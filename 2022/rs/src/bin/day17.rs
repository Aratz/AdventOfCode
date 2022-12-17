mod day17 {
    use std::cmp::max;
    use std::collections::HashMap;

    const WIDTH: usize = 7;

    fn get_stones() -> Vec<Vec<(i64, i64)>> {
        vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],

            vec![        (1, 2),
                 (0, 1), (1, 1), (2, 1),
                         (1, 0)],

            vec![                (2, 2),
                                 (2, 1),
                 (0, 0), (1, 0), (2, 0),],

            vec![(0, 3),
                 (0, 2),
                 (0, 1),
                 (0, 0)],

            vec![(0, 1), (1, 1),
                 (0, 0), (1, 0)],
        ]
    }

    fn is_free(pos: (i64, i64), stone: &[(i64, i64)], tower: &[[bool; WIDTH]]) -> bool {
        stone.iter().all(|(dx, dy)|
                    0 <= pos.0 + dx
                    && pos.0 + dx < WIDTH as i64
                    && 0 <= pos.1 + dy
                    &&  !tower[(pos.1 + dy) as usize][(pos.0 + dx) as usize])

    }

    pub fn solve_a(input: &str) -> usize {
        let mut tower = vec![[false; WIDTH]; 3];
        let mut h_tower = 0;
        let mut jet_gas = input.chars().cycle();

        for stone in get_stones().into_iter().cycle().take(2022) {
            let h_stone = *stone.iter().map(|(_, dy)| dy).max().unwrap() as usize + 1;

            let stone_ext = (3 + h_stone + h_tower) as i64 - tower.len() as i64;
            if stone_ext > 0 {
                tower.extend_from_slice(
                    &vec![[false; WIDTH]; stone_ext as usize]);
            }

            let mut pos = (2,  (h_tower + 3) as i64);
            while let Some(dir) = jet_gas.next() {
                let new_pos = if dir == '<' {
                    (pos.0 - 1, pos.1)
                }
                else {
                    (pos.0 + 1, pos.1)
                };

                if is_free(new_pos, &stone, &tower) {
                    pos = new_pos;
                }

                let new_pos = (pos.0, pos.1 - 1);
                if is_free(new_pos, &stone, &tower) {
                    pos = new_pos;
                }
                else {
                    for (dx, dy) in &stone {
                        tower[(pos.1 + dy) as usize][(pos.0 + dx) as usize] = true;
                    }

                    h_tower = max(
                        h_tower,
                        stone.iter()
                            .map(|(_dx, dy)| pos.1 + dy + 1)
                            .max().unwrap() as usize);
                    break;
                }
            }
        }

        h_tower
    }

    fn get_profile(tower: &[[bool; WIDTH]]) -> [usize; WIDTH] {
        let mut base = [0; 7];

        for j in 0..7 {
            base[j] = tower.iter()
                .map(|row| row[j])
                .enumerate()
                .filter(|(_, v)| *v)
                .map(|(i, _)| i)
                .max().unwrap_or(0);
        }

        let min_base = *base.iter().min().unwrap();

        for j in 0..7 {
            base[j] -= min_base;
        }

        base
    }

    fn pattern_height(input: &str) -> ([usize; WIDTH], (usize, usize), usize, (usize, usize)) {
        let mut tower = vec![[false; WIDTH]; 3];
        let mut h_tower = 0;
        let mut jet_gas = input.chars().cycle().enumerate().peekable();

        let mut profiles: HashMap<[usize; WIDTH], (usize, usize, usize)> = HashMap::new();

        let n_stone = get_stones().len();

        for (i_s, stone) in get_stones().into_iter().cycle().enumerate().peekable() {
            let h_stone = *stone.iter().map(|(_, dy)| dy).max().unwrap() as usize + 1;

            let stone_ext = (3 + h_stone + h_tower) as i64 - tower.len() as i64;
            if stone_ext > 0 {
                tower.extend_from_slice(
                    &vec![[false; WIDTH]; stone_ext as usize]);
            }

            let mut pos = (2, (h_tower + 3) as i64);
            while let Some((i_d, dir)) = jet_gas.next() {
                let new_pos = if dir == '<' {
                    (pos.0 - 1, pos.1)
                }
                else {
                    (pos.0 + 1, pos.1)
                };

                if is_free(new_pos, &stone, &tower) {
                    pos = new_pos;
                }

                let new_pos = (pos.0, pos.1 - 1);
                if is_free(new_pos, &stone, &tower) {
                    pos = new_pos;
                }
                else {
                    for (dx, dy) in &stone {
                        tower[(pos.1 + dy) as usize][(pos.0 + dx) as usize] = true;
                    }

                    h_tower = max(
                        h_tower,
                        stone.iter()
                            .map(|(_dx, dy)| pos.1 + dy + 1)
                            .max().unwrap() as usize);

                    let profile = get_profile(&tower);

                    if profiles.contains_key(&profile)
                        && i_s % n_stone == profiles[&profile].0 % n_stone
                        && i_d % input.len() == profiles[&profile].1 {
                            return (
                                profile,
                                (profiles[&profile].0, i_s),
                                i_d,
                                (profiles[&profile].2, h_tower));
                        }
                    else {
                        profiles.insert(profile, (i_s, i_d % input.len(), h_tower));
                    }
                    break;
                }
            }
        }
        unreachable!();
    }

    pub fn solve_b(input: &str) -> usize {
        let n_stone = get_stones().len();

        let target = 1_000_000_000_000;
        let (profile, n_stones, i_d, h_tower) = pattern_height(input);

        let d_stone = n_stones.1 - n_stones.0;
        let n_skip = (target - n_stones.0 - 1) / d_stone;

        let height_profile = *profile.iter().max().unwrap() + 1;

        let base_height = h_tower.0
            + n_skip * (h_tower.1 - h_tower.0)
            - height_profile;

        let mut tower = vec![[false; WIDTH]; height_profile + 3];
        for (i, h) in profile.iter().enumerate() {
            for j in 0..=*h {
                tower[j][i] = true;
            }
        }

        let mut h_tower = height_profile;
        let mut jet_gas = input.chars().cycle().skip((i_d + 1) % input.len());

        for stone in get_stones()
                .into_iter().cycle()
                .skip((n_stones.1 + 1) % n_stone)
                .take(target - (n_stones.0 + 1 + (n_stones.1 - n_stones.0) * n_skip))
                {
            let h_stone = *stone.iter().map(|(_, dy)| dy).max().unwrap() as usize + 1;

            let stone_ext = (3 + h_stone + h_tower) as i64 - tower.len() as i64;
            if stone_ext > 0 {
                tower.extend_from_slice(
                    &vec![[false; WIDTH]; stone_ext as usize]);
            }

            let mut pos = (2,  (h_tower + 3) as i64);
            while let Some(dir) = jet_gas.next() {
                let new_pos = if dir == '<' {
                    (pos.0 - 1, pos.1)
                }
                else {
                    (pos.0 + 1, pos.1)
                };

                if is_free(new_pos, &stone, &tower) {
                    pos = new_pos;
                }

                let new_pos = (pos.0, pos.1 - 1);
                if is_free(new_pos, &stone, &tower) {
                    pos = new_pos;
                }
                else {
                    for (dx, dy) in &stone {
                        tower[(pos.1 + dy) as usize][(pos.0 + dx) as usize] = true;
                    }

                    h_tower = max(
                        h_tower,
                        stone.iter()
                            .map(|(_dx, dy)| pos.1 + dy + 1)
                            .max().unwrap() as usize);
                    break;
                }
            }
        }

        h_tower + base_height
    }

    #[cfg(test)]
    mod test_day17 {
        use super::*;

        static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 3068);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 1514285714288);
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

    println!("Solution A-part: {}", day17::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day17::solve_b(&buffer.trim()));
}
