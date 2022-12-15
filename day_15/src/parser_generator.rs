use crate::sensors::{Point, Sensor};
use peg::error::ParseError;
use peg::str::LineCol;

pub fn parse_sensors(input: &str) -> Result<Vec<Sensor>, ParseError<LineCol>> {
    parser::sensors(input)
}

peg::parser! {
    grammar parser() for str {
        rule int() -> i64 = n:$("-"? ['0'..='9']+) { n.parse().unwrap() }
        rule point() -> Point = "x=" x:int() ", y=" y:int() { Point { x, y } }
        rule sensor() -> Sensor = "Sensor at " position:point() ": closest beacon is at " closest_beacon:point() {Sensor::new(position, closest_beacon)}
        pub rule sensors() -> Vec<Sensor> = n:(sensor() ** "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paths() {
        assert_eq!(
            parser::sensors(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16"
            )
            .unwrap(),
            vec![
                Sensor {
                    position: Point { x: 2, y: 18 },
                    closest_beacon: Point { x: -2, y: 15 },
                    beacon_distance: 7
                },
                Sensor {
                    position: Point { x: 9, y: 16 },
                    closest_beacon: Point { x: 10, y: 16 },
                    beacon_distance: 1
                },
            ]
        );
    }
}
