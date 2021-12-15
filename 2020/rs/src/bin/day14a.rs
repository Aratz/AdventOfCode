extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut and_mask = u64::MAX; // Set to 0
    let mut or_mask = 0; // Set to 1

    let re_mask = Regex::new(r"^mask = ([X01]{36})$").unwrap();
    let re_mem= Regex::new(r"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let instruction = line.unwrap().to_string();
        if let Some(mask_match) = re_mask.captures(&instruction) {
            let mask = mask_match.get(1).unwrap().as_str();
            and_mask = u64::MAX;
            or_mask = 0;

            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    '1' => { or_mask += 1<<i; },
                    '0' => { and_mask -= 1<<i; },
                    'X' => {},
                    _ => { panic!("Mask parsing error!"); },
                }
            }
        }
        else if let Some(mem_match) = re_mem.captures(&instruction) {
            let address:u64 = mem_match.name("address").unwrap().as_str().parse().unwrap();
            let value:u64 = mem_match.name("value").unwrap().as_str().parse().unwrap();

            memory.insert(address, (value & and_mask) | or_mask);
        }
        else {
            panic!("Unrecognized instruction! ({})", instruction);
        }
    }

    println!("{}", memory.values().sum::<u64>());
}
