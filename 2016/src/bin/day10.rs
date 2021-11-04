extern crate regex;

mod day10 {
    use std::collections::HashMap;

    pub fn solve_a(lines: &Vec<String>) -> Result<i32, &'static str> {
        let mut network: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut bots: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut values: Vec<i32> = Vec::new();
        let mut outputs: HashMap<i32, Vec<i32>> = HashMap::new();

        for inst in lines {
            let words: Vec<_> = inst.split(" ").collect();
            if words[0] == "value" {
                let val = words[1].parse().unwrap();
                let bot = words[5].parse().unwrap();
                values.push(val);
                bots.entry(bot).or_default().push(val);
            }
            else {
                let bot: i32 = words[1].parse().unwrap();
                let low: i32 = match words[5] {
                    "bot" => words[6].parse().unwrap(),
                    "output" => -1 - words[6].parse::<i32>().unwrap(),
                    _ => unreachable!(),
                };
                let high: i32 = match words[10] {
                    "bot" => words[11].parse().unwrap(),
                    "output" => -1 - words[11].parse::<i32>().unwrap(),
                    _ => unreachable!(),
                };

                network.insert(bot, (low, high));
            }
        }

        while values.len() != outputs.values().map(|v| v.len()).sum() {
            let ready_bots: Vec<_> = bots.iter().filter(|(_bot, chips)| chips.len() == 2)
                .map(|(bot, chips)| (*bot, chips.clone()))
                .collect();

            for (bot, chips) in ready_bots {
                let low = *chips.iter().min().unwrap();
                let high = *chips.iter().max().unwrap();
                if network[&bot].0 >= 0 {
                    bots.entry(network[&bot].0).or_default().push(low);
                }
                else {
                    outputs.entry(-(1 + network[&bot].0)).or_default().push(low);
                }

                if network[&bot].1 >= 0 {
                    bots.entry(network[&bot].1).or_default().push(high);
                }
                else {
                    outputs.entry(-(1 + network[&bot].1)).or_default().push(high);
                }
                bots.remove(&bot);

                if low == 17 && high == 61 {
                    return Ok(bot);
                }
            }
        }

        Err("No bot handled V17 and V61")
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<String> = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect();

    println!("Solution A-part: {}", day10::solve_a(&lines).unwrap());

}
