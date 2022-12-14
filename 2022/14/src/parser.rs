use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{char, newline};
use nom::IResult;
use nom::multi::{many0, separated_list1};
use nom::sequence::separated_pair;

#[derive(Debug, PartialEq)]
pub struct Path(Vec<Point>);

#[derive(Debug, PartialEq)]
struct Point(u16, u16);

pub fn parse(input: &str) -> IResult<&str, Vec<Path>> {
    many0(path)(input)
}

fn path(input: &str) -> IResult<&str, Path> {
    let (input, points) = separated_list1(tag(" -> "), point)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, Path(points)))
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(number, char(','), number)(input)?;
    Ok((input, Point(x, y)))
}

fn number(input: &str) -> IResult<&str, u16> {
    complete::u16(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let (_, path) = parse("1,1 -> 2,2\n3,3 -> 4,4 -> 5,5\n").unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], Path(vec![Point(1, 1), Point(2, 2)]));
        assert_eq!(path[1], Path(vec![Point(3, 3), Point(4, 4), Point(5, 5)]));
    }
}
