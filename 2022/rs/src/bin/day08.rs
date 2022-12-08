mod day08 {
    fn is_visible(coord: (usize, usize), tree_map: &[Vec<usize>]) -> bool {
        let (x, y) = coord;
        //top
        {
            let (mut xt, mut yt) = coord;
            while xt > 0 {
                if tree_map[xt - 1][yt] < tree_map[x][y] { xt -= 1; }
                else { break; }
            }
            if xt == 0 {
                return true;
            }
        }
        //right
        {
            let (mut xt, mut yt) = coord;
            while yt < tree_map[0].len() - 1 {
                if tree_map[xt][yt + 1] < tree_map[x][y] { yt += 1; }
                else { break; }
            }
            if yt == tree_map[0].len() - 1 {
                return true;
            }
        }
        //bottom
        {
            let (mut xt, mut yt) = coord;
            while xt < tree_map.len() - 1 {
                if tree_map[xt + 1][yt] < tree_map[x][y] { xt += 1; }
                else { break; }
            }
            if xt == tree_map.len() - 1 {
                return true;
            }
        }
        //left
        {
            let (mut xt, mut yt) = coord;
            while yt > 0 {
                if tree_map[xt][yt - 1] < tree_map[x][y] { yt -= 1; }
                else { break; }
            }
            if yt == 0 {
                return true;
            }
        }

        false
    }

    fn scenic_score(coord: (usize, usize), tree_map: &[Vec<usize>]) -> usize {
        let mut res = 1;
        let (x, y) = coord;
        //top
        {
            let (mut xt, mut yt) = coord;
            while xt > 0 && tree_map[xt - 1][yt] < tree_map[x][y] {
                xt -= 1;
            }
            if xt > 0 { xt -= 1; }
            res *= x.abs_diff(xt);
        }
        //right
        {
            let (mut xt, mut yt) = coord;
            while yt < tree_map[0].len() - 1 && tree_map[xt][yt + 1] < tree_map[x][y] {
                yt += 1;
            }
            if yt < tree_map[0].len() - 1 { yt += 1; }
            res *= y.abs_diff(yt);
        }
        //bottom
        {
            let (mut xt, mut yt) = coord;
            while xt < tree_map.len() - 1 && tree_map[xt + 1][yt] < tree_map[x][y] {
                xt += 1;
            }
            if xt < tree_map.len() - 1 { xt += 1; }
            res *= x.abs_diff(xt);
        }
        //left
        {
            let (mut xt, mut yt) = coord;
            while yt > 0 && tree_map[xt][yt - 1] < tree_map[x][y] {
                yt -= 1;
            }
            if yt > 0 { yt -= 1; }
            res *= y.abs_diff(yt);
        }

        res
    }

    fn parse(input: &str) -> Vec<Vec<usize>> {
        input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
            .collect()
    }

    pub fn solve_a(input: &str) -> usize {
        let tree_map = parse(input);
        let (max_x, max_y) = (tree_map.len(), tree_map[0].len());
        (0..max_x).map(
            |x| (0..max_y).filter(|&y| is_visible((x, y), &tree_map)).count())
            .sum()
    }

    pub fn solve_b(input: &str) -> usize {
        let tree_map = parse(input);
        let (max_x, max_y) = (tree_map.len(), tree_map[0].len());
        (1..max_x-1).map(
            |x| (1..max_y-1).map(|y| scenic_score((x, y), &tree_map)).max().unwrap())
            .max().unwrap()
    }

    #[cfg(test)]
    mod test_day08 {
        use super::*;

        #[test]
        fn test_is_visible() {
            let input = "30373
25512
65332
33549
35390";
            let tree_map = parse(&input);
            assert!(is_visible((2, 1), &tree_map));
        }

        #[test]
        fn test_scenic_score() {
            let input = "30373
25512
65332
33549
35390";
            let tree_map = parse(&input);
            assert_eq!(scenic_score((3, 2), &tree_map), 8);
        }

        #[test]
        fn test_solve_a() {
            let input = "30373
25512
65332
33549
35390";

            assert_eq!(solve_a(&input), 21);
        }

        #[test]
        fn test_solve_b() {
            let input = "30373
25512
65332
33549
35390";

            assert_eq!(solve_b(&input), 8);
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

    println!("Solution A-part: {}", day08::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day08::solve_b(&buffer.trim()));
}
