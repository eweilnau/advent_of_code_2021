pub fn print_answers() {
    println!("\n--- Day 16: Packet Decoder");
    let input = std::fs::read_to_string("assets\\day_16_input.txt").unwrap();
    let packet = Decoder::parse(&input);
    println!(
        "What do you get if you add up the version numbers in all packets? {}",
        packet.get_version_sum(),
    );
    println!(
        "What do you get if you evaluate the expression represented by your hexadecimal-encoded BITS transmission? {}",
        packet.evaluate(),
    );
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

impl Packet {
    fn get_version_sum(&self) -> u64 {
        let mut sum = self.version as u64;
        if let PacketData::Operator { sub_packets } = &self.data {
            sum += sub_packets.iter().map(|p| p.get_version_sum()).sum::<u64>();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        match &self.data {
            PacketData::Literal(value) => *value,
            PacketData::Operator { sub_packets } => match self.type_id {
                0 => sub_packets.iter().map(|p| p.evaluate()).sum::<u64>(),
                1 => sub_packets.iter().map(|p| p.evaluate()).product::<u64>(),
                2 => sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
                5 => {
                    if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("invalid operator: {}", self.type_id),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PacketData {
    Literal(u64),
    Operator { sub_packets: Vec<Packet> },
}

struct Decoder {}

impl Decoder {
    fn parse(input: &str) -> Packet {
        let bits = input
            .trim()
            .chars()
            .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
            .collect::<Vec<_>>()
            .concat();
        let (packet, _) = Self::parse_packet(&bits, 0);
        packet
    }

    fn parse_packet(bits: &str, index: usize) -> (Packet, usize) {
        let mut i = index;
        let version = u8::from_str_radix(&bits[i..i + 3], 2).unwrap();
        i += 3;
        let type_id = u8::from_str_radix(&bits[i..i + 3], 2).unwrap();
        i += 3;
        let (data, i) = match type_id {
            4 => Self::parse_literal(bits, i),
            _ => Self::parse_operator(bits, i),
        };
        (
            Packet {
                version,
                type_id,
                data,
            },
            i,
        )
    }

    fn parse_literal(bits: &str, index: usize) -> (PacketData, usize) {
        let mut i = index;
        let mut literal_bits = String::new();
        loop {
            let continue_bit = &bits[i..i + 1];
            literal_bits += &bits[i + 1..i + 5];
            i += 5;
            if continue_bit == "0" {
                break;
            }
        }
        (
            PacketData::Literal(u64::from_str_radix(&literal_bits, 2).unwrap()),
            i,
        )
    }

    fn parse_operator(bits: &str, index: usize) -> (PacketData, usize) {
        let mut i = index;
        let length_type_id = &bits[i..i + 1];
        i += 1;

        let mut sub_packets = Vec::new();
        if length_type_id == "0" {
            let sub_packet_bit_len = usize::from_str_radix(&bits[i..i + 15], 2).unwrap();
            i += 15;

            while i < index + 16 + sub_packet_bit_len {
                let (packet, new_i) = Self::parse_packet(bits, i);
                sub_packets.push(packet);
                i = new_i;
            }
        } else {
            let sub_packet_len = usize::from_str_radix(&bits[i..i + 11], 2).unwrap();
            i += 11;

            for _ in 0..sub_packet_len {
                let (packet, new_i) = Self::parse_packet(bits, i);
                sub_packets.push(packet);
                i = new_i;
            }
        }

        (PacketData::Operator { sub_packets }, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let packet = Decoder::parse("D2FE28");
        assert_eq!(packet.version, 6);
    }

    #[test]
    fn test_parse_literal() {
        let packet = Decoder::parse("D2FE28");
        assert_eq!(packet.data, PacketData::Literal(2021));
    }

    #[test]
    fn test_parse_operator_with_length_type_id_0() {
        let packet = Decoder::parse("38006F45291200");
        let expected = PacketData::Operator {
            sub_packets: vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    data: PacketData::Literal(10),
                },
                Packet {
                    version: 2,
                    type_id: 4,
                    data: PacketData::Literal(20),
                },
            ],
        };
        assert_eq!(packet.data, expected);
    }

    #[test]
    fn test_parse_operator_with_length_type_id_1() {
        let packet = Decoder::parse("EE00D40C823060");
        let expected = PacketData::Operator {
            sub_packets: vec![
                Packet {
                    version: 2,
                    type_id: 4,
                    data: PacketData::Literal(1),
                },
                Packet {
                    version: 4,
                    type_id: 4,
                    data: PacketData::Literal(2),
                },
                Packet {
                    version: 1,
                    type_id: 4,
                    data: PacketData::Literal(3),
                },
            ],
        };
        assert_eq!(packet.data, expected);
    }

    #[test]
    fn test_packet_get_version_sum() {
        let packet = Decoder::parse("8A004A801A8002F478");
        assert_eq!(packet.get_version_sum(), 16);
    }

    #[test]
    fn test_packet_evaluate_sum_operator() {
        let packet = Decoder::parse("C200B40A82");
        assert_eq!(packet.evaluate(), 3)
    }

    #[test]
    fn test_packet_evaluate_product_operator() {
        let packet = Decoder::parse("04005AC33890");
        assert_eq!(packet.evaluate(), 54)
    }

    #[test]
    fn test_packet_evaluate_minimum_operator() {
        let packet = Decoder::parse("880086C3E88112");
        assert_eq!(packet.evaluate(), 7)
    }

    #[test]
    fn test_packet_evaluate_maximum_operator() {
        let packet = Decoder::parse("CE00C43D881120");
        assert_eq!(packet.evaluate(), 9)
    }

    #[test]
    fn test_packet_evaluate_less_than_operator() {
        let packet = Decoder::parse("D8005AC2A8F0");
        assert_eq!(packet.evaluate(), 1)
    }

    #[test]
    fn test_packet_evaluate_greater_than_operator() {
        let packet = Decoder::parse("F600BC2D8F");
        assert_eq!(packet.evaluate(), 0)
    }

    #[test]
    fn test_packet_evaluate_equal_to_operator() {
        let packet = Decoder::parse("9C0141080250320F1802104A08");
        assert_eq!(packet.evaluate(), 1)
    }
}
