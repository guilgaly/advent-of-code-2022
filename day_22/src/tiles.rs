use common::itertools::Itertools;
use peg::error::ParseError;
use peg::str::LineCol;
use std::iter::repeat;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub fn password(pos: Position, dir: Direction) -> usize {
    let facing = match dir {
        Direction::Up => 3,
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
    };
    1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Tile {
    Open,
    Wall,
    Empty,
}

pub fn parse_tiles(input: &str) -> Result<Vec<Vec<Tile>>, ParseError<LineCol>> {
    tiles_parser::tiles(input)
}

peg::parser! {
    grammar tiles_parser() for str {
        rule open() -> Tile = "." { Tile::Open }
        rule wall() -> Tile = "#" { Tile::Wall }
        rule empty() -> Tile = " " { Tile::Empty }
        rule tile() -> Tile = open() / wall() / empty()
        rule row() -> Vec<Tile> = (tile() ** "")
        pub rule tiles() -> Vec<Vec<Tile>> = n:(row() ** "\n") { normalize_lengths(n) }
    }
}

fn normalize_lengths(tiles: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let length = tiles.iter().map(|row| row.len()).max().unwrap();
    tiles
        .into_iter()
        .map(|row| {
            let diff = length - row.len();
            row.into_iter()
                .chain(repeat(Tile::Empty).take(diff))
                .collect_vec()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tiles() {
        assert_eq!(
            parse_tiles(
                "  #.
  ..
...#
.."
            )
            .unwrap(),
            vec![
                vec![Tile::Empty, Tile::Empty, Tile::Wall, Tile::Open],
                vec![Tile::Empty, Tile::Empty, Tile::Open, Tile::Open],
                vec![Tile::Open, Tile::Open, Tile::Open, Tile::Wall],
                vec![Tile::Open, Tile::Open, Tile::Empty, Tile::Empty],
            ]
        )
    }
}
