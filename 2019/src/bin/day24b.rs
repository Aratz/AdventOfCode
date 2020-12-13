use std::io::{self, Read};
use std::collections::HashMap;

fn get_mask(x: usize, y: usize) -> i32 {
    1<<(5*y + x)
}

struct Grid {
    layouts: HashMap<i32, i32>
}

#[allow(dead_code)]
impl Grid {
    fn new() -> Self {
        Grid { layouts:HashMap::new() }
    }
    fn get_neighbors(&self, x: usize, y:usize, level: i32) -> usize {
        vec![(0, 1), (2, 1), (1, 0), (1, 2)].iter()
            .map(|(dx, dy)| {
                let outer_layout = match self.layouts.get(&(level - 1)) {
                    Some(&x) => x,
                    None => 0,
                };
                let layout = self.layouts[&level];
                let inner_layout = match self.layouts.get(&(level + 1)) {
                    Some(&x) => x,
                    None => 0,
                };

                // Outer edge
                if x + dx < 1 { 
                    if outer_layout & get_mask(1, 2) > 0 { 1 } else { 0 }
                }
                else if x + dx >= 6 {
                    if outer_layout & get_mask(3, 2) > 0 { 1 } else { 0 }
                }
                else if y + dy < 1 {
                    if outer_layout & get_mask(2, 1) > 0 { 1 } else { 0 }
                }
                else if y + dy >= 6 {
                    if outer_layout & get_mask(2, 3) > 0 { 1 } else { 0 }
                }

                // Inner edge
                else if x + dx - 1 == 2 && y + dy - 1 == 2 {
                    if x == 1 {
                        (0..5).map(|y| if inner_layout & get_mask(0, y) > 0 { 1 } else { 0 })
                            .sum()
                    }
                    else if x == 3 {
                        (0..5).map(|y| if inner_layout & get_mask(4, y) > 0 { 1 } else { 0 })
                            .sum()
                    }
                    else if y == 1 {
                        (0..5).map(|x| if inner_layout & get_mask(x, 0) > 0 { 1 } else { 0 })
                            .sum()
                    }
                    else if y == 3 {
                        (0..5).map(|x| if inner_layout & get_mask(x, 4) > 0 { 1 } else { 0 })
                            .sum()
                    }
                    else {
                        panic!("Neighbor count error!");
                    }
                }
                else {
                    if layout & get_mask(x + dx -1, y + dy - 1) > 0 { 1 } else { 0 }
                }
            }).sum()
    }

    fn update(&mut self) {

        if let Some((&min_level, &layout)) = self.layouts.get_key_value(self.layouts.keys().min().unwrap()) {
            if layout > 0 {
                self.layouts.insert(min_level - 1, 0);
            }
        }
        if let Some((&max_level, &layout)) = self.layouts.get_key_value(self.layouts.keys().max().unwrap()) {
            if layout > 0 {
                self.layouts.insert(max_level + 1, 0);
            }
        }

        let mut new_grid = self.layouts.clone();

        for (&level, &layout) in self.layouts.iter() {
            for x in 0..5 {
                for y in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let mask = get_mask(x, y);
                    let neighbors = self.get_neighbors(x, y, level);
                    if layout & mask == mask {
                        if neighbors != 1 {
                            new_grid.entry(level)
                                .or_insert(0);
                            new_grid.entry(level)
                                .and_modify(|e| *e ^= mask);
                        }
                    }
                    else if layout & mask == 0 {
                        if neighbors == 1 || neighbors == 2 {
                            new_grid.entry(level)
                                .or_insert(0);
                            new_grid.entry(level)
                                .and_modify(|e| *e ^= mask);
                        }
                    }
                }
            }
        }

        self.layouts = new_grid;
    }

    fn count_bugs(&self) -> i32 {
        self.layouts.values().map(|&layout|
            (0..5).map(|x|
                (0..5).map(|y|
                    if layout & get_mask(x, y) > 0 { 1 } else { 0 }
                ).sum::<i32>()
            ).sum::<i32>()
        ).sum::<i32>()
    }

    fn print_layout(&self) {
        let mut keys = self.layouts.keys().map(|x| *x).collect::<Vec<i32>>();
        keys.sort();

        for &level in keys.iter() {
            let layout = self.layouts[&level];
            println!("Depth {}:", level);
            for y in 0..5 {
                for x in 0..5 {
                    if layout & get_mask(x, y) == get_mask(x, y) {
                        print!("#");
                    }
                    else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
        println!();

        for &level in keys.iter() {
            println!("Depth {}:", level);
            for y in 0..5 {
                for x in 0..5 {
                    print!("{}", self.get_neighbors(x, y, level));
                }
                println!();
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let layout: i32 = buffer.chars().filter(|&c| c != '\n').enumerate()
        .map(|(i, c)|
            if c == '#' {
                1<<i
            }
            else { 0 }
            ).sum();

    let mut grid = Grid::new();

    grid.layouts.insert(0, layout);


    for _ in 0..200 {
        grid.update();
    }

    println!("{}", grid.count_bugs());
}
