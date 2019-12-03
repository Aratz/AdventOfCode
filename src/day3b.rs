fn distance(p1:&(i32, i32), p2:&(i32, i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

fn convert_to_coordinates<'a, T: Iterator<Item=&'a str>>(moves: T) -> Vec<(i32, i32)> {
    let start = (0, 0);
    let mut res = Vec::new();
    res.push(start);
    for m in moves {
        let new_pos = {
            let current_pos = res.last().unwrap();
            let direction = &m[..1];
            let distance = m[1..].parse::<i32>().unwrap()
                * if ["R", "U"].contains(&direction) {1} else {-1};
            (
                current_pos.0 + if ["L", "R"].contains(&direction) {distance} else {0},
                current_pos.1 + if ["U", "D"].contains(&direction) {distance} else {0})
        };
        res.push(new_pos);
    }

    return res
}

fn main() {
    use std::io::{self, BufRead};
    use std::cmp;

    let stdin = io::stdin();

    let wires = (0..2).map(
        |_| convert_to_coordinates(
            stdin.lock().lines().next().unwrap().unwrap().split(",")))
        .collect::<Vec<_>>();

    let sections = wires.iter().map(|wire| wire.iter().zip(wire.iter().skip(1))
                                    .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dist = sections.iter().map(|section| section.iter().scan(
            0, |d, &s| {
                let d1 = *d;
                *d = *d + distance(s.0, s.1);
                Some(d1)
            }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let res = sections[0].iter().zip(dist[0].iter()).filter_map(
        |(section1, dist1)| sections[1].iter().zip(dist[1].iter()).filter_map(
            |(section2, dist2)| {
                let (x1, x2, x, y1, y2, y) = if (section1.0).0 == (section1.1).0 {
                    ((section2.0).0, (section2.1).0, (section1.0).0,
                     (section1.0).1, (section1.1).1, (section2.0).1)
                }
                else {
                    ((section1.0).0, (section1.1).0, (section2.0).0,
                     (section2.0).1, (section2.1).1, (section1.0).1)
                };

                let (x1, x2) = (cmp::min(x1, x2), cmp::max(x1, x2));
                let (y1, y2) = (cmp::min(y1, y2), cmp::max(y1, y2));

                if x1 < x && x < x2 && y1 < y && y < y2 {
                    Some(dist1 + distance(section1.0, &(x, y))
                        + dist2 + distance(section2.0, &(x, y)))
                }
                else {
                    None
                }
            }).min()).min().unwrap();

    println!("{}", res);
}
