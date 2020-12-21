extern crate regex;

mod day21 {
    use std::collections::{HashMap, HashSet};

    pub fn solve_ab(food_list: &Vec<(Vec<String>, Vec<String>)>)
            -> (usize, String) {
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

        let mut allergen_map: HashMap<String, String> = HashMap::new();
        let mut identified_ingredients: HashSet<String> = HashSet::new();
        let mut allergens: Vec<String> = potential_allergens.keys().map(|s| s.into()).collect();
        allergens.sort_by_key(|k| (potential_allergens[k].len()));
        allergens.reverse();

        while let Some(allergen) = allergens.pop() {
            let ing = potential_allergens[&allergen].iter().filter(
                    |ing| !identified_ingredients.contains(*ing))
                .next().unwrap();
            allergen_map.insert(
                allergen.into(),
                ing.into(),
                );
            identified_ingredients.insert(ing.into());
            allergens.sort_by_key(|k| potential_allergens[k].iter()
                                  .filter(|ing| !identified_ingredients.contains(*ing))
                                  .count());
            allergens.reverse();
        }

        let mut allergens: Vec<String> = potential_allergens.keys().map(|s| s.into()).collect();
        allergens.sort();
        let dangerous_ingredients: Vec<String> = allergens.iter().map(
            |alln| allergen_map[alln].clone()).collect();

        let safe_ingredients: HashSet<String> = unique_ingredients.iter().filter(
            |ing| potential_allergens.values().all(
                |allerg_ingrts| !allerg_ingrts.contains(*ing))
            ).map(|s| s.into()).collect();

        (
            food_list.iter().map(
                |(ingredients, _allergens)| ingredients.iter().filter(
                    |ing| safe_ingredients.contains(*ing)).count()
                ).sum(),
            dangerous_ingredients.join(","),
            )
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

            assert_eq!(solve_ab(&food_list).0, 5);

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

    let (sol_a, sol_b) = day21::solve_ab(&food_list);
    println!("Solution A-part: {}", sol_a);
    println!("Solution B-part: {}", sol_b);
}
