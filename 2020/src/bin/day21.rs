extern crate regex;

mod day21 {
    use std::collections::{HashMap, HashSet};

    pub fn solve_a(food_list: &Vec<(Vec<String>, Vec<String>)>) -> usize {
        let mut potential_allergens: HashMap<String, HashSet<String>> = HashMap::new();
        let mut unique_ingredients: HashSet<String> = HashSet::new();

        for (ingredients, allergens) in food_list.iter() {
            for allergen in allergens.iter() {
                potential_allergens.entry(allergen.into())
                    .and_modify(
                        |set| {
                            let other = ingredients.clone().into_iter().collect();
                            *set = set.intersection(&other).map(|s| s.into()).collect();
                        })
                    .or_insert(ingredients.clone().into_iter().collect());
                for ingredient in ingredients.iter() {
                    unique_ingredients.insert(ingredient.into());
                }
            }
        }

        let safe_ingredients: HashSet<String> = unique_ingredients.iter().filter(
            |ing| potential_allergens.values().all(
                |allerg_ingrts| !allerg_ingrts.contains(*ing))
            ).map(|s| s.into()).collect();

        food_list.iter().map(
            |(ingredients, _allergens)| ingredients.iter().filter(
                |ing| safe_ingredients.contains(*ing)).count()
            ).sum()
    }

    #[cfg(test)]
    mod test_day21 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let food_list = vec![
                (
                    vec!["mxmxvkd".into(), "kfcds".into(), "sqjhc".into(), "nhms".into()],
                    vec!["dairy".into(), "fish".into()],
                    ),
                (
                    vec!["trh".into(), "fvjkl".into(), "sbzzf".into(), "mxmxvkd".into()],
                    vec!["dairy".into()],
                    ),
                (
                    vec!["sqjhc".into(), "fvjkl".into()],
                    vec!["soy".into()],
                    ),
                (
                    vec!["sqjhc".into(), "mxmxvkd".into(), "sbzzf".into()],
                    vec!["fish".into()],
                    ),
            ];

            assert_eq!(solve_a(&food_list), 5);

        }
    }
}

fn main() {
    use std::io::{self, Read};
    use regex::Regex;

    let re_food = Regex::new(r"(?P<ingredients>(\w+ )*\w+) \(contains (?P<allergens>(\w+, )*\w+)\)").unwrap();

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let food_list = re_food.captures_iter(&buffer).map(
        |capt| (
            capt.name("ingredients").unwrap().as_str().split(" ")
                .map(|s| s.into()).collect(),
            capt.name("allergens").unwrap().as_str().split(", ")
                .map(|s| s.into()).collect()
                )
        ).collect();

    println!("Solution A-part: {}", day21::solve_a(&food_list));
}
