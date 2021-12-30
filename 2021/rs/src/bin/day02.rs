mod day02 {
    fn execute_course_a(course: &[(String, i32)]) -> (i32, i32) {
        let mut pos = (0, 0);
        for (command, x) in course {
            pos = match command.as_str() {
                "forward" => { (pos.0 + x, pos.1) },
                "down" => { (pos.0, pos.1 + x) },
                "up" => { (pos.0, pos.1 - x) },
                _ => unreachable!(),
            };
        }

        pos
    }

    fn execute_course_b(course: &[(String, i32)]) -> (i32, i32) {
        let mut aim = 0;
        let mut pos = (0, 0);
        for (command, x) in course {
            match command.as_str() {
                "forward" => { pos = (pos.0 + x, pos.1 + x * aim) },
                "down" => { aim += x },
                "up" => { aim -= x },
                _ => unreachable!(),
            };
        }

        pos
    }

    pub fn solve_a(course: &[(String, i32)]) -> i32 {
        let final_pos = execute_course_a(course);
        final_pos.0 * final_pos.1
    }

    pub fn solve_b(course: &[(String, i32)]) -> i32 {
        let final_pos = execute_course_b(course);
        final_pos.0 * final_pos.1
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let course: Vec<(String, i32)> = stdin.lock().lines()
        .map(|s| {
            let command: Vec<String> = s.unwrap().split(' ').map(|s| s.into()).collect();
            (String::from(&command[0]), command[1].parse().unwrap())})
        .collect();

    println!("Solution A-part: {}", day02::solve_a(&course));
    println!("Solution B-part: {}", day02::solve_b(&course));
}
