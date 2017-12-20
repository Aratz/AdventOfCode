fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let n = stdin.lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();

    let j = (0..).skip_while(|&j| (2*j + 1 as i32).pow(2) < n).next().unwrap();
    let mut start_point = (j, std::cmp::min(0, 1 - j));

    let length = 2*j + 1;
    let mut linear_pos = (std::cmp::max(0, 2*(j-1) + 1) as i32).pow(2) + 1;

    let directions_temp = [(0, 1), (-1, 0), (0, -1), (1, 0),
                      (0, 1)];
    let mut directions = directions_temp.iter().peekable();

    loop {
        if n == 1 { break; }

        let &(dx, dy) = directions.next().unwrap();
        if linear_pos + length - 1 <= n {
            linear_pos += length - 1;

            start_point.0 += dx*(length-2) + directions.peek().unwrap().0;
            start_point.1 += dy*(length-2) + directions.peek().unwrap().1;
        }
        else {
            start_point.0 += dx*(n - linear_pos);
            start_point.1 += dy*(n - linear_pos);
            break;
        }
    }

    println!("{}", start_point.0.abs() + start_point.1.abs());
}
