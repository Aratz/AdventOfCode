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

    res
}

fn main() {
    use std::io::{self, BufRead};
    use std::cmp;

    let stdin = io::stdin();

    let wire1 = convert_to_coordinates(
        stdin.lock().lines().next().unwrap().unwrap().split(','));
    let wire2 = convert_to_coordinates(
        stdin.lock().lines().next().unwrap().unwrap().split(','));

    let res = wire1.iter().zip(wire1.iter().skip(1)).filter_map(
        |section1| wire2.iter().zip(wire2.iter().skip(1)).filter_map(
            |section2| {
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
                    Some(x.abs() + y.abs())
                }
                else {
                    None
                }
            }).min()).min().unwrap();

    println!("{}", res);
}
