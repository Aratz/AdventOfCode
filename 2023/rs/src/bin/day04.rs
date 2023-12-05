extern crate regex;

mod day04 {
    use regex::Regex;
    use std::collections::HashSet;

    struct Card {
        id: u32,
        win: HashSet<u32>,
        nums: Vec<u32>,
    }

    impl Card {
        fn n_winning_nums(&self) -> u32 {
            self.nums.iter()
                .filter(|n| self.win.contains(n))
                .count() as u32
        }
        fn points(&self) -> u32 {
            let n_winning_nums = self.n_winning_nums();

            if n_winning_nums > 0 { 2_u32.pow(n_winning_nums - 1) } else { 0 }
        }
    }

    fn parse(input: &str) -> Vec<Card> {
        let re_card = Regex::new(r"Card\s+(?P<id>\d+):\s+(?P<win>(\d+\s+)+)\|(?P<nums>(\s+\d+)+)").unwrap();

        re_card.captures_iter(input).map(
            |capt_card| Card {
                id: capt_card["id"].parse().unwrap(),
                win: capt_card["win"]
                    .split(' ')
                    .filter(|num| num.len() > 0)
                    .map(|num| num.parse().unwrap())
                    .collect(),
                nums: capt_card["nums"]
                    .split(' ')
                    .filter(|num| num.len() > 0)
                    .map(|num| num.parse().unwrap())
                    .collect(),
                }
            ).collect()
    }

    pub fn solve_a(input: &str) -> u32 {
        parse(input).into_iter().map(|card| card.points()).sum()
    }

    pub fn solve_b(input: &str) -> u32 {
        let cards = parse(input);
        let mut n_cards = (0..cards.len()).map(|_| 1).collect::<Vec<_>>();

        for card in cards {
            let n_winning_nums = card.n_winning_nums();
            for new_card_id in card.id + 1..card.id + 1 + n_winning_nums {
                n_cards[new_card_id as usize - 1] += n_cards[card.id as usize - 1];
            }
        }

        n_cards.into_iter().sum()
    }

    #[cfg(test)]
    mod test_day04 {
        use super::*;

        static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 13);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 30);
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

    println!("Solution A-part: {}", day04::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day04::solve_b(&buffer.trim()));
}
