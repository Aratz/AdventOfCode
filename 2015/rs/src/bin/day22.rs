mod day22 {
    use std::cmp::{max, Reverse};
    use std::collections::{BinaryHeap, HashSet};

    #[derive(Debug, Hash, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    struct State {
        me: (i32, i32, i32),
        boss: (i32, i32),
        effects: (i32, i32, i32),
    }

    fn parse(input: &str) -> State {
        let mut lines = input.lines();
        let hp = lines.next().unwrap()
            .split(": ").skip(1).next().unwrap()
            .parse().unwrap();
        let dmg = lines.next().unwrap()
            .split(": ").skip(1).next().unwrap()
            .parse().unwrap();

        State {
            me: (50, 500, 0),
            boss: (hp, dmg),
            effects: (0, 0, 0),
        }
    }

    fn apply_effects(state: &mut State) -> bool {
        state.me.2 = 0;

        if state.effects.0 > 0 {
            state.effects.0 -= 1;
            state.me.2 = 7;
        }
        if state.effects.1 > 0{
            state.effects.1 -= 1;
            state.boss.0 -= 3;
        }
        if state.effects.2 > 0{
            state.effects.2 -= 1;
            state.me.1 += 101;
        }

         state.boss.0 <= 0
    }

    fn apply_boss(state: &mut State) -> bool {
        if apply_effects(state) { return true; }

        state.me.0 -= max(1, state.boss.1 - state.me.2);

        state.me.0 > 0
    }

    pub fn solve_ab(input: &str, hard: bool) -> i32 {
        let init = parse(input);

        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        //queue.push((Reverse(0 + init.boss.0), Reverse(0), init));
        queue.push((Reverse(0), init));
        visited.insert(init);

        //while let Some((_, Reverse(mana_spent), mut state)) = queue.pop() {
        while let Some((Reverse(mana_spent), mut state)) = queue.pop() {
            if hard {
                state.me.0 -= 1;
                if state.me.0 <= 0 { continue; }
            }
            if apply_effects(&mut state) { return mana_spent; }

            let cost = 53;
            if state.me.1 >= cost {
                let (mut mana_spent, mut state) = (mana_spent, state);
                // Magic Missile
                mana_spent += cost;
                state.me.1 -= cost;
                state.boss.0 -= 4;

                if apply_boss(&mut state) && !visited.contains(&state) {
                    queue.push((
                            //Reverse(mana_spent + state.boss.0),
                            Reverse(mana_spent),
                            state));
                }
            }

            let cost = 73;
            if state.me.1 >= cost {
                let (mut mana_spent, mut state) = (mana_spent, state);
                // Drain
                let cost = 73;
                mana_spent += cost;
                state.me.1 -= cost;
                state.boss.0 -= 2;
                state.me.0 += 2;

                if apply_boss(&mut state) && !visited.contains(&state) {
                    queue.push((
                            //Reverse(mana_spent + state.boss.0),
                            Reverse(mana_spent),
                            state));
                }
            }

            let cost = 113;
            if state.me.1 >= cost && state.effects.0 == 0 {
                let (mut mana_spent, mut state) = (mana_spent, state);
                // Shield
                mana_spent += cost;
                state.me.1 -= cost;
                state.effects.0 = 6;

                if apply_boss(&mut state) && !visited.contains(&state) {
                    queue.push((
                            //Reverse(mana_spent + state.boss.0),
                            Reverse(mana_spent),
                            state));
                }
            }

            let cost = 173;
            if state.me.1 >= cost && state.effects.1 == 0 {
                let (mut mana_spent, mut state) = (mana_spent, state);
                // Poison
                mana_spent += cost;
                state.me.1 -= cost;
                state.effects.1 = 6;

                if apply_boss(&mut state) && !visited.contains(&state) {
                    queue.push((
                            //Reverse(mana_spent + state.boss.0),
                            Reverse(mana_spent),
                            state));
                }
            }

            let cost = 229;
            if state.me.1 >= cost && state.effects.2 == 0 {
                let (mut mana_spent, mut state) = (mana_spent, state);
                // Recharge
                mana_spent += cost;
                state.me.1 -= cost;
                state.effects.2 = 5;

                if apply_boss(&mut state) && !visited.contains(&state) {
                    queue.push((
                            //Reverse(mana_spent + state.boss.0),
                            Reverse(mana_spent),
                            state));
                }
            }
        }

        unreachable!()
    }

    #[cfg(test)]
    mod test_day22 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "Hit Points: 4
Damage: 9";

            assert_eq!(solve_ab(&input, false), 53);
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

    println!("Solution A-part: {}", day22::solve_ab(&buffer.trim(), false));
    println!("Solution B-part: {}", day22::solve_ab(&buffer.trim(), true));
}
