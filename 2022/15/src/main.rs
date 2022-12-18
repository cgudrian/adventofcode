use itertools::Itertools;

use crate::parser::parse;

mod parser;

fn solve1(input: &str) -> usize {
    let (_, sensors) = parse(input).unwrap();
    sensors
        .into_iter()
        .flat_map(|sensor| sensor.covered_range_for_y(2000000))
        .flatten()
        .unique()
        .count()
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Answer 1: {}", solve1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EX1: &str = include_str!("example.txt");

    #[test]
    fn example1() {
        assert_eq!(solve1(EX1), 26);
    }
}
