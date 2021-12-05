mod day05 {
    use std::collections::HashMap;
    use std::cmp::{min, max};

    pub fn solve_a(vents: &[((i32, i32), (i32, i32))]) -> usize {
        let mut map: HashMap<(i32, i32), usize> = HashMap::new();

        for &((x1, y1), (x2, y2)) in vents {
            if x1 != x2 && y1 != y2 { continue; }

            let ((x1, y1), (x2, y2)) = (
                (min(x1, x2), min(y1, y2)),
                (max(x1, x2), max(y1, y2))
            );
            for x in x1..=x2 {
                for y in y1..=y2 {
                    *map.entry((x, y)).or_default() += 1;
                }
            }
        }

        map.values().filter(|&&v| v >= 2).count()
    }

    pub fn solve_b(vents: &[((i32, i32), (i32, i32))]) -> usize {
        let mut map: HashMap<(i32, i32), usize> = HashMap::new();

        for &((x1, y1), (x2, y2)) in vents {
            if x1 != x2 && y1 != y2 {
                let dx = if x1 < x2 { 1 } else { -1 };
                let dy = if y1 < y2 { 1 } else { -1 };

                let (mut x, mut y) = (x1, y1);
                while x != x2 {
                    *map.entry((x, y)).or_default() += 1;
                    x += dx; y += dy;
                }
                *map.entry((x, y)).or_default() += 1;
            }
            else {
                let ((x1, y1), (x2, y2)) = (
                    (min(x1, x2), min(y1, y2)),
                    (max(x1, x2), max(y1, y2))
                );
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        *map.entry((x, y)).or_default() += 1;
                    }
                }
            }
        }

        map.values().filter(|&&v| v >= 2).count()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let vents: Vec<((i32, i32), (i32, i32))> = stdin.lock().lines()
        .map(|l| l.unwrap().split(" -> ")
             .flat_map(|xy| xy.split(","))
             .map(|n| n.parse().unwrap())
             .collect::<Vec<i32>>())
        .map(|v| ((v[0], v[1]), (v[2], v[3])))
        .collect();

    println!("Solution A-part: {}", day05::solve_a(&vents));
    println!("Solution B-part: {}", day05::solve_b(&vents));
}
