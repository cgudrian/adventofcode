use itertools::Itertools;
use lalrpop_util::lalrpop_mod;

use crate::ast::PacketData;
use crate::ast::PacketData::{Integer, List};

lalrpop_mod!(pub parser);
mod ast;

fn load_packet_pairs(input: &str) -> Vec<(PacketData, PacketData)> {
    load_packets(input).into_iter()
        .tuples::<(_, _)>()
        .collect()
}

fn load_packets(input: &str) -> Vec<PacketData> {
    let parser = parser::PacketDataParser::new();
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| parser.parse(l).unwrap())
        .collect()
}

fn solve_1(input: &str) -> usize {
    load_packet_pairs(input).into_iter()
        .enumerate()
        .fold(0, |res, (idx, (lhs, rhs))| {
            if lhs < rhs {
                res + (idx + 1)
            } else {
                res
            }
        })
}

fn solve_2(input: &str) -> usize {
    let divider1 = List(vec![List(vec!(Integer(2)))]);
    let divider2 = List(vec![List(vec!(Integer(6)))]);

    let mut packets = load_packets(input);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();

    let idx1 = packets.binary_search(&divider1).unwrap();
    let idx2 = packets.binary_search(&divider2).unwrap();
    (idx1 + 1) * (idx2 + 1)
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Answer 1: {}", solve_1(INPUT));
    println!("Answer 2: {}", solve_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EX: &str = include_str!("example.txt");

    #[test]
    fn test_parser() {
        let parser = parser::PacketDataParser::new();

        let expr = parser.parse("22").unwrap();
        assert_eq!(&format!("{:?}", expr), "22");

        let expr = parser.parse("[22]").unwrap();
        assert_eq!(&format!("{:?}", expr), "[22]");

        let expr = parser.parse("[1,2,3,4]").unwrap();
        assert_eq!(&format!("{:?}", expr), "[1, 2, 3, 4]");

        let expr = parser.parse("[1,[2,3,4],5,[6,7]]").unwrap();
        assert_eq!(&format!("{:?}", expr), "[1, [2, 3, 4], 5, [6, 7]]");
    }

    #[test]
    fn test_load_packet_pairs() {
        let packet_pairs = load_packet_pairs(EX);
        assert_eq!(packet_pairs.len(), 8);
        assert_eq!(&format!("{:?}", packet_pairs.first().unwrap().0), "[1, 1, 3, 1, 1]");
        assert_eq!(&format!("{:?}", packet_pairs.first().unwrap().1), "[1, 1, 5, 1, 1]");
        assert_eq!(&format!("{:?}", packet_pairs.last().unwrap().0), "[1, [2, [3, [4, [5, 6, 7]]]], 8, 9]");
        assert_eq!(&format!("{:?}", packet_pairs.last().unwrap().1), "[1, [2, [3, [4, [5, 6, 0]]]], 8, 9]");
    }

    #[test]
    fn test_load_packets() {
        let packets = load_packets(EX);
        assert_eq!(packets.len(), 16);
        assert_eq!(&format!("{:?}", packets.first().unwrap()), "[1, 1, 3, 1, 1]");
        assert_eq!(&format!("{:?}", packets.last().unwrap()), "[1, [2, [3, [4, [5, 6, 0]]]], 8, 9]");
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EX), 13);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EX), 140);
    }
}
