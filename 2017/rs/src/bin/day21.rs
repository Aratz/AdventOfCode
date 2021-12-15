mod day21 {
    struct Rule {
        pattern: Chunk,
        output: Chunk,
    }

    impl Rule {
        fn new(raw_rule: &str) -> Self {
            let arms: Vec<_> = raw_rule.split(" => ").collect();

            let pattern = to_chunk(arms[0]);
            let output = to_chunk(arms[1]);

            Rule { pattern, output }
        }

        fn is_match(&self, chunk: &Chunk) -> bool {
            let mut trans = vec![chunk.clone()];
            for _ in 0..3 {
                trans.push(rot90(&trans[trans.len() - 1]));
            }

            trans.push(flip(&trans[0]));

            for _ in 0..3 {
                trans.push(rot90(&trans[trans.len() - 1]));
            }

            trans.iter().any(|c| c == &self.pattern)
        }
    }


    type Chunk = Vec<Vec<char>>;

    fn to_chunk(s: &str) -> Chunk {
        s.split('/')
            .map(|row| row.chars().collect())
            .collect()
    }

    fn rot90(chunk: &Chunk) -> Chunk {
        let size = chunk.len();
        let mut new_chunk = chunk.clone();

        for i in 0..size {
            for j in 0..size {
                new_chunk[i][j] = chunk[size - 1 - j][i]
            }
        }

        new_chunk
    }

    fn flip(chunk: &Chunk) -> Chunk {
        let size = chunk.len();
        let mut chunk = chunk.clone();

        for i in 0..size {
            for j in 0..size/2 {
                chunk[i].swap(j, size - 1 - j)
            }
        }

        chunk
    }

    fn enhance(chunk: &mut Chunk, rule: &Rule) {
        *chunk = rule.output.clone();
    }

    fn parse_rules(raw_rules: &[String]) -> Vec<Rule> {
        raw_rules.iter().map(|r| Rule::new(r)).collect()
    }

    fn split_grid(grid: &[Vec<char>]) -> Vec<Vec<Chunk>> {
        let div = if grid.len() % 2 == 0  { 2 } else { 3 };

        (0..grid.len()/div)
            .map(|i| (0..grid.len()/div)
                 .map(|j| {
                     let mut chunk = vec![vec!['.'; div]; div];
                     for di in 0..div {
                        for dj in 0..div {
                            chunk[di][dj] = grid[div*i + di][div*j + dj];
                        }
                     }

                     chunk
                 }).collect()
                ).collect()
    }

    fn merge_chunks(chunks: &[Vec<Chunk>]) -> Vec<Vec<char>> {
        let div = chunks[0][0].len();

        let mut grid = vec![Vec::new(); div*chunks.len()];

        for (i, row_chunks) in chunks.iter().enumerate() {
            for chunk in row_chunks.iter() {
                for di in 0..div {
                    grid[i*div + di].extend_from_slice(&chunk[di]);
                }
            }
        }

        grid
    }

    pub fn solve(raw_rules: &[String], max_it: usize) -> usize {
        let rules = parse_rules(raw_rules);
        let mut grid: Vec<Vec<char>> = ".#.\n..#\n###".split('\n')
            .map(|s| s.chars().collect())
            .collect();

        for _ in 0..max_it {
            let mut chunks = split_grid(&grid);

            for row in chunks.iter_mut() {
                for chunk in row.iter_mut() {
                    let rule = rules.iter().find(|r| r.is_match(&chunk)).unwrap();
                    enhance(chunk, rule);
                }
            }

            grid = merge_chunks(&chunks);
        }

        grid.iter().map(|row| row.iter().filter(|&&c| c == '#').count()).sum()
    }

    #[cfg(test)]
    mod test_day21 {
        use super::*;

        #[test]
        fn test_solve() {
            let rules = vec![
            "../.# => ##./#../...".to_string(),
            ".#./..#/### => #..#/..../..../#..#".to_string(),
            ];

            assert_eq!(solve(&rules, 2), 12);
        }

        #[test]
        fn test_rot90() {
            let chunk = vec![
                vec!['a', 'b'],
                vec!['c', 'd'],
            ];
            assert_eq!(rot90(&chunk), vec![
                       vec!['c', 'a'],
                       vec!['d', 'b'],
            ]);

            let chunk = vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ];
            assert_eq!(rot90(&chunk), vec![
                vec!['g', 'd', 'a'],
                vec!['h', 'e', 'b'],
                vec!['i', 'f', 'c'],
            ]);
        }

        #[test]
        fn test_flip() {
            let chunk = vec![
                vec!['a', 'b'],
                vec!['c', 'd'],
            ];
            assert_eq!(flip(&chunk), vec![
                       vec!['b', 'a'],
                       vec!['d', 'c'],
            ]);

            let chunk = vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ];
            assert_eq!(flip(&chunk), vec![
                vec!['c', 'b', 'a'],
                vec!['f', 'e', 'd'],
                vec!['i', 'h', 'g'],
            ]);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let raw_rules: Vec<String> = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect();

    println!("Solution A-part: {}", day21::solve(&raw_rules, 5));
    println!("Solution B-part: {}", day21::solve(&raw_rules, 18));

}
