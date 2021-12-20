mod day20 {
    use std::collections::HashSet;

    fn decode(
        image: &HashSet<(i32, i32)>,
        algorithm: &[bool],
        rev_source: bool,
        ) -> HashSet<(i32, i32)> {
        let neighbors: Vec<(i32, i32)> = (-1..=1)
            .flat_map(|di| (-1..=1).map(move |dj| (di, dj)))
            .collect();

        image.iter()
            .flat_map(|(i, j)| neighbors.iter().map(move |(di, dj)| (i + di, j + dj)))
            .filter_map(|(i, j)| {
                let index = neighbors.iter()
                    .map(move |(di, dj)| (i + di, j + dj))
                    .map(|(i, j)| {
                        if !rev_source {
                            if image.contains(&(i, j)) { 1 } else { 0 }
                        }
                        else {
                            if !image.contains(&(i, j)) { 1 } else { 0 }
                        }
                    })
                    .enumerate()
                    .map(|(i, n)| n * 1<<(8 - i))
                    .sum::<usize>();

                if !rev_source {
                    if !algorithm[index] { Some((i, j)) } else { None }
                }
                else {
                    if algorithm[index] { Some((i, j)) } else { None }
                }
            })
            .collect()
    }

    fn parse_image(raw_image: &[String]) -> HashSet<(i32, i32)> {
        raw_image.iter().enumerate()
            .flat_map(|(i, s)| s.chars().enumerate()
                      .filter(|&(_, c)| c == '#')
                      .map(move |(j, _)| (i as i32, j as i32)))
            .collect()
    }

    fn parse_algorithm(raw_algorithm: &str) -> Vec<bool> {
        raw_algorithm.chars().map(|c| c == '#').collect()
    }

    pub fn solve(raw_image: &[String], raw_algorithm: &str, n: usize) -> usize {
        let mut image = parse_image(raw_image);
        let algorithm = parse_algorithm(raw_algorithm);
        let mut rev_source = false;

        for _ in 0..n {
            image = decode(&image, &algorithm, rev_source);
            rev_source = !rev_source;
        }

        image.len()
    }
    //guess 5272, too high
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();

    let raw_alg = lines.next().unwrap().unwrap().to_string();

    let raw_img: Vec<String> = lines.skip(1)
        .map(|l| l.unwrap().to_string())
        .collect();

    println!("Solution A-part: {}", day20::solve(&raw_img, &raw_alg, 2));
    println!("Solution B-part: {}", day20::solve(&raw_img, &raw_alg, 50));

}
