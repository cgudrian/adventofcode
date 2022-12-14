use crate::parser::Bounded;

mod parser;

fn solve1(input: &str) -> usize {
    let (_, cave) = parser::parse(input).unwrap();
    let bounds = cave.bounds().to_tuple().unwrap();
    bounds.0 as usize
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Answer 1: {}", solve1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    static EX: &str = include_str!("example.txt");

    #[test]
    fn example1() {
        assert_eq!(solve1(EX), 24);
    }
}
