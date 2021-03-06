#[aoc(day16, part1)]
fn part1(bits: &str) -> u64 {
    let bits = bits
        .lines()
        .next()
        .unwrap()
        .chars()
        .flat_map(hex_to_binary)
        .collect::<Vec<_>>();
    let bits = bits.as_slice();
    let packet = parse_packet(bits);
    packet.version
}

#[aoc(day16, part2)]
fn part2(bits: &str) -> u64 {
    let bits = bits
        .lines()
        .next()
        .unwrap()
        .chars()
        .flat_map(hex_to_binary)
        .collect::<Vec<_>>();
    let bits = bits.as_slice();
    parse_packet(bits).value
}

struct Packet {
    version: u64,
    length: usize,
    value: u64,
}

fn parse_packet(bits: &[u8]) -> Packet {
    let version_number = parse_version(bits);

    let operator = parse_packet_type(bits);
    println!("Package v{}, op{}", version_number, operator);

    return match operator {
        4 => {
            // literal
            // don't need to do anything right now
            let (value, length) = parse_literal_value(bits);
            let length = length + 6;
            println!("\tLiteral value: {}, total pkg length: {}", value, length);
            Packet {
                version: version_number,
                length,
                value,
            }
        }
        type_id => {
            // operator
            let length_type = parse_length_type_id(bits);
            let mut sub_packets: Vec<Packet> = vec![];
            let extra_length;
            match length_type {
                0 => {
                    // length based
                    let sub_packet_length = parse_sub_packet_length_type_0(bits);
                    println!(
                        "{} bytes of packages excepted, {} bytes left",
                        sub_packet_length,
                        bits.len()
                    );
                    //remove our headers
                    let mut sub_package_bits = &bits[22..];
                    println!("{} bytes left", sub_package_bits.len());
                    let mut sum_length = 0;
                    while sum_length < sub_packet_length {
                        let packet = parse_packet(sub_package_bits);
                        sum_length += packet.length;
                        sub_package_bits = &sub_package_bits[packet.length..];
                        sub_packets.push(packet);
                    }
                    extra_length = 22;
                }
                1 => {
                    // num packets based
                    let number_of_sub_packets = parse_sub_packet_length_type_1(bits);
                    let mut sub_package_bits = &bits[18..];
                    for _ in 0..number_of_sub_packets {
                        let packet = parse_packet(sub_package_bits);
                        sub_package_bits = &sub_package_bits[packet.length..];
                        sub_packets.push(packet);
                    }
                    extra_length = 18;
                }
                _ => panic!("not supported"),
            }
            // lets process the packages
            let total_length: usize = extra_length
                + sub_packets
                    .iter()
                    .map(|packet| packet.length)
                    .sum::<usize>();
            let total_version: u64 = sub_packets.iter().map(|packet| packet.version).sum();
            let subpackets_value = calculate_subpackets_value(type_id, sub_packets);
            Packet {
                version: total_version + version_number,
                length: total_length,
                value: subpackets_value,
            }
        }
    };
}

fn calculate_subpackets_value(type_id: u64, sub_packets: Vec<Packet>) -> u64 {
    match type_id {
        0 => sub_packets.into_iter().map(|packet| packet.value).sum(),
        1 => sub_packets.into_iter().map(|packet| packet.value).product(),
        2 => sub_packets
            .into_iter()
            .map(|packet| packet.value)
            .min()
            .unwrap(),
        3 => sub_packets
            .into_iter()
            .map(|packet| packet.value)
            .max()
            .unwrap(),
        5 => {
            if sub_packets[0].value > sub_packets[1].value {
                1
            } else {
                0
            }
        }
        6 => {
            if sub_packets[0].value < sub_packets[1].value {
                1
            } else {
                0
            }
        }
        7 => {
            if sub_packets[0].value == sub_packets[1].value {
                1
            } else {
                0
            }
        }
        _ => panic!("not supported"),
    }
}

pub fn hex_to_binary(hex: char) -> [u8; 4] {
    match hex {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        e => panic!("Not supported: {}", e),
    }
}

pub fn parse_version(packet: &[u8]) -> u64 {
    let version = &packet[0..3];
    bits_to_dec(version)
}

pub fn parse_packet_type(packet: &[u8]) -> u64 {
    let t = &packet[3..6];
    bits_to_dec(t)
}

fn parse_length_type_id(packet: &[u8]) -> u64 {
    let t = &packet[6..7];
    bits_to_dec(t)
}

#[allow(clippy::cast_possible_truncation)]
fn parse_sub_packet_length_type_0(packet: &[u8]) -> usize {
    let t = &packet[7..22];
    bits_to_dec(t) as usize
}

fn parse_sub_packet_length_type_1(packet: &[u8]) -> u64 {
    let t = &packet[7..18];
    bits_to_dec(t)
}

pub fn parse_literal_value(packet: &[u8]) -> (u64, usize) {
    let mut length = 0;
    let mut bits = vec![];
    let mut last_chunk = false;
    for chunk in packet[6..].chunks(5) {
        if last_chunk {
            break;
        }
        length += chunk.len();
        for chunk in chunk.iter().take(5).skip(1) {
            bits.push(*chunk);
        }
        if chunk[0] == 0 {
            last_chunk = true;
        }
    }
    (bits_to_dec(bits.as_slice()), length)
}

