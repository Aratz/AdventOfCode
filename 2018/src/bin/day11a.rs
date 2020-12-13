fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let snumber = stdin.lock().lines().next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    let grid = (1..=300).map(|x| (1..=300).map(|y|
        (((x+10)*y + snumber)*(x+10)/100)%10 - 5
        ).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    let tuples = (0..300 - 3).flat_map(|x|
        (0..300 - 3).map(move |y| (x, y)));

    let squares = tuples
        .map(|(x, y)| (grid[x..x+3].iter()
             .map::<i32, _>(|column| column[y..y+3].iter().sum()).sum::<i32>(), (x, y)));

    let answer = squares.max().unwrap();
    println!("{},{}", (answer.1).0 + 1, (answer.1).1 + 1);

}
