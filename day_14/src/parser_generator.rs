use peg::error::ParseError;
use peg::str::LineCol;

pub fn parse_paths(input: &str) -> Result<Vec<Path>, ParseError<LineCol>> {
    parser::paths(input)
}

pub type Path = Vec<Point>;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

peg::parser! {
    grammar parser() for str {
        rule int() -> i64 = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule point() -> Point = x:int() "," y:int() { Point { x, y } }
        rule path() -> Path = n:(point() ** " -> ")
        pub rule paths() -> Vec<Path> = n:(path() ** "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paths() {
        assert_eq!(
            parser::paths(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            )
            .unwrap(),
            vec![
                vec![
                    Point { x: 498, y: 4 },
                    Point { x: 498, y: 6 },
                    Point { x: 496, y: 6 }
                ],
                vec![
                    Point { x: 503, y: 4 },
                    Point { x: 502, y: 4 },
                    Point { x: 502, y: 9 },
                    Point { x: 494, y: 9 }
                ]
            ]
        );
    }
}
