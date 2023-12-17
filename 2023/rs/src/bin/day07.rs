mod day07 {
    use std::collections::HashMap;

    fn card2value(card: char, a: bool) -> u32 {
        let cardmap: HashMap::<char, u32> = vec![
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', if a { 11 } else { 1 }),
            ('T', 10),
        ].into_iter().collect();

        cardmap.get(&card).map_or_else(
            || card.to_digit(10).unwrap(),
            |v| *v)
    }

    #[derive(Debug)]
    struct Hand {
        cards: Vec<u32>,
        bid: u32,
    }

    impl Hand {
        fn hand_type(&self) -> Vec<u32> {
            let mut hand_count = self.cards.iter()
                .fold(
                    HashMap::new(),
                    |mut dict, card| {
                        let entry = dict.entry(card).or_default();
                        *entry += 1;
                        dict
                    }
                );

            let n_J = hand_count.remove(&1).unwrap_or_default();

            if n_J < 5 {
                let mut hand_type = hand_count.values().cloned().collect::<Vec<u32>>();
                hand_type.sort();
                hand_type.reverse();
                hand_type[0] += n_J;
                hand_type
            }
            else {
                vec![5]
            }
        }
    }

    fn parse(input: &str, a: bool) -> Vec<Hand> {
        input.lines()
            .map(
                |l| {
                    let hand_bid: Vec<_> = l.split(' ').collect();
                    Hand {
                        cards: hand_bid[0].chars()
                            .map(|c| card2value(c, a))
                            .collect(),
                        bid: hand_bid[1].parse().unwrap()
                    }
                }
            ).collect()
    }

    pub fn solve(input: &str, a: bool) -> u32 {
        let mut hands = parse(input, a);

        hands.sort_by_key(
            |hand| (
                hand.hand_type(),
                hand.cards.clone()
            )
        );

        hands.into_iter().enumerate()
            .map(|(i, hand)| (i as u32 +1) * hand.bid)
            .sum()
    }

    #[cfg(test)]
    mod test_day07 {
        use super::*;

        static INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve(INPUT, true), 6440);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve(INPUT, false), 5905);
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

    println!("Solution A-part: {}", day07::solve(&buffer.trim(), true));
    println!("Solution B-part: {}", day07::solve(&buffer.trim(), false));
}
