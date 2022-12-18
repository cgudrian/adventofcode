use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::newline;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point(i32, i32);

impl Point {
    fn distance_to(&self, rhs: &Point) -> i32 {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SensorReading {
    pos: Point,
    beacon: Point,
    distance: i32,
}

impl SensorReading {
    fn new(pos: Point, beacon: Point) -> SensorReading {
        SensorReading {
            pos,
            beacon,
            distance: pos.distance_to(&beacon),
        }
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<SensorReading>> {
    all_consuming(many0(terminated(sensor_reading, newline)))(input)
}

fn sensor_reading(input: &str) -> IResult<&str, SensorReading> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, pos_x) = i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, pos_y) = i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = i32(input)?;
    Ok((
        input,
        SensorReading::new(Point(pos_x, pos_y), Point(beacon_x, beacon_y)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sensor_reading() {
        let (_, reading) =
            sensor_reading("Sensor at x=13, y=-2: closest beacon is at x=15, y=3").unwrap();
        assert_eq!(reading, SensorReading::new(Point(13, -2), Point(15, 3)));
    }

    #[test]
    fn parse_multiple_sensor_reading() {
        let (_, reading) =
            parse("Sensor at x=13, y=-2: closest beacon is at x=15, y=3\nSensor at x=14, y=17: closest beacon is at x=10, y=16\n").unwrap();
        assert_eq!(
            reading,
            [
                SensorReading::new(Point(13, -2), Point(15, 3)),
                SensorReading::new(Point(14, 17), Point(10, 16)),
            ]
        )
    }

    #[test]
    fn point_distance_x_and_y_different() {
        assert_eq!(Point(8, 7).distance_to(&Point(2, 10)), 9);
    }

    #[test]
    fn point_distance_equal_points() {
        assert_eq!(Point(8, 7).distance_to(&Point(8, 7)), 0);
    }

    #[test]
    fn point_distance_x_different() {
        assert_eq!(Point(8, 7).distance_to(&Point(9, 7)), 1);
        assert_eq!(Point(8, 7).distance_to(&Point(7, 7)), 1);
    }

    #[test]
    fn point_distance_y_different() {
        assert_eq!(Point(8, 7).distance_to(&Point(8, 8)), 1);
        assert_eq!(Point(8, 7).distance_to(&Point(8, 6)), 1);
    }
}
