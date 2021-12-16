#[aoc(day16, part1)]
fn part1(bits: &str) -> u64 {
    let mut bits = bits.lines().next().unwrap().chars().flat_map(hex_to_binary).collect::<Vec<_>>();

    sum_version_numbers(bits.clone()).0
}

type PacketLength = usize;
fn sum_version_numbers(bits: Vec<u8>) -> (u64, PacketLength) {
    let version_number = parse_version(&bits);

    let operator = parse_packet_type(&bits);
    println!("Package v{}, op{}", version_number, operator);

    match operator {
        4 => {
            // literal
            // don't need to do anything right now
            let (literal, length) = parse_literal(&bits);
            let length = length + 6;
            println!("\tLiteral value: {}, total pkg length: {}", literal, length);
            return (version_number, length);
        },
        _ => {
            // operator
            let length_type = parse_length_type_id(&bits);
            match length_type {
                0 => {
                    // length based
                    let sub_packet_length = parse_sub_packet_length_type_0(&bits);
                    println!("{} bytes of packages excepted, {} bytes left", sub_packet_length, bits.len());
                    //remove our headers
                    let mut sub_package_bits = bits[22..].to_vec();
                    println!("{} bytes left", sub_package_bits.len());
                    let mut sub_package_version_sum = 0;
                    let mut sum_length = 0;
                    while sum_length < sub_packet_length {
                        let (v, l) = sum_version_numbers(sub_package_bits.clone());
                        if l == 0 {
                            println!("time to break, length is {}", sum_length);
                            break;
                        }
                        sub_package_version_sum += v;
                        sum_length += l;
                        sub_package_bits = sub_package_bits[l..].to_vec();
                    }
                    return (sub_package_version_sum + version_number, sum_length + 22)
                },
                1 => {
                    // num packets based
                    let number_of_sub_packets = parse_sub_packet_length_type_1(&bits);
                    let mut sub_package_version_sum = 0;
                    let mut sum_length = 0;
                    let mut sub_package_bits = bits[18..].to_vec();
                    for _ in 0..number_of_sub_packets {
                        let (v, l) = sum_version_numbers(sub_package_bits.clone());
                        sub_package_version_sum += v;
                        sum_length += l;
                        sub_package_bits = sub_package_bits[l..].to_vec();
                    }
                    return (sub_package_version_sum + version_number, sum_length + 18)
                },
                _ => panic!("not supported"),
            }
        },
    }


    todo!()
}

// #[aoc(day16, part2)]
// fn part2(input: &[Line]) -> usize {
//     todo!()
// }

fn hex_to_binary(hex: char) -> [u8; 4] {
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

fn parse_version(packet: &Vec<u8>) -> u64 {
    let version = &packet.as_slice()[0..3];
    bits_to_dec(version)
}

fn parse_packet_type(packet: &Vec<u8>) -> u64 {
    let t = &packet.as_slice()[3..6];
    bits_to_dec(t)
}

fn parse_length_type_id(packet: &Vec<u8>) -> u64 {
    let t = &packet.as_slice()[6..7];
    bits_to_dec(t)
}

fn parse_sub_packet_length_type_0(packet: &Vec<u8>) -> usize {
    let t = &packet.as_slice()[7..22];
    bits_to_dec(t) as usize
}

fn parse_sub_packet_length_type_1(packet: &Vec<u8>) -> u64 {
    let t = &packet.as_slice()[7..18];
    bits_to_dec(t)
}


fn parse_literal(packet: &Vec<u8>) -> (u64, usize) {
    let mut length = 0;
    let mut bits = vec![];
    let mut last_chunk = false;
    for chunk in packet.as_slice()[6..].chunks(5) {
        // println!("\tprocessing chunk: {:?}, length is {}", chunk, length);
        if last_chunk {
            // length += chunk.len();
            break
        }
        length += chunk.len();
        for i in 1..5 {
            bits.push(chunk[i]);
            // length += 1;
        }
        if chunk[0] == 0 {
            // length += chunk.len();
            last_chunk = true;
        }
    }
    // let bits = &packet.as_slice()[6..].chunks(5).flat_map(|chunk|{
    //     if chunk.len() != 5 {
    //         &[]
    //     } else {
    //         &chunk[1..]
    //     }
    // })
    //     .map(|b|*b)
    //     .collect::<Vec<_>>();
    (bits_to_dec(bits.as_slice()), length)
}

fn to_u32(slice: &[u32]) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

fn bits_to_dec(bits: &[u8]) -> u64 {
    let mut s = String::new();
    for b in bits {
        s += b.to_string().as_str()
    }
    u64::from_str_radix(s.as_str(), 2).unwrap_or_else(|_|panic!("Unable to parse {:?}", bits))
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day16.txt");
        assert_eq!(part1(input), 6666);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2021/day16.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

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

        let (literal,length) = parse_literal(&bits);
        println!("literal value: {}", literal);
        println!("literal bytes length {}", length);
        assert_eq!(literal, 2021);
        assert_eq!(length, 15); // I changed this from 18 to 15
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

        let literal = parse_literal(&bits).0;
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

        let literal = parse_literal(&bits).0;
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

        let literal = parse_literal(&bits).0;
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

        let literal = parse_literal(&bits).0;
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

        let literal = parse_literal(&bits).0;
        println!("literal value: {}", literal);
        assert_eq!(literal, 3);
    }

    #[test]
    fn test_parse_literal() {
        let bits =  "D2FE28".chars().flat_map(hex_to_binary).collect::<Vec<_>>();
        let (version, length) = sum_version_numbers(bits.clone());
        assert_eq!(version, 6);
        assert_eq!(length, 21);
    }

    #[test]
    fn test_parse_literal_from_inside_pkg() {
        let (version, length) = sum_version_numbers(vec![1,1,0,1,0,0,0,1,0,1,0,0,1,0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0]);
        assert_eq!(version, 6);
        assert_eq!(length, 11);
    }

    #[test]
    fn test_parse_type_0() {
        let bits =  "38006F45291200".chars().flat_map(hex_to_binary).collect::<Vec<_>>();
        let (version, length) = sum_version_numbers(bits.clone());
        assert_eq!(version, 1+6+2);
        assert_eq!(length, 49);
    }

    #[test]
    fn test_parse_type_1() {
        let bits =  "EE00D40C823060".chars().flat_map(hex_to_binary).collect::<Vec<_>>();
        let (version, length) = sum_version_numbers(bits.clone());
        assert_eq!(version, 7+2+4+1);
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
    fn part2_provided_example() {}
}
