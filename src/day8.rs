use std::collections::HashMap;
use std::io::{self, BufRead};

fn main(){
    let mut registers:HashMap<String, i32> = HashMap::new();

    let stdin = io::stdin();
    for raw_line in stdin.lock().lines() {
        let line = raw_line.unwrap();
        let line = line.split(" if ").collect::<Vec<&str>>();

        let instruction = line[0].split(" ").collect::<Vec<&str>>();
        let condition = line[1].split(" ").collect::<Vec<&str>>();

        let register = condition[0];
        let symbol = condition[1];
        let value = condition[2].parse::<i32>().unwrap();

        let register_value = *registers.entry(String::from(register)).or_insert(0);

        if match symbol {
            "<" => register_value < value,
            ">" => register_value > value,
            "==" => register_value == value,
            "!=" => register_value != value,
            "<=" => register_value <= value,
            ">=" => register_value >= value,
            _ => true,
        } {
            let register = instruction[0];
            let direction = instruction[1];
            let value = instruction[2].parse::<i32>().unwrap();
            let register_value = registers.entry(String::from(register)).or_insert(0);
            *register_value += {match direction { "dec" => -1, _ => 1 }} * value;
        }
    }
    println!("{:?}", registers.values().max());
}
