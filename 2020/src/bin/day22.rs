extern crate regex;

mod day22 {
    use std::collections::{VecDeque, HashSet};

    pub fn solve_a(cards1 : &Vec<usize>, cards2: &Vec<usize>) -> usize {
        let mut decks = vec![
            VecDeque::from(cards1.clone()),
            VecDeque::from(cards2.clone()),
        ];

        while !decks[0].is_empty() && !decks[1].is_empty() {
            let cards = vec![decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap()];
            let winner = if cards[0] > cards[1] { 0 } else { 1 };
            decks[winner].push_back(cards[winner]);
            decks[winner].push_back(cards[1-winner]);
        }

        let winner = if decks[1].is_empty() { 0 } else { 1 };

        decks[winner].iter().rev().enumerate().map(|(i, c)| (i + 1) * c).sum::<usize>()
    }

    fn play_rec(
        cards1: &Vec<usize>,
        cards2: &Vec<usize>,
        ) -> (usize, usize) {

        let mut decks = vec![
            VecDeque::from(cards1.clone()),
            VecDeque::from(cards2.clone()),
        ];

        let mut previously_played: HashSet<Vec<VecDeque<usize>>> = HashSet::new();


        while !decks[0].is_empty() && !decks[1].is_empty() {
            if previously_played.contains(&decks) {
                decks[1].clear();
                break;
            }
            else {
                previously_played.insert(decks.clone());
            }

            let cards = vec![decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap()];
            let winner = if cards[0] <= decks[0].len() && cards[1] <= decks[1].len() {
                play_rec(
                    &decks[0].iter().take(cards[0]).map(|v| *v).collect(),
                    &decks[1].iter().take(cards[1]).map(|v| *v).collect(),
                    //&decks[0].make_contiguous()[0..cards[0]].to_vec(),
                    //&decks[1].make_contiguous()[0..cards[1]].to_vec(),
                    //This creates card doubles, I don't really understand why :(
                    ).0
            }
            else {
                if cards[0] > cards[1] { 0 } else { 1 }
            };
            decks[winner].push_back(cards[winner]);
            decks[winner].push_back(cards[1-winner]);

        }

        let winner = if decks[1].is_empty() { 0 } else { 1 };

        (
            winner,
            decks[winner].iter().rev().enumerate().map(|(i, c)| (i + 1) * c).sum::<usize>()
            )
    }

    pub fn solve_b(cards1 : &Vec<usize>, cards2: &Vec<usize>) -> usize {
        play_rec(cards1, cards2).1
    }

    #[cfg(test)]
    mod test_day22 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let cards1 = vec![9, 2, 6, 3, 1];
            let cards2 = vec![5, 8, 4, 7, 10];

            assert_eq!(solve_a(&cards1, &cards2), 306);
        }

        #[test]
        fn test_solve_b() {
            let cards1 = vec![9, 2, 6, 3, 1];
            let cards2 = vec![5, 8, 4, 7, 10];

            assert_eq!(solve_b(&cards1, &cards2), 291);
        }

        #[test]
        fn test_solve_b_inf() {
            let cards1 = vec![43, 19];
            let cards2 = vec![2, 29, 14];

            assert_eq!(solve_b(&cards1, &cards2), 105);
        }
    }
}

fn main() {
    use regex::Regex;
    use std::io::{self, Read};

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re_cards = Regex::new(r"Player (?P<player_id>\d):\n(?P<deck>(\d+\n?)+)").unwrap();

    let cards: Vec<Vec<usize>> = re_cards.captures_iter(&buffer).map(
        |capt| capt.name("deck").unwrap().as_str().lines().map(
            |card| card.parse().unwrap()).collect()
        ).collect();

    println!("Solution A-part: {}", day22::solve_a(&cards[0], &cards[1]));
    println!("Solution B-part: {}", day22::solve_b(&cards[0], &cards[1]));
}
