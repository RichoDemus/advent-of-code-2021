use std::borrow::Borrow;
use ouroboros::self_referencing;

use crate::day16::{bits_to_dec, hex_to_binary, parse_literal_value};

#[self_referencing]
struct VecAndSlice {
    v: Vec<i32>,
    #[borrows(v)]
    s: &'this [i32],
}

fn create() -> VecAndSlice {
    let vec = vec![1, 2, 3];
    let slice = vec.as_slice();
    VecAndSliceBuilder {
        v: vec,
        s_builder: |v| v.as_slice(),
    }.build()
}

#[self_referencing]
struct RPacket {
    bits: Vec<u8>,
    #[borrows(bits)]
    #[covariant]
    packet: Box<Packet<'this>>,
}

struct OPacket<'a> {
    bits: &'a [u8],
     start: usize,
}

 struct LPacket<'a> {
     bits: &'a [u8],
     start: usize,
}

impl LPacket<'_> {
    fn value(&self) -> u64 {
        let (value, _length) = parse_literal_value2(self.bits, self.start);
        value
    }
}

enum Packet<'a> {
    RootPacket(RPacket),
    OpPacket(OPacket<'a>),
    LiteralPacket(LPacket<'a>),
}

impl<'a> Packet<'a> {
    fn from_hex(hex_string: &str) -> Packet<'a> {
        let bits: Vec<u8> = hex_string.chars().flat_map(hex_to_binary).collect::<Vec<_>>();
        println!("bits: {:?}", bits);
        Packet::RootPacket(RPacketBuilder {
            bits,
            packet_builder: |bits| Box::new(Packet::from_bits(bits.as_slice(),0)),
        }.build())
    }
    fn from_bits(bits: &[u8], start: usize) -> Packet {
        let type_bits = &bits[start+3..start+6];
        let type_id = bits_to_dec(type_bits);
        if type_id == 4 {
            Packet::LiteralPacket(LPacket{ bits, start })
        } else {
            Packet::OpPacket(OPacket{ bits, start })
        }
    }
    fn bits_start(&'a self) -> (&'a[u8], usize) {
        match self {
            Packet::RootPacket(p) => (p.borrow_bits().as_slice(), 0),
            Packet::OpPacket(p) => (p.bits, p.start),
            Packet::LiteralPacket(p) => (p.bits, p.start),
        }
    }

    fn version(&self) -> u64 {
        let (bits, start) = self.bits_start();
        let version_bits = &bits[start..start+3];
        bits_to_dec(version_bits)
    }
    fn type_id(&self) -> u64 {
        let (bits, start) = self.bits_start();
        let type_id_bits = &bits[start+3..start+6];
        bits_to_dec(type_id_bits)
    }
    fn value(&self) -> u64 {
        match self {
            Packet::RootPacket(r) => r.borrow_packet().value(),
            Packet::LiteralPacket(l) => l.value(),
            Packet::OpPacket(_) => todo!(),
        }
    }
}

fn parse_literal_value2(packet: &[u8], start: usize) -> (u64, usize) {
    let mut length = 0;
    let mut bits = vec![];
    let mut last_chunk = false;
    for chunk in packet[start+6..].chunks(5) {
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
// impl<'a> Packet<'a> {
//     fn from_hex(hex_string: &str) -> Packet<'a> {
//         let bits: Vec<u8> = hex_string.chars().flat_map(hex_to_binary).collect::<Vec<_>>();
//         let slice: &'a [u8] = bits.as_slice();
//         let result: Packet<'a> = Packet::RootPacket {
//             bits,
//             packet: Box::new(Packet::from_bits(slice))
//         };
//         result
//     }
//     fn from_bits(bits: &'a [u8]) -> Self {
//         todo!()
//     }
//     fn version(&self) -> u64 {
//         todo!()
//     }
//     fn type_id(&self) -> u64 {
//         todo!()
//     }
//     fn value(&self) -> u64 {
//         todo!()
//     }
//     fn sub_packets(&self) -> Vec<Packet> {
//         todo!()
//     }
// }

// struct Packetz<'a>  {
//     // bytes: Bits<'a>,
//     start: usize,
// }
//
// impl<'a> Packetz<'a> {
//     fn parse(bytes: &'a[bool], start:usize) -> Self {
//
//         let arr: [i32; 1] = [2];
//         let arr2:&[u8] = &[];
//         let v:Vec<u8> = vec![];
//
//
//
//         todo!()
//     }
//     fn from_hex(hex_string: &'a str) -> Self {
//         todo!()
//         // let vec = hex_string.chars().flat_map(hex_to_binary).collect::<Vec<_>>();
//         // let slice:&'a [u8] = vec.as_slice();
//         // let bits:Bits<'a> = Bits { bits: slice };
//         // Self{
//         //     bytes: bits,
//         //     start: 0,
//         // }
//     }
//
//     fn version(&self) -> u64 {
//         todo!()
//     }
//     fn type_id(&self) -> u64 {
//         todo!()
//     }
//     fn value(&self) -> u64 {
//         todo!()
//     }
// }

// struct Bits<'a> {
//     bits: &'a [u8],
// }
//
// impl<'a> Bits<'a> {
//
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "D2FE28";
        let packet = Packet::from_hex(input);

        assert_eq!(packet.version(), 6);
        assert_eq!(packet.type_id(), 4);
        assert_eq!(packet.value(), 2021);
    }
}
