use crate::instructions::Instr;
use crate::tiles::{password, Direction, Position, Tile};

pub fn part_1(tiles: &Vec<Vec<Tile>>, instructions: &[Instr]) -> usize {
    let board = Board::new(tiles.clone());
    let mut pos = board.start_pos();
    let mut dir = Direction::Right;
    for instruction in instructions {
        match instruction {
            Instr::Forward(length) => pos = move_forward(pos, dir, *length, &board),
            Instr::Left => dir = dir.rotate_left(),
            Instr::Right => dir = dir.rotate_right(),
        }
    }
    password(pos, dir)
}

fn move_forward(init_pos: Position, dir: Direction, length: usize, board: &Board) -> Position {
    (0..length)
        .into_iter()
        .fold(init_pos, |prev, _| move_forward_once(prev, dir, board))
}

fn move_forward_once(init_pos: Position, dir: Direction, board: &Board) -> Position {
    fn recurs(Position { x, y }: Position, dir: Direction, board: &Board) -> Option<Position> {
        let target_pos = match dir {
            Direction::Up => Position { x, y: if y == 0 { board.max_y } else { y - 1 } },
            Direction::Right => Position { x: if x == board.max_x { 0 } else { x + 1 }, y },
            Direction::Down => Position { x, y: if y == board.max_y { 0 } else { y + 1 } },
            Direction::Left => Position { x: if x == 0 { board.max_x } else { x - 1 }, y },
        };
        let tile = board.tiles[target_pos.y][target_pos.x];
        // println!("init_pos: {:?}, dir: {:?}, target_pos: {:?}, tile: {:?}", init_pos, dir, target_pos, tile);
        match tile {
            Tile::Wall => None,
            Tile::Open => Some(target_pos),
            Tile::Empty => recurs(target_pos, dir, board),
        }
    }
    recurs(init_pos, dir, board).unwrap_or(init_pos)
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    max_x: usize,
    max_y: usize,
}

impl Board {
    fn new(tiles: Vec<Vec<Tile>>) -> Board {
        let max_x = tiles[0].len() - 1;
        let max_y = tiles.len() - 1;
        Board { tiles, max_x, max_y }
    }
    fn start_pos(&self) -> Position {
        Position {
            x: self.tiles[0].iter().position(|&t| t == Tile::Open).unwrap(),
            y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tiles() -> Vec<Vec<Tile>> {
        parse_tiles(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.",
        )
        .unwrap()
    }

    fn test_instructions() -> Vec<Instr> {
        parse_instructions("10R5L5R10L4R5L5").unwrap()
    }

    #[test]
    fn test_move_forward_once() {
        let board = Board::new(test_tiles());
        assert_eq!(
            move_forward_once(Position { x: 11, y: 6 }, Direction::Right, &board),
            Position { x: 0, y: 6 }
        );
        assert_eq!(
            move_forward_once(Position { x: 11, y: 6 }, Direction::Left, &board),
            Position { x: 10, y: 6 }
        );
        assert_eq!(
            move_forward_once(Position { x: 5, y: 7 }, Direction::Down, &board),
            Position { x: 5, y: 4 }
        );
        assert_eq!(
            move_forward_once(Position { x: 5, y: 7 }, Direction::Up, &board),
            Position { x: 5, y: 6 }
        );
        assert_eq!(
            move_forward_once(Position { x: 2, y: 7 }, Direction::Up, &board),
            Position { x: 2, y: 7 }
        );
        assert_eq!(
            move_forward_once(Position { x: 3, y: 7 }, Direction::Down, &board),
            Position { x: 3, y: 7 }
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_tiles(), &test_instructions()), 6032)
    }
}
