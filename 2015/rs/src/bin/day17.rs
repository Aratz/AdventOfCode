mod day17 {
    use std::collections::HashMap;

    fn parse(input: &str) -> Vec<i32> {
        input.lines()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn dp_solve_a(
        i_cont: usize,
        vol: i32,
        containers: &[i32],
        dp_table: &mut HashMap<(usize, i32), usize>) -> usize {
        match dp_table.get(&(i_cont, vol)) {
            Some(&n) => n,
            None => {
                if i_cont == 0 {
                    dp_table.insert(
                        (i_cont, vol),
                        if vol == containers[0] || vol == 0 { 1 } else { 0 });
                }
                else {
                    let with_i = if vol - containers[i_cont] >= 0 {
                        dp_solve_a(i_cont-1, vol - containers[i_cont], containers, dp_table)
                    } else { 0 };
                    let without_i = dp_solve_a(
                        i_cont-1, vol,
                        containers, dp_table);

                    dp_table.insert((i_cont, vol), with_i + without_i);
                }

                dp_table[&(i_cont, vol)]
            }
        }
    }

    pub fn solve_a(input: &str, vol: i32) -> usize {
        let containers = parse(input);
        let mut dp_table = HashMap::new();

        dp_solve_a(containers.len() - 1, vol, &containers, &mut dp_table)
    }

    fn dp_solve_b(
        i_cont: usize,
        vol: i32,
        max_cont: usize,
        containers: &[i32],
        dp_table: &mut HashMap<(usize, i32, usize), usize>) -> usize {
        match dp_table.get(&(i_cont, vol, max_cont)) {
            Some(&n) => n,
            None => {
                if i_cont == 0 {
                    dp_table.insert(
                        (i_cont, vol, max_cont),
                        if (vol == containers[0] && max_cont == 1)
                            || (vol == 0 && max_cont == 0) { 1 } else { 0 });
                }
                else {
                    let with_i = if vol - containers[i_cont] >= 0 && max_cont > 0 {
                        dp_solve_b(
                            i_cont-1,
                            vol - containers[i_cont],
                            max_cont - 1,
                            containers, dp_table)
                    } else { 0 };
                    let without_i = dp_solve_b(
                        i_cont-1, vol, max_cont,
                        containers, dp_table);

                    dp_table.insert((i_cont, vol, max_cont), with_i + without_i);
                }

                dp_table[&(i_cont, vol, max_cont)]
            }
        }
    }

    pub fn solve_b(input: &str, vol: i32) -> usize {
        let containers = parse(input);
        (1..containers.len())
            .map(|max_cont| {
                let mut dp_table = HashMap::new();

                dp_solve_b(containers.len() - 1, vol, max_cont, &containers, &mut dp_table)
            })
            .find(|&x| x > 0).unwrap()
    }

    #[cfg(test)]
    mod test_day17 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "20
15
10
5
5";
            assert_eq!(solve_a(&input, 25), 4);
        }

        #[test]
        fn test_solve_b() {
            let input = "20
15
10
5
5";
            assert_eq!(solve_b(&input, 25), 3);
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

    println!("Solution A-part: {}", day17::solve_a(&buffer.trim(), 150));
    println!("Solution B-part: {}", day17::solve_b(&buffer.trim(), 150));
}
