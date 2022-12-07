mod day07 {
    use std::collections::HashMap;

    enum FileSystem {
        File(usize),
        Dir(HashMap<String, FileSystem>),
    }

    impl FileSystem {
        fn size(&self) -> usize {
            match self {
                FileSystem::File(s) => { *s },
                FileSystem::Dir(subfs) => {
                    subfs.values()
                        .map(|fs| fs.size())
                        .sum()
                },
            }
        }

        fn size_small_dir(&self, bound: usize) -> usize {
            match self {
                FileSystem::File(_) => { 0 },
                FileSystem::Dir(subfs) => {
                    let fs_size = self.size();
                    (if fs_size <= bound { fs_size } else { 0 })
                    + subfs.values()
                        .map(|fs| fs.size_small_dir(bound))
                        .sum::<usize>()
                },
            }
        }

        fn smallest(&self, bound: usize) -> Option<usize> {
            match self {
                FileSystem::File(_) => { None },
                FileSystem::Dir(subfs) => {
                    match subfs.values()
                        .filter_map(|fs| fs.smallest(bound))
                        .min() {
                            Some(val) => {
                                Some(val)
                            },
                            None => {
                                let fs_size = self.size();
                                if fs_size > bound {
                                    Some(fs_size)
                                }
                                else {
                                    None
                                }
                            },
                    }
                },
            }
        }
    }

    fn parse(input: &str) -> FileSystem {
        let mut cli = input.lines();
        let mut fs = FileSystem::Dir(HashMap::new());
        let mut pwd = &mut fs;
        let mut path = vec![];

        let mut line = cli.next().unwrap();

        loop {
            let words = line.split(' ').collect::<Vec<_>>();
            match words[1] {
                "cd" => {
                    match words[2] {
                        "/" => {
                            path = vec![];
                            pwd =  &mut fs;
                        },
                        ".." => {
                            path.pop();
                            pwd = &mut fs;
                            for dir in path.iter() {
                                pwd = match pwd {
                                    FileSystem::Dir(content) => { content.get_mut(dir).unwrap() },
                                    FileSystem::File(_) => unreachable!(),

                                };
                            }
                        },
                        _ => {
                            path.push(String::from(words[2]));
                            pwd = match pwd {
                                FileSystem::Dir(content) => {
                                    content
                                        .entry(words[2].to_string())
                                        .or_insert(FileSystem::Dir(HashMap::new()))
                                },
                                FileSystem::File(_) => unreachable!(),
                            }
                        },
                    }

                    match cli.next() {
                        Some(line_tmp) => { line = line_tmp; },
                        None => { return fs; },
                    }
                },
                "ls" => {
                    loop {
                        match cli.next() {
                            Some(line_tmp) => {
                                let words = line_tmp.split(' ').collect::<Vec<_>>();
                                match words[0] {
                                    "dir" => {
                                        match pwd {
                                            FileSystem::Dir(content) => {
                                                content
                                                    .entry(words[1].to_string())
                                                    .or_insert(FileSystem::Dir(HashMap::new()))
                                            },
                                            FileSystem::File(_) => unreachable!(),
                                        };
                                    },
                                    "$" => {
                                        line = line_tmp;
                                        break;
                                    },
                                    _ => {
                                        let size = FileSystem::File(words[0].parse().unwrap());
                                        let name = String::from(words[1]);
                                        match pwd {
                                            FileSystem::Dir(content) => {
                                                content.insert(name, size);
                                            },
                                            FileSystem::File(_) => unreachable!(),
                                        };
                                    },
                                }
                            },
                            None => { return fs; },
                        }
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    pub fn solve_a(input: &str) -> usize {
        let fs = parse(input);
        fs.size_small_dir(100000)
    }

    pub fn solve_b(input: &str) -> usize {
        let fs = parse(input);
        let target_unused = 30_000_000;
        let total_space = 70_000_000;
        let unused = total_space - fs.size();
        let missing = target_unused - unused;

        fs.smallest(missing).unwrap()
    }

    #[cfg(test)]
    mod test_day07 {
        use super::*;

        static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        #[test]
        fn test_size() {
            let fs = parse(&INPUT);
            assert_eq!(fs.size(), 48381165);
        }

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(&INPUT), 95437);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(&INPUT), 24933642);
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

    println!("Solution A-part: {}", day07::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day07::solve_b(&buffer.trim()));
}
