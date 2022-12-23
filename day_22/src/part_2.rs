use crate::instructions::Instr;
use crate::tiles::{password, Direction, Position, Tile};

pub fn part_2(tiles: &Vec<Vec<Tile>>, instructions: &[Instr]) -> usize {
    let mut pos = Position { x: 50, y: 0 };
    let mut dir = Direction::Right;
    for instruction in instructions {
        match instruction {
            Instr::Forward(length) => (pos, dir) = move_forward(pos, dir, *length, tiles),
            Instr::Left => dir = dir.rotate_left(),
            Instr::Right => dir = dir.rotate_right(),
        }
    }
    password(pos, dir)
}

fn move_forward(
    init_pos: Position,
    init_dir: Direction,
    length: usize,
    tiles: &Vec<Vec<Tile>>,
) -> (Position, Direction) {
    (0..length)
        .into_iter()
        .fold((init_pos, init_dir), |(prev_pos, prev_dir), _| {
            move_forward_once(prev_pos, prev_dir, tiles)
        })
}

fn move_forward_once(
    init_pos @ Position { x, y }: Position,
    init_dir: Direction,
    tiles: &Vec<Vec<Tile>>,
) -> (Position, Direction) {
    let (target_pos, target_dir) = match init_dir {
        Direction::Up => {
            if y == 100 && x <= 49 {
                (Position { x: 50, y: x + 50 }, Direction::Right)
            } else if y == 0 && x <= 99 {
                (Position { x: 0, y: x + 100 }, Direction::Right)
            } else if y == 0 && x >= 100 {
                (Position { x: x - 100, y: 199 }, Direction::Up)
            } else {
                (Position { x, y: y - 1 }, init_dir)
            }
        }
        Direction::Right => {
            if x == 149 {
                (Position { x: 99, y: 149 - y }, Direction::Left)
            } else if x == 99 && y >= 50 && y <= 99 {
                (Position { x: y + 50, y: 49 }, Direction::Up)
            } else if x == 99 && y >= 100 && y <= 149 {
                (Position { x: 149, y: 149 - y }, Direction::Left)
            } else if x == 49 && y >= 150 {
                (Position { x: y - 100, y: 149 }, Direction::Up)
            } else {
                (Position { x: x + 1, y }, init_dir)
            }
        }
        Direction::Down => {
            if y == 199 && x <= 49 {
                (Position { x: x + 100, y: 0 }, Direction::Down)
            } else if y == 149 && x >= 50 && x <= 99 {
                (Position { x: 49, y: x + 100 }, Direction::Left)
            } else if y == 49 && x >= 100 && x <= 149 {
                (Position { x: 99, y: x - 50 }, Direction::Left)
            } else {
                (Position { x, y: y + 1 }, init_dir)
            }
        }
        Direction::Left => {
            if x == 50 && y <= 49 {
                (Position { x: 0, y: 149 - y }, Direction::Right)
            } else if x == 50 && y >= 50 && y <= 99 {
                (Position { x: y - 50, y: 100 }, Direction::Down)
            } else if x == 0 && y >= 100 && y <= 149 {
                (Position { x: 50, y: 149 - y }, Direction::Right)
            } else if x == 0 && y >= 150 {
                (Position { x: y - 100, y: 0 }, Direction::Down)
            } else {
                (Position { x: x - 1, y }, init_dir)
            }
        }
    };
    let tile = tiles[target_pos.y][target_pos.x];
    match tile {
        Tile::Wall => (init_pos, init_dir),
        Tile::Open => (target_pos, target_dir),
        Tile::Empty => panic!("Empty target {:?}", target_pos),
    }
}
