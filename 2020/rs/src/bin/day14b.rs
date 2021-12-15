extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut or_mask = 0; // Set to 1

    let re_mask = Regex::new(r"^mask = ([X01]{36})$").unwrap();
    let re_mem = Regex::new(r"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();
    let mut floating_bits: Vec<usize> = Vec::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let instruction = line.unwrap().to_string();
        if let Some(mask_match) = re_mask.captures(&instruction) {
            let mask = mask_match.get(1).unwrap().as_str();
            or_mask = 0;
            floating_bits.clear();

            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    '1' => { or_mask += 1<<i; },
                    '0' => {},
                    'X' => { floating_bits.push(i); },
                    _ => { panic!("Mask parsing error!"); },
                }
            }
        }
        else if let Some(mem_match) = re_mem.captures(&instruction) {
            let address:u64 = mem_match.name("address").unwrap().as_str().parse::<u64>().unwrap() | or_mask;
            let value:u64 = mem_match.name("value").unwrap().as_str().parse().unwrap();

            for combination in 0..2u64.pow(floating_bits.len() as u32) {
                let mut floating_address: u64 = address;
                for (i, bit_index) in floating_bits.iter().enumerate() {
                    if combination & (1<<i) != 0 {
                         floating_address |= 1<<bit_index;
                    }
                    else {
                        floating_address &= !(1<<bit_index);
                    };
                }
                memory.insert(floating_address, value);
            }
        }
        else {
            panic!("Unrecognized instruction! ({})", instruction);
        }
    }

    println!("{}", memory.values().sum::<u64>());
}
