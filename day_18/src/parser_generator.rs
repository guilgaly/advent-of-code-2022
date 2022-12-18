use peg::error::ParseError;
use peg::str::LineCol;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub fn parse_positions(input: &str) -> Result<HashSet<Pos>, ParseError<LineCol>> {
    let list = parser::positions(input)?;
    Ok(HashSet::from_iter(list.iter().copied()))
}

peg::parser! {
    grammar parser() for str {
        rule int() -> i64 = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule position() -> Pos = x:int() "," y:int() "," z:int() { Pos { x, y, z } }
        pub rule positions() -> Vec<Pos> = n:(position() ** "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paths() {
        assert_eq!(
            parser::positions(
                "2,2,2
1,2,2
3,2,2"
            )
            .unwrap(),
            vec![
                Pos { x: 2, y: 2, z: 2 },
                Pos { x: 1, y: 2, z: 2 },
                Pos { x: 3, y: 2, z: 2 },
            ]
        );
    }
}
