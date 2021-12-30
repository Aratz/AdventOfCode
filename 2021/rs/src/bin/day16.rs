mod day16 {
    enum Content {
        Literal(usize), // ID 4
        Operator(Vec<Packet>) // ID 6
    }

    struct Packet {
        version: usize,
        type_id: usize,
        content: Content,
    }

    impl Packet {
        fn version_sum(&self) -> usize {
            self.version 
                + match &self.content {
                    Content::Literal(_) => 0,
                    Content::Operator(packets) => packets.iter().map(|p| p.version_sum()).sum(),
                }
        }

        fn eval(&self) -> usize {
            match &self.content {
                Content::Literal(v) => *v,
                Content::Operator(packets) => {
                    match &self.type_id {
                        0 => packets.iter().map(|p| p.eval()).sum(),
                        1 => packets.iter().map(|p| p.eval()).product(),
                        2 => packets.iter().map(|p| p.eval()).min().unwrap(),
                        3 => packets.iter().map(|p| p.eval()).max().unwrap(),
                        5 => if packets[0].eval() > packets[1].eval() { 1 } else { 0 },
                        6 => if packets[0].eval() < packets[1].eval() { 1 } else { 0 },
                        7 => if packets[0].eval() == packets[1].eval() { 1 } else { 0 },
                        _ => unreachable!(),
                    }
                },
            }
        }
    }

    fn parse_input(input: &str) -> Vec<char> {
        input.chars()
            .map(|c| format!("{:0>4b}", c.to_digit(16).unwrap()).chars().collect::<Vec<char>>())
            .flatten()
            .collect()
    }

    #[inline]
    fn vec_to_usize(num: &[char]) -> usize {
        usize::from_str_radix(&num.iter().collect::<String>(), 2).unwrap()
    }

    fn decode_packet(packet: &[char]) -> (Packet, usize) {
        let version = vec_to_usize(&packet[0..3]);
        let type_id = vec_to_usize(&packet[3..6]);
        let (content, total_len) = match type_id {
            4 => {
                let mut lit_val = Vec::new();
                let mut i = 6;

                loop {
                    let group = &packet[i..i+5];
                    lit_val.extend_from_slice(&group[1..]);

                    i += 5;

                    if group[0] == '0' { break; }
                }

                (Content::Literal(vec_to_usize(&lit_val)), i)
            },
            _ => {
                match packet[6] {
                    '0' => {
                        let total_len = vec_to_usize(&packet[7..7+15]);
                        let mut i = 7 + 15;
                        let mut packets = Vec::new();
                        while i < 7 + 15 + total_len {
                            let (packet, len) = decode_packet(&packet[i..]);
                            packets.push(packet);
                            i += len;
                        }

                        (Content::Operator(packets), i)
                    },
                    '1' => {
                        let total_packets = vec_to_usize(&packet[7..7+11]);
                        let mut i = 7 + 11;
                        let mut n_packets = 0;
                        let mut packets = Vec::new();
                        while n_packets < total_packets {
                            let (packet, len) = decode_packet(&packet[i..]);
                            packets.push(packet);
                            n_packets += 1;
                            i += len;
                        }

                        (Content::Operator(packets), i)
                    },
                    _ => unreachable!(),
                }
            },
        };

        (Packet { version, type_id, content }, total_len)
    }

    pub fn solve_a(input: &str) -> usize {
        let raw_packet = parse_input(input);

        let (packet, _) = decode_packet(&raw_packet);

        packet.version_sum()
    }

    pub fn solve_b(input: &str) -> usize {
        let raw_packet = parse_input(input);

        let (packet, _) = decode_packet(&raw_packet);

        packet.eval()
    }

    #[cfg(test)]
    mod test_day16 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(&"D2FE28"), 6);
            assert_eq!(solve_a(&"38006F45291200"), 9);
            assert_eq!(solve_a(&"EE00D40C823060"), 14);
            assert_eq!(solve_a(&"8A004A801A8002F478"), 16);
            assert_eq!(solve_a(&"620080001611562C8802118E34"), 12);
            assert_eq!(solve_a(&"C0015000016115A2E0802F182340"), 23);
            assert_eq!(solve_a(&"A0016C880162017C3686B18A3D4780"), 31);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(&"C200B40A82"), 3);
            assert_eq!(solve_b(&"04005AC33890"), 54);
            assert_eq!(solve_b(&"880086C3E88112"), 7);
            assert_eq!(solve_b(&"CE00C43D881120"), 9);
            assert_eq!(solve_b(&"D8005AC2A8F0"), 1);
            assert_eq!(solve_b(&"F600BC2D8F"), 0);
            assert_eq!(solve_b(&"9C005AC2F8F0"), 0);
            assert_eq!(solve_b(&"9C0141080250320F1802104A08"), 1);
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

    println!("Solution A-part: {}", day16::solve_a(buffer.trim()));
    println!("Solution B-part: {}", day16::solve_b(buffer.trim()));
}
