extern crate regex;

mod day16 {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct FieldRange {
        name: String,
        l1: i64,
        u1: i64,
        l2: i64,
        u2: i64,
    }

    impl FieldRange {
        pub fn new(name: String, l1: i64, u1: i64, l2: i64, u2: i64) -> Self {
            FieldRange {
                name, l1, u1, l2, u2,
            }
        }

        fn is_valid(&self, x: i64) -> bool {
            self.l1 <= x &&  x <= self.u1
            || self.l2 <= x &&  x <= self.u2
        }
    }

    pub fn solve_a(ranges: &[FieldRange], tickets: &[Vec<i64>]) -> i64 {
        tickets.iter().flat_map(
            |ticket| ticket.iter().filter(
                |&&field| ranges.iter().all(
                    |range| !range.is_valid(field)
                    )
                )
            ).sum()
    }

    fn valid_tickets<'a>(ranges: &'_ [FieldRange], tickets: &'a [Vec<i64>]) -> Vec<&'a Vec<i64>> {
        tickets.iter().filter(
            |ticket| ticket.iter().all(
                |&field| ranges.iter().any(
                    |range| range.is_valid(field)
                    )
                )
            ).collect()
    }

    pub fn solve_b(ranges: &[FieldRange], tickets: &[Vec<i64>])
            -> HashMap<String, usize> {

        let tickets = valid_tickets(ranges, tickets);
        let mut possible_fields: Vec<(&FieldRange, Vec<usize>)> = ranges.iter().map(
            |range| (range, (0..ranges.len()).filter(
                |&i| tickets.iter().all(|ticket| range.is_valid(ticket[i]))
                ).collect::<Vec<usize>>())
            ).collect();


        possible_fields.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        let mut fields: HashMap<String, usize> = HashMap::new();

        for (field, possibilities) in possible_fields.iter() {
            // Assume there is only one possible field after filtering
            let valid_field = possibilities.iter().find(
                |&i| fields.values().find(|&v| v == i).is_none()
                ).unwrap();
            fields.insert(field.name.clone(), *valid_field);
        }

        fields
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
            f.name("field").unwrap().as_str().into(),
            f.name("l1").unwrap().as_str().parse().unwrap(),
            f.name("u1").unwrap().as_str().parse().unwrap(),
            f.name("l2").unwrap().as_str().parse().unwrap(),
            f.name("u2").unwrap().as_str().parse().unwrap(),
            )
        ).collect();

    let nearby_tickets: Vec<Vec<i64>> = re_ticket.captures_iter(raw_nearby_tickets)
        .map(|t| t.name("ticket").unwrap().as_str().split(',')
             .map(|n| n.parse().unwrap()).collect()
             ).collect();

    println!("Solution A-part: {}", day16::solve_a(&fieldranges, &nearby_tickets));

    let fields = day16::solve_b(&fieldranges, &nearby_tickets);

    let departure_fields = vec![
        "departure location",
        "departure station",
        "departure platform",
        "departure track",
        "departure date",
        "departure time",
    ];

    let my_ticket: Vec<i64> = re_ticket.captures(raw_fields).unwrap()
        .name("ticket").unwrap().as_str().split(',')
        .map(|n| n.parse().unwrap()).collect();

    println!("Solution B-part: {}",
             departure_fields.iter()
             .map(|&field| my_ticket[fields[field]]).product::<i64>());
}

#[cfg(test)]
mod test_day16 {
    use super::*;

    #[test]
    fn test_solve_a() {
        let ranges = vec![
            day16::FieldRange::new("class".into(), 1, 3, 5, 7),
            day16::FieldRange::new("row".into(), 6, 11, 33, 44),
            day16::FieldRange::new("seat".into(), 13, 40, 45, 50),
        ];

        let tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        assert_eq!(day16::solve_a(&ranges, &tickets), 71);
    }

    #[test]
    fn test_solve_b() {
        let ranges = vec![
            day16::FieldRange::new("class".into(), 0, 1, 4, 19),
            day16::FieldRange::new("row".into(), 0, 5, 8, 19),
            day16::FieldRange::new("seat".into(), 0, 13, 16, 19),
        ];

        let tickets = vec![
            vec![3, 9, 18],
            vec![15, 1, 5],
            vec![5, 14, 9],
        ];

        let fields = day16::solve_b(&ranges, &tickets);

        assert_eq!(fields["row"], 0);
        assert_eq!(fields["class"], 1);
        assert_eq!(fields["seat"], 2);
    }
}

