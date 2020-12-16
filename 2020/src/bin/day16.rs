extern crate regex;

mod day16 {
    pub struct FieldRange {
        l1: i32,
        u1: i32,
        l2: i32,
        u2: i32,
    }

    impl FieldRange {
        pub fn new(l1: i32, u1: i32, l2: i32, u2: i32) -> Self {
            FieldRange {
                l1, u1, l2, u2,
            }
        }

        fn is_valid(&self, x: i32) -> bool {
            self.l1 <= x &&  x <= self.u1
            || self.l2 <= x &&  x <= self.u2
        }
    }

    pub fn solve_a(ranges: &Vec<FieldRange>, tickets: &Vec<Vec<i32>>) -> i32 {
        tickets.iter().flat_map(
            |ticket| ticket.iter().filter(
                |field| ranges.iter().all(
                    |range| !range.is_valid(**field)
                    )
                )
            ).sum()
    }
}

fn main() {
    use std::io::{self, Read};
    use regex::Regex;

    let mut buffer = String::new();

    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re_field = Regex::new(r"(?P<field>[a-z ]+): (?P<l1>\d+)-(?P<u1>\d+) or (?P<l2>\d+)-(?P<u2>\d+)").unwrap();
    let re_ticket = Regex::new(r"(?P<ticket>(\d+,)+\d+)").unwrap();

    let (raw_fields, raw_nearby_tickets) = buffer.split_at(buffer.find("nearby tickets:").unwrap());

    let fieldranges: Vec<day16::FieldRange> = re_field.captures_iter(raw_fields)
        .map(|f| day16::FieldRange::new(
            f.name("l1").unwrap().as_str().parse().unwrap(),
            f.name("u1").unwrap().as_str().parse().unwrap(),
            f.name("l2").unwrap().as_str().parse().unwrap(),
            f.name("u2").unwrap().as_str().parse().unwrap(),
            )
        ).collect();

    let nearby_tickets: Vec<Vec<i32>> = re_ticket.captures_iter(raw_nearby_tickets)
        .map(|t| t.name("ticket").unwrap().as_str().split(",")
             .map(|n| n.parse().unwrap()).collect()
             ).collect();

    println!("Solution A-part: {}", day16::solve_a(&fieldranges, &nearby_tickets));
}

#[cfg(test)]
mod test_day16 {
    use super::*;

    #[test]
    fn test_solve_a() {
        let ranges = vec![
            day16::FieldRange::new(1, 3, 5, 7),
            day16::FieldRange::new(6, 11, 33, 44),
            day16::FieldRange::new(13, 40, 45, 50),
        ];

        let tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        assert_eq!(day16::solve_a(&ranges, &tickets), 71);
    }
}

