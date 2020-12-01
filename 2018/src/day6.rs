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

    let xmin = -50;
    let ymin = -50;
    let xmax = 500;
    let ymax = 500;

    let mut area = vec![0; coordinates.len()];
    for x in xmin..xmax {
        for y in ymin..ymax {
            let min_dist = coordinates.iter()
                .map(|xy| distance(&xy, &(x, y))).min().unwrap();
            let mut closest_points = coordinates.iter().enumerate()
                .filter(|&(_i, &xy)| distance(&xy, &(x, y)) <= min_dist);
            let first_el = closest_points.next().unwrap().0;
            area[first_el] += match closest_points.next() {
                Some(_) => 0,
                None => 1,
            }
        }
    }
    for seed in xmin..xmax {
        for &(x, y) in vec![(xmin, seed), (xmax-1, seed), (seed, ymin), (seed, ymax-1)].iter(){
            let min_dist = coordinates.iter()
                .map(|xy| distance(&xy, &(x, y))).min().unwrap();

            let mut closest_points = coordinates.iter().enumerate()
                .filter(|&(_i, &xy)| distance(&xy, &(x, y)) <= min_dist);
            let first_el = closest_points.next().unwrap().0;
            area[first_el] = match closest_points.next() {
                Some(_) => area[first_el],
                None => 0,
            }
        }
    }
    println!("{:?}", area.iter().max());
}