pub fn bits_to_dec(bits: &[u8]) -> u64 {
    let mut s = String::new();
    for b in bits {
        s += b.to_string().as_str();
    }
    u64::from_str_radix(s.as_str(), 2).unwrap_or_else(|_| panic!("Unable to parse {:?}", bits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day16.txt");
        assert_eq!(part1(input), 963);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day16.txt");
        let result = part2(input);
        assert_eq!(result, 1549026292886);
    }

    #[test]
    fn small_test_literal() {
        let input = "D2FE28";

        let bits = input.chars().flat_map(hex_to_binary).collect::<Vec<_>>();
        println!("{:?}", bits);

        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 6);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 4, "4 = literal");

        let (literal, length) = parse_literal_value(&bits);
        println!("literal value: {}", literal);
        println!("literal bytes length {}", length);
        assert_eq!(literal, 2021);
        assert_eq!(length, 15);
    }

    #[test]
    fn small_test_subpackets_type_0() {
        let input = "38006F45291200";
        let mut bits = input.chars().flat_map(hex_to_binary).collect::<Vec<_>>();

        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 1);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 6, "6 = operator");

        let length_type = parse_length_type_id(&bits);
        println!("length type: {}", length_type);
        assert_eq!(length_type, 0);

        let sub_packet_length = parse_sub_packet_length_type_0(&bits);
        println!("sub-packet length: {}", sub_packet_length);
        assert_eq!(sub_packet_length, 27);
        // this packet is done, the two next ones should be length 27 together
        bits = bits[22..].to_vec();

        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 6);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 4, "4 = literal");

        let literal = parse_literal_value(&bits).0;
        println!("literal value: {}", literal);
        assert_eq!(literal, 10);

        bits = bits[11..].to_vec();
        println!("last package");
        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 2);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 4, "4 = literal");

        let literal = parse_literal_value(&bits).0;
        println!("literal value: {}", literal);
        assert_eq!(literal, 20);
    }

    #[test]
    fn small_test_subpackets_type_1() {
        let input = "EE00D40C823060";
        let mut bits = input.chars().flat_map(hex_to_binary).collect::<Vec<_>>();

        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 7);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 3, "3 = operator");

        let length_type = parse_length_type_id(&bits);
        println!("length type: {}", length_type);
        assert_eq!(length_type, 1);

        let sub_packet_length = parse_sub_packet_length_type_1(&bits);
        println!("sub-packet length: {}", sub_packet_length);
        assert_eq!(sub_packet_length, 3);

        // first sub pakcet
        bits = bits[18..].to_vec();
        println!("last package");
        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 2);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 4, "4 = literal");

        let literal = parse_literal_value(&bits).0;
        println!("literal value: {}", literal);
        assert_eq!(literal, 1);

        // second sub pakcet
        bits = bits[11..].to_vec();
        println!("last package");
        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 4);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 4, "4 = literal");

        let literal = parse_literal_value(&bits).0;
        println!("literal value: {}", literal);
        assert_eq!(literal, 2);

        // third sub pakcet
        bits = bits[11..].to_vec();
        println!("last package");
        let version = parse_version(&bits);
        println!("version: {}", version);
        assert_eq!(version, 1);

        let packet_type = parse_packet_type(&bits);
        println!("packet type: {}", packet_type);
        assert_eq!(packet_type, 4, "4 = literal");

        let literal = parse_literal_value(&bits).0;
        println!("literal value: {}", literal);
        assert_eq!(literal, 3);
    }

    #[test]
    fn test_parse_literal() {
        let bits = "D2FE28".chars().flat_map(hex_to_binary).collect::<Vec<_>>();
        let Packet {
            version,
            length,
            value: _,
        } = parse_packet(bits.as_slice());
        assert_eq!(version, 6);
        assert_eq!(length, 21);
    }

    #[test]
    fn test_parse_literal_from_inside_pkg() {
        let input = vec![
            1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        let Packet {
            version,
            length,
            value: _,
        } = parse_packet(input.as_slice());
        assert_eq!(version, 6);
        assert_eq!(length, 11);
    }

    #[test]
    fn test_parse_type_0() {
        let bits = "38006F45291200"
            .chars()
            .flat_map(hex_to_binary)
            .collect::<Vec<_>>();
        let Packet {
            version,
            length,
            value: _,
        } = parse_packet(bits.as_slice());
        assert_eq!(version, 1 + 6 + 2);
        assert_eq!(length, 49);
    }

    #[test]
    fn test_parse_type_1() {
        let bits = "EE00D40C823060"
            .chars()
            .flat_map(hex_to_binary)
            .collect::<Vec<_>>();
        let Packet {
            version,
            length,
            value: _,
        } = parse_packet(bits.as_slice());
        assert_eq!(version, 7 + 2 + 4 + 1);
        assert_eq!(length, 51);
    }

    #[test]
    fn part1_provided_example() {
        assert_eq!(part1("8A004A801A8002F478"), 16, "one");
        assert_eq!(part1("620080001611562C8802118E34"), 12, "two");
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23, "three");
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31, "four");
    }

    #[test]
    fn part2_provided_example() {
        assert_eq!(part2("C200B40A82"), 3, "1");
        assert_eq!(part2("04005AC33890"), 54, "2");
        assert_eq!(part2("880086C3E88112"), 7, "3");
        assert_eq!(part2("CE00C43D881120"), 9, "4");
        assert_eq!(part2("D8005AC2A8F0"), 1, "5");
        assert_eq!(part2("F600BC2D8F"), 0, "6");
        assert_eq!(part2("9C005AC2F8F0"), 0, "7");
        assert_eq!(part2("9C0141080250320F1802104A08"), 1, "8");
    }
}
