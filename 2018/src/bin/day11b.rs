fn main() {
    use std::io::{self, BufRead};
    use std::cmp;

    let stdin = io::stdin();

    let snumber = stdin.lock().lines().next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    let grid = (1..=300).map(|x| (1..=300).map(|y|
        (((x+10)*y + snumber)*(x+10)/100)%10 - 5
        ).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    let tuples = (0..300).flat_map(|x| (0..300).map(move |y| (x, y)));

    let mut cumgrid = (1..=300).map(|_x| (1..=300).map(|_y| 0)
        .collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    for (x,y) in tuples {
        cumgrid[x][y] = grid[x][y]
            - if x > 0 && y >0 { cumgrid[x-1][y-1] } else { 0 }
            + if x > 0 { cumgrid[x-1][y] } else { 0 }
            + if y >0 { cumgrid[x][y-1] } else { 0 };
    }

    let tuples_size = (0..300).flat_map(|x| (0..300).map(move |y| (x, y)))
        .flat_map(|(x,y)| (0..300-cmp::max(x, y)).map(move |s| (x, y, s)));

    let answer = tuples_size.map(|(x, y, s)|
                    (cumgrid[x + s][y + s]
                    + if x > 0 && y >0 { cumgrid[x-1][y-1] } else { 0 }
                    - if x > 0 { cumgrid[x-1][y + s] } else { 0 }
                    - if y >0 { cumgrid[x + s][y-1] } else { 0 },
                    (x, y, s))).max().unwrap();

    println!("{},{},{}", (answer.1).0, (answer.1).1, (answer.1).2);
}
