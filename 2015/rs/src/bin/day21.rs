mod day21 {
    use std::cmp::max;

    fn parse(input: &str) -> (i32, i32, i32) {
        let mut lines = input.lines();
        let hp = lines.next().unwrap()
            .split(": ").skip(1).next().unwrap()
            .parse().unwrap();
        let dmg = lines.next().unwrap()
            .split(": ").skip(1).next().unwrap()
            .parse().unwrap();
        let arm = lines.next().unwrap()
            .split(": ").skip(1).next().unwrap()
            .parse().unwrap();

        (hp, dmg, arm)
    }

    fn get_weapons() -> Vec<(i32, i32, i32)> {
        vec![
            (8, 4, 0),
            (10, 5, 0),
            (25, 6, 0),
            (40, 7, 0),
            (74, 8, 0),
        ]
    }

    fn get_armor() -> Vec<(i32, i32, i32)> {
        vec![
            (0, 0, 0),
            (13, 0, 1),
            (31, 0, 2),
            (53, 0, 3),
            (75, 0, 4),
            (102, 0, 5),
        ]
    }

    fn get_rings() -> Vec<(i32, i32, i32)> {
        vec![
            (0, 0, 0),
            (0, 0, 0),
            (25, 1, 0),
            (50, 2, 0),
            (100, 3, 0),
            (20, 0, 1),
            (40, 0, 2),
            (80, 0, 3),
        ]
    }

    fn sim_fight(mut me: (i32, i32, i32), mut boss: (i32, i32, i32)) -> bool {
        loop {
            boss.0 -= max(1, me.1 - boss.2);
            if boss.0 <= 0 { return true; }

            me.0 -= max(1, boss.1 - me.2);
            if me.0 <= 0 { return false; }
        }
    }


    pub fn solve_a(input: &str) -> i32 {
        let boss = parse(input);
        let weapons = &get_weapons();
        let armor = &get_armor();
        let rings = &get_rings();

        weapons.iter()
            .flat_map(|&wpn| armor.iter()
                 .flat_map(move |&arm| rings.iter()
                      .flat_map(move |&rng1| rings.iter()
                           .map(move |&rng2| (wpn, arm, rng1, rng2)))))
            .filter(|&(wpn, arm_pc, rng1, rng2)| {
                let dmg = wpn.1 + arm_pc.1 + rng1.1 + rng2.1;
                let arm = wpn.2 + arm_pc.2 + rng1.2 + rng2.2;
                sim_fight((100, dmg, arm), boss)
            })
            .map(|(wpn, arm_pc, rng1, rng2)| wpn.0 + arm_pc.0 + rng1.0 + rng2.0)
            .min().unwrap()
    }

    pub fn solve_b(input: &str) -> i32 {
        let boss = parse(input);
        let weapons = &get_weapons();
        let armor = &get_armor();
        let rings = &get_rings();

        weapons.iter()
            .flat_map(|&wpn| armor.iter()
                 .flat_map(move |&arm| rings.iter()
                      .flat_map(move |&rng1| rings.iter()
                           .map(move |&rng2| (wpn, arm, rng1, rng2)))))
            .filter(|&(wpn, arm_pc, rng1, rng2)| {
                let dmg = wpn.1 + arm_pc.1 + rng1.1 + rng2.1;
                let arm = wpn.2 + arm_pc.2 + rng1.2 + rng2.2;
                !sim_fight((100, dmg, arm), boss)
            })
            .map(|(wpn, arm_pc, rng1, rng2)| wpn.0 + arm_pc.0 + rng1.0 + rng2.0)
            .max().unwrap()
    }

    #[cfg(test)]
    mod test_day21 {
        use super::*;

        #[test]
        fn test_sim_fight() {
            assert!(sim_fight((8, 5, 5), (12, 7, 2)));
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

    println!("Solution A-part: {}", day21::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day21::solve_b(&buffer.trim()));
}
