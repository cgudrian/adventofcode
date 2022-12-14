use crate::parser::{Bounded, Bounds, Point};

mod parser;

struct Cave {
    data: Vec<Vec<u8>>,
    bounds: Bounds<usize>,
}

impl Cave {
    fn new(bounds: &Bounds<usize>) -> Cave {
        let width = bounds.delta_x().unwrap() + 1;
        let height = bounds.delta_y().unwrap() + 1;
        let data = (0..height).map(|_| vec![b'.'; width]).collect::<Vec<_>>();
        Cave {
            data,
            bounds: *bounds,
        }
    }

    fn set_rock(&mut self, p: &Point) {
        self.data[p.y() - self.bounds.ymin().unwrap()][p.x() - self.bounds.xmin().unwrap()] = b'#';
    }

    fn dump(&self) {
        for row in &self.data {
            println!("{}", String::from_utf8(row.clone()).unwrap())
        }
    }
}

fn solve1(input: &str) -> usize {
    let (_, paths) = parser::parse(input).unwrap();
    let bounds = paths.bounds();
    let mut cave = Cave::new(&bounds);
    for p in paths.points_iter() {
        cave.set_rock(&p);
    }
    cave.dump();
    0
}

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Answer 1: {}", solve1(INPUT));
    let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let v2: Vec<&u8> = v.iter().flat_map(|e| e.iter()).collect();

    let v = vec![1, 2, 3, 4, 5];
    let res: Vec<(&i32, &i32)> = v.iter().zip(v.iter().skip(1)).collect();
    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;

    static EX: &str = include_str!("example.txt");

    #[test]
    fn example1() {
        assert_eq!(solve1(EX), 24);
    }
}
