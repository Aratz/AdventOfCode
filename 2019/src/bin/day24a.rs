use std::io::{self, Read};
use std::collections::HashSet;

fn get_mask(x: usize, y: usize) -> i32 {
    1<<(5*y + x)
}

fn get_neighbors(x: usize, y:usize, layout: i32) -> usize {
    vec![(0, 1), (2, 1), (1, 0), (1, 2)].iter()
        .filter(|&(dx, dy)|
            (1 <= x + dx && x + dx <= 5)
            && (1 <= y + dy && y + dy <= 5))
        .filter(|&(dx, dy)| layout & get_mask(x + dx -1, y + dy - 1) > 0)
        .count()
}

fn update(mut layout: i32) -> i32 {
    let mut new_layout = layout;
    for x in 0..5 {
        for y in 0..5 {
            let mask = get_mask(x, y);
            if layout & mask == mask {
                if get_neighbors(x, y, layout) != 1 {
                    new_layout ^= mask;
                }
            }
            else if layout & mask == 0 {
                if get_neighbors(x, y, layout) == 1 || get_neighbors(x, y, layout) == 2 {
                    new_layout ^= mask;
                }
            }
        }
    }

    new_layout
}

fn print_layout(layout: i32) {
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

    for y in 0..5 {
        for x in 0..5 {
            print!("{}", get_neighbors(x, y, layout));
        }
        println!();
    }
    println!();
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let mut layout: i32 = buffer.chars().filter(|&c| c != '\n').enumerate()
        .map(|(i, c)|
            if c == '#' {
                1<<i
            }
            else { 0 }
            ).sum();

    let mut visited = HashSet::new();

    while !visited.contains(&layout) {
        visited.insert(layout);
        layout = update(layout);
    }

    println!("{}", layout);
}
