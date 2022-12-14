use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{char, newline};
use nom::combinator::all_consuming;
use nom::IResult;
use nom::multi::{many0, separated_list1};
use nom::sequence::separated_pair;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bounds<T> {
    Empty,
    Bounded {
        xmin: T,
        xmax: T,
        ymin: T,
        ymax: T,
    },
}

pub trait Bounded<T> {
    fn bounds(&self) -> Bounds<T>;
}

impl<T: Copy + Ord> Bounds<T> {
    fn new() -> Bounds<T> {
        Bounds::Empty
    }

    fn union(&self, rhs: &Bounds<T>) -> Self {
        match (self, rhs) {
            (Bounds::Empty, Bounds::Empty) => Bounds::Empty,
            (lhs, Bounds::Empty) => *lhs,
            (Bounds::Empty, rhs) => *rhs,
            (Bounds::Bounded {
                xmin: xminl,
                xmax: xmaxl,
                ymin: yminl,
                ymax: ymaxl
            }, Bounds::Bounded {
                xmin: xminr,
                xmax: xmaxr,
                ymin: yminr,
                ymax: ymaxr
            }) => Bounds::Bounded {
                xmin: *xminl.min(xminr),
                xmax: *xmaxl.max(xmaxr),
                ymin: *yminl.min(yminr),
                ymax: *ymaxl.max(ymaxr),
            }
        }
    }

    pub fn to_tuple(&self) -> Option<(T, T, T, T)> {
        match self {
            Bounds::Empty => None,
            Bounds::Bounded { xmin, xmax, ymin, ymax } => Some((*xmin, *xmax, *ymin, *ymax)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Cave(Vec<Path>);

impl<T, E> Bounded<T> for Vec<E>
    where T: Copy + Ord,
          E: Bounded<T>,
{
    fn bounds(&self) -> Bounds<T> {
        self.iter()
            .fold(Bounds::Empty, |mut bounds, b| {
                bounds.union(&b.bounds())
            })
    }
}

impl Bounded<u16> for Cave {
    fn bounds(&self) -> Bounds<u16> {
        self.0.bounds()
    }
}

#[derive(Debug, PartialEq)]
pub struct Path(Vec<Point>);

impl Bounded<u16> for Path {
    fn bounds(&self) -> Bounds<u16> {
        self.0.bounds()
    }
}

#[derive(Debug, PartialEq)]
struct Point(u16, u16);

impl Bounded<u16> for Point {
    fn bounds(&self) -> Bounds<u16> {
        Bounds::Bounded {
            xmin: self.0,
            xmax: self.0,
            ymin: self.1,
            ymax: self.1,
        }
    }
}

pub fn parse(input: &str) -> IResult<&str, Cave> {
    let (input, paths) = all_consuming(many0(path))(input)?;
    Ok((input, Cave(paths)))
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
    fn valid_input() {
        let (_, cave) = parse("101,202 -> 303,404\n505,606 -> 707,808 -> 909,1000\n").unwrap();
        assert_eq!(cave, Cave(vec![
            Path(vec![Point(101, 202), Point(303, 404)]),
            Path(vec![Point(505, 606), Point(707, 808), Point(909, 1000)]),
        ]));
    }

    #[test]
    fn empty_input_is_ok() {
        assert!(parse("").is_ok());
    }

    #[test]
    fn missing_eol() {
        assert!(parse("1,1 -> 2,2").is_err());
    }

    #[test]
    fn only_whitespace_is_err() {
        assert!(parse("  \t ").is_err());
    }

    #[test]
    fn path_bounds() {
        let path = Path(vec![Point(10, 1), Point(3, 4), Point(1, 20)]);
        assert_eq!(path.bounds(), Bounds::Bounded { xmin: 1, xmax: 10, ymin: 1, ymax: 20 });
    }

    #[test]
    fn cave_bounds() {
        let cave = Cave(vec![
            Path(vec![Point(101, 202), Point(303, 404)]),
            Path(vec![Point(505, 606), Point(707, 808), Point(909, 1000)]),
        ]);
        assert_eq!(cave.bounds(), Bounds::Bounded {
            xmin: 101,
            xmax: 909,
            ymin: 202,
            ymax: 1000,
        });
    }

    #[test]
    fn bounds_union_bounded_bounded() {
        let b1 = Bounds::Bounded { xmin: 1, xmax: 4, ymin: 2, ymax: 10 };
        let b2 = Bounds::Bounded { xmin: 10, xmax: 40, ymin: 20, ymax: 100 };
        assert_eq!(b1.union(&b2), Bounds::Bounded { xmin: 1, xmax: 40, ymin: 2, ymax: 100 });
    }

    #[test]
    fn bounds_union_empty_bounded() {
        let b1 = Bounds::Empty;
        let b2 = Bounds::Bounded { xmin: 10, xmax: 40, ymin: 20, ymax: 100 };
        assert_eq!(b1.union(&b2), b2);
    }

    #[test]
    fn bounds_union_bounded_empty() {
        let b1 = Bounds::Bounded { xmin: 1, xmax: 4, ymin: 2, ymax: 10 };
        let b2 = Bounds::Empty;
        assert_eq!(b1.union(&b2), b1);
    }

    #[test]
    fn bounds_to_tuple_empty() {
        let b = Bounds::<u16>::Empty;
        assert_eq!(b.to_tuple(), None);
    }

    #[test]
    fn bounds_to_tuple_bounded() {
        let b = Bounds::Bounded { xmin: 1, xmax: 2, ymin: 3, ymax: 4 };
        assert_eq!(b.to_tuple(), Some((1, 2, 3, 4)));
    }
}
