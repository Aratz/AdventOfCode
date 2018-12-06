fn distance(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut coordinates = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut xy = line.split(", ").map(|x| x.parse::<i32>().unwrap());
        let x = xy.next().unwrap();
        let y = xy.next().unwrap();

        coordinates.push((x, y));
    }

    let xmin = -200;
    let ymin = -200;
    let xmax = 600;
    let ymax = 600;

    let mut area = 0;
    for x in xmin..xmax {
        for y in ymin..ymax {
            let sum_dist:i32 = coordinates.iter()
                .map(|xy| distance(&xy, &(x, y))).sum();
            area += if sum_dist < 10000 { 1 } else { 0 };
        }
    }
    println!("{:?}", area);
}
