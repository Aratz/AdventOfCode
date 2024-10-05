mod day15 {
    use std::collections::HashMap;

    #[derive(Clone)]
    struct Lens {
        label: String,
        strength: usize,
    }

    #[derive(Clone)]
    struct LensBox {
        lenses: HashMap<String, (usize, Lens)>,

    }

    impl LensBox {
        fn new() -> Self {
            Self { lenses: HashMap::new() }
        }
        fn insert(&mut self, lens: Lens) {
            let max_rk = self.lenses.values()
                .map(|(rk, _)| *rk)
                .max().unwrap_or(0);
            self.lenses.entry(lens.label.to_string())
                .and_modify(|e| *e = (e.0, lens.clone()))
                .or_insert((max_rk + 1, lens));
        }

        fn remove(&mut self, label: &str) {
            self.lenses.remove(label);
        }
    }

    fn parse(input: &str) -> Vec<String> {
        input.split(",").map(|seq| seq.to_string()).collect()
    }

    fn hash(seq: &str) -> usize {
        seq.chars()
            .fold(0, |acc, c| (acc + (c as usize))*17%256)
    }

    pub fn solve_a(input: &str) -> usize {
        parse(input).iter()
            .map(|seq| hash(seq))
            .sum::<usize>()
    }

    pub fn solve_b(input: &str) -> usize {
        let mut boxes = vec![LensBox::new(); 256];

        let insts = parse(input);

        for inst in insts {
            let split_inst = inst.split('=').collect::<Vec<_>>();
            match split_inst.len() {
                2 => {
                    let label = split_inst[0].to_string();
                    let strength = split_inst[1].parse().unwrap();

                    let lens_box = &mut boxes[hash(&label)];
                    let lens = Lens { label, strength };

                    lens_box.insert(lens);
                },
                1 => {
                    let label_len = split_inst[0].len();
                    let label = split_inst[0][0..label_len-1].to_string();
                    let lens_box = &mut boxes[hash(&label)];
                    lens_box.remove(&label);
                },
                _ => unreachable!(),
            }
        }

        boxes.into_iter().enumerate()
            .flat_map(|(i, lens_box)| {
                let mut lenses = lens_box.lenses.values()
                    .map(|e| e.clone())
                    .collect::<Vec<_>>();
                lenses.sort_unstable_by_key(|(pos, _)| *pos);
                lenses.into_iter().enumerate()
                    .map(move |(pos, (_, lens))| (i+1) * (pos+1) * lens.strength)
            })
            .sum::<usize>()
    }

    #[cfg(test)]
    mod test_day15 {
        use super::*;

        static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 1320);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 145);
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

    println!("Solution A-part: {}", day15::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day15::solve_b(&buffer.trim()));
}
