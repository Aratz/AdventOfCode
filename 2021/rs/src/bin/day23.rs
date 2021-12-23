mod day23 {
    use std::collections::{HashMap, BinaryHeap};
    use std::cmp::Reverse;

    const HSIZE: usize = 7;
    const RSIZE: usize = 4;
    const R_N: usize = 4;

    #[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
    struct State {
        hallway: [Option<usize>; HSIZE],
        rooms: [[Option<usize>; RSIZE]; R_N],
        r_size: usize,
    }

    impl State {
        fn is_final(&self) -> bool {
            self.hallway.iter().all(|&v| v.is_none())
            && self.rooms.iter().enumerate()
                .all(|(i, r)| r[..self.r_size].iter().all(|&v| v == Some(i)))
        }

        fn is_valid(&self) -> bool {
            self.hallway.iter().filter_map(|&v| v).map(|v| v + 1).sum::<usize>()
                + self.rooms.iter().map(|r| r.iter().filter_map(|&v| v).map(|v| v + 1).sum::<usize>()).sum::<usize>()
                == 10*self.r_size
        }

        fn find_free_spot(&self, room: usize) -> Option<usize> {
            if self.rooms[room].iter().filter_map(|&v| v).all(|v| v == room) {
                (0..self.r_size)
                    .map(|i| self.r_size - 1 - i)
                    .find(|&i| self.rooms[room][i].is_none())
            }
            else {
                None
            }
        }
    }

    fn parse(input: &str, ab: bool) -> State {
        let mut lines = input.lines().skip(2);
        let r1: Vec<usize> = lines.next().unwrap().chars()
            .filter(|&c| c != '#' && c != ' ')
            .map(|c| ((c as u8) - ('A' as u8)) as usize)
            .collect();

        let r2: Vec<usize> = lines.next().unwrap().chars()
            .filter(|&c| c != '#' && c != ' ')
            .map(|c| ((c as u8) - ('A' as u8)) as usize)
            .collect();

        let mut state = State {
            hallway: [None; HSIZE],
            rooms: [[None; RSIZE]; R_N],
            r_size: if !ab { 2 } else { 4 },
        };

        for (i, i_move) in r1.into_iter().enumerate() {
            state.rooms[i][0] = Some(i_move);
        }

        for (i, i_move) in r2.into_iter().enumerate() {
            state.rooms[i][if !ab { 1 } else { 3 }] = Some(i_move);
        }

        if ab {
            state.rooms[0][1] = Some(3);
            state.rooms[0][2] = Some(3);

            state.rooms[1][1] = Some(2);
            state.rooms[1][2] = Some(1);

            state.rooms[2][1] = Some(1);
            state.rooms[2][2] = Some(0);

            state.rooms[3][1] = Some(0);
            state.rooms[3][2] = Some(2);
        }

        state
    }

    fn pathfinder(start: &State, r_size: usize) -> usize {
        let costs = vec![1, 10, 100, 1000];

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), start.clone()));

        let mut visited = HashMap::new();

        while let Some((Reverse(d), state)) = queue.pop() {
            if visited.contains_key(&state) { continue; }

            visited.insert(state.clone(), d);

            if state.is_final() {
                return d;
            }

            for i_r in 0..R_N {
                let mut partial_state = state.clone();
                let mut cost = 2;
                let mut i_move = None;
                for i_dpt in 0..r_size {
                    if let Some(i) = state.rooms[i_r][i_dpt] {
                        if state.rooms[i_r][i_dpt..r_size].iter().any(|&v| v != Some(i_r)) {
                            partial_state.rooms[i_r][i_dpt] = None;
                            i_move = Some(i);
                        }
                        break;
                    }
                    cost += 1;
                }

                if let Some(i_move) = i_move {
                    let mut ihleft = i_r + 1;
                    let mut cost_hway = 0;
                    while state.hallway[ihleft].is_none() {
                        let mut new_state = partial_state.clone();

                        new_state.hallway[ihleft] = Some(i_move);
                        assert!(new_state.is_valid());
                        queue.push((Reverse(d + (cost + cost_hway)*costs[i_move]), new_state));

                        if ihleft == 0 { break; }
                        cost_hway += 2; ihleft -= 1;
                        if ihleft == 0 { cost_hway -= 1; }
                    }

                    let mut ihright = i_r + 2;
                    let mut cost_hway = 0;
                    while ihright < HSIZE && state.hallway[ihright].is_none() {
                        let mut new_state = partial_state.clone();

                        new_state.hallway[ihright] = Some(i_move);
                        assert!(new_state.is_valid());
                        queue.push((Reverse(d + (cost + cost_hway)*costs[i_move]), new_state));

                        cost_hway += 2; ihright += 1;
                        if ihright == HSIZE - 1 { cost_hway -= 1; }
                    }
                }
            }

            for i_h in 0..HSIZE {
                if let Some(i_move) = state.hallway[i_h] {
                    let mut new_state = state.clone();
                    new_state.hallway[i_h] = None;
                    //req: hallway free down to room & room is empty or has correct occupant
                    if (i_h >= i_move + 2 && state.hallway[i_move + 2..i_h].iter().all(|&v| v.is_none()))
                        || (i_h <= i_move + 1 && state.hallway[i_h + 1..=i_move + 1].iter().all(|&v| v.is_none())) {
                        let mut cost = 2 + 2*(if i_h >= i_move + 2 { i_h - (i_move + 2) } else { i_move + 1 - i_h });
                        if i_h == 0 || i_h == HSIZE - 1 { cost -= 1; }

                        //find first free spot in room
                        if let Some(i_dpt) = state.find_free_spot(i_move) {
                            new_state.rooms[i_move][i_dpt] = Some(i_move);
                            assert!(new_state.is_valid());
                            queue.push((Reverse(d + (cost + i_dpt)*costs[i_move]), new_state));
                        }
                    }
                }
            }
        }

        unreachable!();
    }

    pub fn solve_a(input: &str) -> usize {
        let state = parse(input, false);

        pathfinder(&state, 2)
    }

    pub fn solve_b(input: &str) -> usize {
        let state = parse(input, true);

        pathfinder(&state, 4)
    }

    #[cfg(test)]
    mod test_day23 {
        use super::*;

        #[test]
        fn test_final() {
            let state = State {
                hallway: [None; 7],
                rooms: [
                    [Some(0), Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [Some(3), Some(3), None, None],
                ],
                r_size: 2,
            };
            assert!(state.is_final());
        }

        #[test]
        fn test_valid() {
            let state = State {
                hallway: [None; 7],
                rooms: [
                    [None, Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [Some(3), Some(3), None, None],
                ],
                r_size: 2,
            };
            assert!(!state.is_valid());

            let state = State {
                hallway: [None, None, None, None, None, Some(0), None],
                rooms: [
                    [None, Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [Some(3), Some(3), None, None],
                ],
                r_size: 2,
            };
            assert!(state.is_valid());
        }

        #[test]
        fn test_find_free() {
            let state = State {
                hallway: [None, None, None, None, None, Some(0), None],
                rooms: [
                    [None, Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [Some(3), Some(3), None, None],
                ],
                r_size: 2,
            };

            assert_eq!(state.find_free_spot(0), Some(0));
        }

        #[test]
        fn test_pathfinder() {
            let start = State {
                hallway: [None; 7],
                rooms: [
                    [Some(0), Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [Some(3), Some(3), None, None],
                ],
                r_size: 2,
            };
            assert_eq!(pathfinder(&start, 2), 0);

            let start = State {
                hallway: [None, None, None, None, None, Some(0), None],
                rooms: [
                    [None, Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [Some(3), Some(3), None, None],
                ],
                r_size: 2,
            };
            assert_eq!(pathfinder(&start, 2), 8);

            let start = State {
                hallway: [None, None, None, Some(3), None, Some(0), None],
                rooms: [
                    [None, Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [None, Some(3), None, None],
                ],
                r_size: 2
            };
            //assert_eq!(pathfinder(&start, 2), 8 + 4000);

            let start = State {
                hallway: [None, None, None, Some(3), Some(3), Some(0), None],
                rooms: [
                    [None, Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [None, None, None, None],
                ],
                r_size: 2,
            };
            //assert_eq!(pathfinder(&start, 2), 8 + 7000);

            let start = State {
                hallway: [None, None, None, Some(3), None, None, None],
                rooms: [
                    [None, Some(0), None, None],
                    [Some(1), Some(1), None, None],
                    [Some(2), Some(2), None, None],
                    [Some(3), Some(0), None, None],
                ],
                r_size: 2,
            };
            //assert_eq!(pathfinder(&start, 2), 8 + 7000 + 2003);
        }

        #[test]
        fn test_solve_a() {
            let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

            //assert_eq!(solve_a(&input), 12521);
        }

        #[test]
        fn test_solve_b() {
            let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

            //assert_eq!(solve_a(&input), 44169);
        }
    }
}

fn main() {
    use std::io::{self, Read};

    let stdin = io::stdin();

    let mut buffer= String::new();

    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    println!("Solution A-part: {}", day23::solve_a(&buffer));
    println!("Solution B-part: {}", day23::solve_b(&buffer));
}
