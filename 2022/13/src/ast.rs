use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum PacketData {
    Integer(usize),
    List(Vec<PacketData>),
}

impl Eq for PacketData {}

impl PartialEq<Self> for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, rhs: &PacketData) -> Ordering {
        use PacketData::*;

        match (self, rhs) {
            (Integer(l), Integer(r)) => l.cmp(r),

            (l @ List(_), r @ Integer(_)) => l.cmp(&List(vec![r.clone()])),

            (l @ Integer(_), r @ List(_)) => List(vec![l.clone()]).cmp(r),

            (List(l), List(r)) => {
                for (l, r) in l.iter().zip(r.iter()) {
                    let res = l.cmp(r);
                    if res != Ordering::Equal {
                        return res;
                    }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

impl Debug for PacketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PacketData::*;

        match self {
            Integer(n) => write!(f, "{}", n),
            List(l) => { write!(f, "{:?}", l) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::PacketData::*;

    #[test]
    fn compare_two_numbers() {
        assert_eq!(Integer(2).cmp(&Integer(0)), Ordering::Greater);
        assert_eq!(Integer(0).cmp(&Integer(2)), Ordering::Less);
        assert_eq!(Integer(4).cmp(&Integer(4)), Ordering::Equal);
    }

    #[test]
    fn compare_list_and_number() {
        assert_eq!(List(vec![Integer(2), Integer(3), Integer(4)]).cmp(&Integer(4)), Ordering::Less);
        assert_eq!(Integer(4).cmp(&List(vec![Integer(2), Integer(3), Integer(4)])), Ordering::Greater);
    }

    #[test]
    fn compare_lists() {
        assert_eq!(List(vec![Integer(1), Integer(2)]).cmp(&List(vec![Integer(1), Integer(2)])), Ordering::Equal);
        assert_eq!(List(vec![Integer(1), Integer(2)]).cmp(&List(vec![Integer(1), Integer(2), Integer(3)])), Ordering::Less);
        assert_eq!(List(vec![Integer(1), Integer(2), Integer(3)]).cmp(&List(vec![Integer(1), Integer(2)])), Ordering::Greater);
    }

    #[test]
    fn debug_output() {
        let expr = List(vec![Integer(1), Integer(2), Integer(3)]);
        let fmt = format!("{:?}", expr);
        assert_eq!(fmt, "[1, 2, 3]");

        let expr = List(vec![Integer(1), List(vec![Integer(1), Integer(2), Integer(3)])]);
        let fmt = format!("{:?}", expr);
        assert_eq!(fmt, "[1, [1, 2, 3]]");
    }
}