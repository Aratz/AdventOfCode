mod day16 {
    fn expand(data: &[bool], max_len: usize) -> Vec<bool> {
        let mut a = data.to_vec();

        while a.len() < max_len {
            let mut b: Vec<bool> = a.iter().map(|c| !c).collect();
            b.reverse();

            a.push(false);
            a.append(&mut b);
        }

        a.truncate(max_len);

        a
    }

    fn get_checksum(data: &[bool]) -> Vec<bool> {
        if data.len() % 2 == 0 {
            get_checksum(
                &data.iter().zip(data.iter().skip(1))
                .step_by(2)
                .map(|(c1, c2)| c1 == c2)
                .collect::<Vec<bool>>())
        }
        else {
            data.to_vec()
        }
    }

    pub fn solve_ab(raw_data: &str, max_len: usize) -> String {
        let data: Vec<bool> = raw_data.chars().map(|c| c == '1').collect();
        let checksum = get_checksum(&expand(&data, max_len));

        checksum.iter().map(|&c| if c { '1' } else { '0' }).collect()
    }

    #[cfg(test)]
    mod test_day16 {
        use super::*;

        #[test]
        fn test_expand() {
            assert_eq!(expand(&vec![true], 3), vec![true, false, false]);
            assert_eq!(expand(&vec![false], 3), vec![false, false, true]);
            assert_eq!(expand(&vec![true, true, true, true, true], 11), vec![true, true, true, true, true, false, false, false, false, false, false]);
        }

        #[test]
        fn test_checksum() {
            assert_eq!(get_checksum(
                    &vec![true, true, false, false, true, false, true, true, false, true, false, false]),
                    vec![true, false, false]);
        }

        #[test]
        fn test_solve_ab() {
            assert_eq!(solve_ab("10000", 20), "01100");
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

    println!("Solution A-part: {}", day16::solve_ab(&buffer.trim(), 272));
    println!("Solution B-part: {}", day16::solve_ab(&buffer.trim(), 35_651_584));
}
