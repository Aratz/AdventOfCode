extern crate regex;

use std::io::{self, Read};
use regex::Regex;

fn validate_passport(passport: &regex::Captures) -> Option<()> {
    let byr = passport.name("byr")?.as_str().parse::<u32>().ok()?;
    if !(1920 <= byr && byr <= 2002) { return None; }

    let iyr = passport.name("iyr")?.as_str().parse::<u32>().ok()?;
    if !(2010 <= iyr && iyr <= 2020) { return None; }

    let eyr = passport.name("eyr")?.as_str().parse::<u32>().ok()?;
    if !(2020 <= eyr && eyr <= 2030) { return None; }

    let hgt = passport.name("hgt")?.as_str().parse::<u32>().ok()?;
    let unt = passport.name("unt")?.as_str();
    if !((unt == "cm" && 150 <= hgt && hgt <= 193)
         || (unt == "in" && 59 <= hgt && hgt <= 76)) { return None; }

    passport.name("hcl")?;

    passport.name("ecl")?;

    passport.name("pid")?;

    return Some(());
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re = Regex::new(r"(?:ecl:(?P<ecl>amb|blu|brn|gry|grn|hzl|oth)\s|pid:(?P<pid>\d{9})\s|eyr:(?P<eyr>\d{4})\s|hcl:(?P<hcl>#[0-9a-f]{6})\s|byr:(?P<byr>\d{4})\s|iyr:(?P<iyr>\d{4})\s|cid:(?P<cid>\d+)\s|hgt:(?P<hgt>\d+)(?P<unt>in|cm)\s)+").unwrap();

    let res = re.captures_iter(&buffer)
        .filter_map(|passport| validate_passport(&passport)).count();

    println!("{}", res);
}
