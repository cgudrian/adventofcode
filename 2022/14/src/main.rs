use std::iter::once;

use crate::parser::{Bounded, Bounds, Paths, Point};

mod parser;
mod cave;

fn solve1(input: &str) -> usize {
    let (_, rocks) = parser::parse(input).unwrap();
    let mut cave = cave::Cave::new((500, 0), Some(&rocks), false);
    let mut counter = 0;
    while cave.drop_sand() {
        counter += 1;
    }
    println!("{}", cave);
    counter
}

fn solve2(input: &str) -> usize {
    let (_, rocks) = parser::parse(input).unwrap();
    let mut cave = cave::Cave::new((500, 0), Some(&rocks), true);
    let mut counter = 0;

    while cave.drop_sand() {
        counter += 1;
    }
    println!("{}", cave);
    counter + 1
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Answer 1: {}", solve1(INPUT));
    println!("Answer 2: {}", solve2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;

    static EX: &str = include_str!("example.txt");

    #[test]
    fn example1() {
        //assert_eq!(solve2(EX), 24);
    }
}
