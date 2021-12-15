mod day13 {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn do_fold(points: &[(i32, i32)], fold: (i32, i32)) -> Vec<(i32, i32)> {
        let unique_points: HashSet<(i32, i32)> = HashSet::from_iter(
            points.iter().map(|&(x, y)| {
            if fold.0 > 0 {
                if x > fold.0 { (fold.0 - (x - fold.0), y) } else { (x, y) }
            }
            else {
                if y > fold.1 { (x, fold.1 - (y - fold.1)) } else { (x, y) }
            }
        }));

        unique_points.into_iter().collect()
    }

    pub fn solve_a(points: &[(i32, i32)], folds: &[(i32, i32)]) -> usize {
        do_fold(points, folds[0]).len()
    }

    pub fn solve_b(points: &[(i32, i32)], folds: &[(i32, i32)]) {
        let mut points = points.to_vec();

        for &fold in folds {
            points = do_fold(&points, fold);
        }

        let max_x = *points.iter().map(|(x, _)| x).max().unwrap() + 1;
        let max_y = *points.iter().map(|(_, y)| y).max().unwrap() + 1;

        let mut paper = vec![vec![' '; max_x as usize]; max_y as usize];

        for (x, y) in points {
            paper[y as usize][x as usize] = '▮';
        }

        for row in paper {
            println!("{}", row.iter().collect::<String>());
        }
    }

    #[cfg(test)]
    mod test_day13 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let points = vec![
                (6,10),
                (0,14),
                (9,10),
                (0,3),
                (10,4),
                (4,11),
                (6,0),
                (6,12),
                (4,1),
                (0,13),
                (10,12),
                (3,4),
                (3,0),
                (8,4),
                (1,10),
                (2,14),
                (8,10),
                (9,0),
            ];

            let folds = vec![(0, 7), (5, 0)];

            assert_eq!(solve_a(&points, &folds), 17);
        }
    }
}

fn main() {
    use std::io::{self, Read};

    let stdin = io::stdin();

    let mut buffer = String::new();

    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let points_folds:Vec<String> = buffer.split("\n\n")
        .map(|s| s.to_string()).collect();
    let (points, folds) = (points_folds[0].clone(), points_folds[1].clone());

    let points:Vec<(i32, i32)> = points.lines()
        .map(|s| {
            let xy: Vec<i32> = s.split(',').map(|v| v.parse().unwrap()).collect();
            (xy[0], xy[1])
        }).collect();

    let folds: Vec<(i32, i32)> = folds.lines()
        .map(|s| {
            let s: Vec<&str> = s.split('=').collect();
            (
                if s[0].chars().last().unwrap() == 'x' { s[1].parse().unwrap() } else { 0 },
                if s[0].chars().last().unwrap() == 'y' { s[1].parse().unwrap() } else { 0 }
                )
        }).collect();

    println!("Solution A-part: {}", day13::solve_a(&points, &folds));
    println!("Solution B-part:\n"); day13::solve_b(&points, &folds);
}
