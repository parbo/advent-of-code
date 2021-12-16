use bitlab::ExtractBitsFromVecU8;
use std::iter::*;
use std::time::Instant;

type Answer = i64;

struct Packet {
    version: u8,
    id: u8,
    value: Option<i64>,
    packets: Vec<Packet>,
}

fn parse_packet(bytes: &Vec<u8>, bp: u32, c: usize) -> (Packet, u32) {
    let version = bytes.get_u8(0, bp, 3).unwrap();
    let id = bytes.get_u8(0, bp + 3, 3).unwrap();
    if let 4u8 = id {
        let mut pos = bp + 6;
        let mut num: i64 = 0;
        loop {
            let cont = bytes.get_u8(0, pos, 1).unwrap();
            let nybble = bytes.get_u8(0, pos + 1, 4).unwrap() as i64;
            pos += 5;
            num <<= 4;
            num += nybble;
            if cont == 0u8 {
                break;
            }
        }
        (
            Packet {
                version,
                id,
                value: Some(num),
                packets: vec![],
            },
            pos - (bp + 6),
        )
    } else {
        let length_type = bytes.get_u8(0, bp + 6, 1).unwrap();
        if let 0 = length_type {
            let mut bits = bytes.get_u16(0, bp + 7, 15).unwrap() as u32;
            let mut pos = bp + 22;
            let mut packets = vec![];
            while bits > 0 {
                let (p, len) = parse_packet(bytes, pos, c + 1);
                pos += 6 + len;
                bits -= 6 + len;
                packets.push(p);
                if pos as usize + 11 >= bytes.len() * 8 {
                    break;
                }
            }
            (
                Packet {
                    version,
                    id,
                    value: None,
                    packets,
                },
                pos - (bp + 6),
            )
        } else {
            let num = bytes.get_u16(0, bp + 7, 11).unwrap();
            let mut packets = vec![];
            let mut pos = bp + 18;
            for _i in 0..num {
                let (p, len) = parse_packet(bytes, pos, c + 1);
                pos += 6 + len;
                packets.push(p);
            }
            (
                Packet {
                    version,
                    id,
                    value: None,
                    packets,
                },
                pos - (bp + 6),
            )
        }
    }
}

fn sum_version(packet: &Packet) -> i64 {
    let mut v = packet.version as i64;
    for p in &packet.packets {
        v += sum_version(p)
    }
    v
}

fn calculate(packet: &Packet) -> i64 {
    match packet.id {
        0 => packet.packets.iter().map(|c| calculate(c)).sum(),
        1 => packet.packets.iter().map(|c| calculate(c)).product(),
        2 => packet.packets.iter().map(|c| calculate(c)).min().unwrap(),
        3 => packet.packets.iter().map(|c| calculate(c)).max().unwrap(),
        4 => packet.value.unwrap(),
        5 => {
            assert_eq!(packet.packets.len(), 2);
            if calculate(&packet.packets[0]) > calculate(&packet.packets[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            assert_eq!(packet.packets.len(), 2);
            if calculate(&packet.packets[0]) < calculate(&packet.packets[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            assert_eq!(packet.packets.len(), 2);
            if calculate(&packet.packets[0]) == calculate(&packet.packets[1]) {
                1
            } else {
                0
            }
        }
        _ => panic!(),
    }
}

fn part1(bytes: &Vec<u8>) -> Answer {
    let (packet, _len) = parse_packet(bytes, 0, 0);
    sum_version(&packet)
}

fn part2(bytes: &Vec<u8>) -> Answer {
    let (packet, _len) = parse_packet(bytes, 0, 0);
    calculate(&packet)
}

fn parse(lines: &[String]) -> Vec<u8> {
    let c: Vec<_> = lines[0].chars().collect();
    let b: Vec<u8> = c
        .chunks(2)
        .map(|x| x.iter().collect::<String>())
        .map(|x| u8::from_str_radix(&x, 16).unwrap())
        .collect();
    b
}

fn main() {
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet_1() {
        let data = parse(&vec!["D2FE28".into()]);
        let (p, len) = parse_packet(&data, 0, 0);
        assert_eq!(len, 15);
        assert_eq!(p.id, 4u8);
        assert_eq!(p.version, 6u8);
        assert_eq!(p.value, Some(2021));
    }

    #[test]
    fn test_parse_packet_2() {
        let data = parse(&vec!["38006F45291200".into()]);
        let (p, len) = parse_packet(&data, 0, 0);
        assert_eq!(len, 43);
        assert_eq!(p.id, 6u8);
        assert_eq!(p.version, 1u8);
        assert_eq!(p.packets.len(), 2);
        assert_eq!(p.packets[0].value, Some(10));
        assert_eq!(p.packets[1].value, Some(20));
    }

    #[test]
    fn test_parse_packet_3() {
        let data = parse(&vec!["EE00D40C823060".into()]);
        let (p, len) = parse_packet(&data, 0, 0);
        assert_eq!(len, 45);
        assert_eq!(p.id, 3u8);
        assert_eq!(p.version, 7u8);
        assert_eq!(p.packets.len(), 3);
        assert_eq!(p.packets[0].value, Some(1));
        assert_eq!(p.packets[1].value, Some(2));
        assert_eq!(p.packets[2].value, Some(3));
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&parse(&vec!["8A004A801A8002F478".into()])), 16);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(
            part1(&parse(&vec!["620080001611562C8802118E34".into()])),
            12
        );
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(
            part1(&parse(&vec!["C0015000016115A2E0802F182340".into()])),
            23
        );
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(
            part1(&parse(&vec!["A0016C880162017C3686B18A3D4780".into()])),
            31
        );
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&parse(&vec!["C200B40A82".into()])), 3);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&parse(&vec!["04005AC33890".into()])), 54);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2(&parse(&vec!["880086C3E88112".into()])), 7);
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(part2(&parse(&vec!["CE00C43D881120".into()])), 9);
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(part2(&parse(&vec!["D8005AC2A8F0".into()])), 1);
    }

    #[test]
    fn test_part2_6() {
        assert_eq!(part2(&parse(&vec!["F600BC2D8F".into()])), 0);
    }

    #[test]
    fn test_part2_7() {
        assert_eq!(part2(&parse(&vec!["9C005AC2F8F0".into()])), 0);
    }

    #[test]
    fn test_part2_8() {
        assert_eq!(part2(&parse(&vec!["9C0141080250320F1802104A08".into()])), 1);
    }
}
