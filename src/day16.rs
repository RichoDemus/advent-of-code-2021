#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<u8> {
    todo!()
}


#[aoc(day16, part1)]
fn part1(input: &[u8]) -> usize {
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
        _ => panic!("Not supported"),
    }
}

fn parse_version(packet: &Vec<u8>) -> u32 {
    let version = &packet.as_slice()[0..3];
    bits_to_dec(version)
}

fn parse_packet_type(packet: &Vec<u8>) -> u32 {
    let t = &packet.as_slice()[3..6];
    bits_to_dec(t)
}

fn parse_length_type_id(packet: &Vec<u8>) -> u32 {
    let t = &packet.as_slice()[6..7];
    bits_to_dec(t)
}

fn parse_sub_packet_length_type_0(packet: &Vec<u8>) -> u32 {
    let t = &packet.as_slice()[7..22];
    bits_to_dec(t)
}

fn parse_sub_packet_length_type_1(packet: &Vec<u8>) -> u32 {
    let t = &packet.as_slice()[7..18];
    bits_to_dec(t)
}


fn parse_literal(packet: &Vec<u8>) -> u32 {
    let mut bits = vec![];
    for chunk in packet.as_slice()[6..].chunks(5) {
        println!("\tprocessing chunk: {:?}", chunk);
        if chunk.len() != 5 {
            break
        }
        for i in 1..5 {
            bits.push(chunk[i])
        }
        if chunk[0] == 0 {
            break
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
    bits_to_dec(bits.as_slice())
}

fn to_u32(slice: &[u32]) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

fn bits_to_dec(bits: &[u8]) -> u32 {
    let mut s = String::new();
    for b in bits {
        s += b.to_string().as_str()
    }
    u32::from_str_radix(s.as_str(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn verify_part1() {
    //     let input = include_str!("../input/2021/day16.txt");
    //     let input = parse_input(input);
    //     assert_eq!(part1(input.as_slice()), 6666);
    // }

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

        let literal = parse_literal(&bits);
        println!("literal value: {}", literal);
        assert_eq!(literal, 2021);
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

        let literal = parse_literal(&bits);
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

        let literal = parse_literal(&bits);
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

        let literal = parse_literal(&bits);
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

        let literal = parse_literal(&bits);
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

        let literal = parse_literal(&bits);
        println!("literal value: {}", literal);
        assert_eq!(literal, 3);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#""#,
        ));

        assert_eq!(result, 5)
    }

    #[test]
    fn part2_provided_example() {}
}
