mod day03 {
    fn is_triangle(sides: &Vec<i32>) -> bool {
        let mut sides = sides.to_vec();
        sides.sort();

        sides[0] + sides[1] > sides[2]
    }

    pub fn solve_a(lines: &Vec<Vec<i32>>) -> usize {
        lines.iter()
            .map(|sides| is_triangle(&sides))
            .filter(|&is_tri| is_tri)
            .count()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let specs: Vec<_> = stdin.lock().lines()
        .map(|line| String::from(line.unwrap().trim()))
        .map(|line| line.split(' ').filter(|&s| !s.is_empty())
             .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect();

    println!("Solution A-part: {}", day03::solve_a(&specs));
}
