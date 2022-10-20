extern crate regex;
mod day15 {
    use regex::Regex;
    use std::cmp::{min, max};
    use std::ops::{Add, Mul};
    use std::collections::HashMap;

    #[derive(Copy, Clone)]
    struct Ingredient {
        cap: i64,
        dur: i64,
        fla: i64,
        tex: i64,
        cal: i64,
    }

    impl Add for Ingredient {
        type Output = Self;

        #[inline]
        fn add(self, other: Self) -> Self {
            Self {
                cap: self.cap + other.cap,
                dur: self.dur + other.dur,
                fla: self.fla + other.fla,
                tex: self.tex + other.tex,
                cal: self.cal + other.cal,
            }
        }
    }

    impl Mul<i64> for Ingredient {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: i64) -> Self {
            Self {
                cap: self.cap * rhs,
                dur: self.dur * rhs,
                fla: self.fla * rhs,
                tex: self.tex * rhs,
                cal: self.cal * rhs,
            }
        }
    }

    impl Ingredient {
        #[inline]
        fn score(&self) -> i64 {
            max(0, self.cap)
                * max(0, self.dur)
                * max(0, self.fla)
                * max(0, self.tex)
        }
    }

    fn parse(input: &str) -> HashMap<String, Ingredient> {
        let re_ing = Regex::new(r"(?P<name>\w+): capacity (?P<cap>-?\d+), durability (?P<dur>-?\d+), flavor (?P<fla>-?\d+), texture (?P<tex>-?\d+), calories (?P<cal>-?\d+)").unwrap();

        re_ing.captures_iter(input)
            .map(|capt| (
                    capt["name"].to_string(),
                    Ingredient {
                        cap: capt["cap"].parse().unwrap(),
                        dur: capt["dur"].parse().unwrap(),
                        fla: capt["fla"].parse().unwrap(),
                        tex: capt["tex"].parse().unwrap(),
                        cal: capt["cal"].parse().unwrap(),
                    }))
            .collect()
    }

    pub fn solve_a(input: &str) -> i64 {
        let ingredients = &parse(input);
        (1..=97).flat_map(
            move |spr| (1..=97).flat_map(
                move |pea| (
                    1..min(97, ingredients["PeanutButter"].dur*pea
                    + ingredients["Sprinkles"].dur*spr)).filter_map(
                        move |fro| {
                            let sug = 100 - spr - pea - fro;
                            if sug < 0
                                || sug >= ingredients["PeanutButter"].cap*pea
                                            + ingredients["Sprinkles"].cap*spr {
                                None
                            } else {
                                Some((ingredients["Sprinkles"]*spr
                                    + ingredients["PeanutButter"]*pea
                                    + ingredients["Frosting"]*fro
                                    + ingredients["Sugar"]*sug).score())
                            }})))
            .max().unwrap()
    }

    pub fn solve_b(input: &str) -> i64 {
        let ingredients = &parse(input);
        (1..=97).flat_map(
            move |spr| (1..=97).flat_map(
                move |pea| (
                    1..min(97, ingredients["PeanutButter"].dur*pea
                    + ingredients["Sprinkles"].dur*spr)).filter_map(
                        move |fro| {
                            let sug = 100 - spr - pea - fro;
                            if sug < 0
                                || sug >= ingredients["PeanutButter"].cap*pea
                                            + ingredients["Sprinkles"].cap*spr {
                                None
                            } else {
                                let cookie = ingredients["Sprinkles"]*spr
                                    + ingredients["PeanutButter"]*pea
                                    + ingredients["Frosting"]*fro
                                    + ingredients["Sugar"]*sug;
                                if cookie.cal != 500 { None }
                                else { Some(cookie.score()) }
                            }})))
            .max().unwrap()
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

    println!("Solution A-part: {}", day15::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day15::solve_b(&buffer.trim()));
}
