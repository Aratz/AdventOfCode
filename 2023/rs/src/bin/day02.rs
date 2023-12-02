mod day02 {
    use std::cmp::max;

    fn parse(input: &str) -> Vec<(i32, Vec<(i32, i32, i32)>)> {
        input.lines().map(
            |l| {
                let game_draws = l.split(": ").collect::<Vec<_>>();
                let game = game_draws[0].split(' ').last().unwrap()
                    .parse::<i32>().unwrap();
                let draws =game_draws[1].split("; ")
                    .map(|draw| draw.split(", ")
                         .map(|s_n_color| {
                             let n_color = s_n_color.split(' ').collect::<Vec<_>>();
                             let n = n_color[0].parse().unwrap();
                             match n_color[1] {
                                "red" => (n, 0, 0),
                                "green" => (0, n, 0),
                                "blue" => (0, 0, n),
                                _ => unreachable!(),
                             }
                         }).fold(
                             (0, 0, 0),
                             |(acc_r, acc_g, acc_b), (x_r, x_g, x_b)|
                             (acc_r + x_r, acc_g + x_g, acc_b + x_b)
                        )
                    ).collect();
                (game, draws)
            }).collect()
    }

    pub fn solve_a(input: &str) -> i32 {
        let max_cubes = (12, 13, 14);
        let games = parse(input);

        games.into_iter()
            .filter(
                |(_, draws)|
                draws.iter().all(
                    |&(r, g, b)|
                    r <= max_cubes.0 && g <= max_cubes.1 && b <= max_cubes.2
                )
            ).map(|(game_id, _)| game_id)
            .sum()
    }

    pub fn solve_b(input: &str) -> i32 {
        let games = parse(input);

        games.into_iter()
            .map(
                |(_, draws)|
                draws.into_iter().fold(
                    (0, 0, 0),
                    |(acc_r, acc_g, acc_b), (x_r, x_g, x_b)|
                    (max(acc_r, x_r), max(acc_g, x_g), max(acc_b, x_b))
                )
            ).map(|(r, g, b)| r*g*b)
            .sum()
    }

    #[cfg(test)]
    mod test_day02 {
        use super::*;

        static INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 8);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 2286);
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

    println!("Solution A-part: {}", day02::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day02::solve_b(&buffer.trim()));
}
