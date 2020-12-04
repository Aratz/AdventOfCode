extern crate regex;

use std::io::{self, Read};
use regex::Regex;

fn main() {
    let mandatory_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re = Regex::new(r"(?:ecl:(?P<ecl>[a-z]{3}|#[a-f0-9]+)\s|pid:(?P<pid>#?\w+)\s|eyr:(?P<eyr>\d+)\s|hcl:(?P<hcl>#?\w+)\s|byr:(?P<byr>\d+)\s|iyr:(?P<iyr>\d+)\s|cid:(?P<cid>\d+)\s|hgt:(?P<hgt>\d+(?:in|cm)?\s))+").unwrap();

    let res = re.captures_iter(&buffer)
        .filter(|passport| mandatory_fields.iter()
            .filter_map(|field| passport.name(field)).count() == mandatory_fields.len())
        .count();

    println!("{}", res);
}
