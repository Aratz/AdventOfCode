mod day07 {
    use std::collections::HashMap;

    enum InputWire {
        Val(u16), Wire(String)
    }

    impl InputWire {
        pub fn parse(input_wire: &str) -> Self {
            match input_wire.parse() {
                Ok(val) => { Self::Val(val) },
                Err(_) => { Self::Wire(input_wire.to_string()) },
            }
        }
    }

    enum Gate {
        Sig(InputWire),
        Not(InputWire),
        And(InputWire, InputWire),
        Or(InputWire, InputWire),
        Lshift(InputWire, usize),
        Rshift(InputWire, usize),
    }

    fn parse(input: &str) -> HashMap<String, Gate> {
        let mut network = HashMap::new();
        for line in input.lines() {
            let (input_values, output_wire) = {
                let io = line.split(" -> ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                (
                    io[0].split(' ').map(|s| s.to_string()).collect::<Vec<_>>(),
                    io[1].clone()
                    )
            };
            match input_values.len() {
                1 => {
                    network.insert(
                        output_wire,
                        Gate::Sig(InputWire::parse(&input_values[0])));
                },
                2 => {
                    network.insert(
                        output_wire,
                        Gate::Not(InputWire::parse(&input_values[1])));
                },
                3 => {
                    match input_values[1].as_str() {
                        "AND" => {
                            network.insert(
                                output_wire,
                                Gate::And(
                                    InputWire::parse(&input_values[0]),
                                    InputWire::parse(&input_values[2])));
                        },
                        "OR" => {
                            network.insert(
                                output_wire,
                                Gate::Or(
                                    InputWire::parse(&input_values[0]),
                                    InputWire::parse(&input_values[2])));
                        },
                        "LSHIFT" => {
                            network.insert(
                                output_wire,
                                Gate::Lshift(
                                    InputWire::parse(&input_values[0]),
                                    input_values[2].parse().unwrap()));
                        },
                        "RSHIFT" => {
                            network.insert(
                                output_wire,
                                Gate::Rshift(
                                    InputWire::parse(&input_values[0]),
                                    input_values[2].parse().unwrap()));
                        },
                        _ => panic!("Input error!"),
                    }
                },
                _ => panic!("Input error!"),
            }
        }

        network
    }

    fn solve_dp(
        network: &HashMap<String, Gate>,
        wires: &mut HashMap<String, u16>,
        target: &InputWire) -> u16 {
        match target {
            InputWire::Val(val) => *val,
            InputWire::Wire(target) => {
                match wires.get(target) {
                    Some(&v) => v,
                    None => {
                        match &network[target] {
                            Gate::Sig(i) => {
                                let input = solve_dp(network, wires, &i);
                                wires.insert(target.to_string(), input);
                            }
                            Gate::Not(i) => {
                                let input = solve_dp(network, wires, &i);
                                wires.insert(target.to_string(), !input);
                            },
                            Gate::And(i1, i2) => {
                                let input1 = solve_dp(network, wires, &i1);
                                let input2 = solve_dp(network, wires, &i2);
                                wires.insert(target.to_string(), input1 & input2);
                            },
                            Gate::Or(i1, i2) => {
                                let input1 = solve_dp(network, wires, &i1);
                                let input2 = solve_dp(network, wires, &i2);
                                wires.insert(target.to_string(), input1 | input2);
                            },
                            Gate::Lshift(i, shift) => {
                                let input = solve_dp(network, wires, &i);
                                wires.insert(target.to_string(), input << shift);
                            },
                            Gate::Rshift(i, shift) => {
                                let input = solve_dp(network, wires, &i);
                                wires.insert(target.to_string(), input >> shift);
                            },
                        }

                        wires[target]
                    }
                }
            }
        }
    }

    pub fn solve_a(input: &str, target: &str) -> u16 {
        let network = parse(input);

        let mut wires = HashMap::new();

        solve_dp(&network, &mut wires, &InputWire::Wire(target.to_string()))
    }

    pub fn solve_b(input: &str, target: &str) -> u16 {
        let network = parse(input);

        let mut wires = HashMap::new();

        let a = solve_dp(&network, &mut wires, &InputWire::Wire(target.to_string()));

        let mut wires = HashMap::new();
        wires.insert(String::from("b"), a);

        solve_dp(&network, &mut wires, &InputWire::Wire(target.to_string()))
    }

    #[cfg(test)]
    mod test_day07 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

            assert_eq!(solve_a(&input, "d"), 72);
            assert_eq!(solve_a(&input, "e"), 507);
            assert_eq!(solve_a(&input, "f"), 492);
            assert_eq!(solve_a(&input, "g"), 114);
            assert_eq!(solve_a(&input, "h"), 65412);
            assert_eq!(solve_a(&input, "i"), 65079);
            assert_eq!(solve_a(&input, "x"), 123);
            assert_eq!(solve_a(&input, "y"), 456);
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

    println!("Solution A-part: {}", day07::solve_a(&buffer.trim(), "a"));
    println!("Solution B-part: {}", day07::solve_b(&buffer.trim(), "a"));
}
