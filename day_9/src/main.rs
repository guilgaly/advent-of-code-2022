use std::collections::HashSet;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let moves = parse_input(INPUT)?;

    println!("Part 1 result: {}", part_1(&moves));
    println!("Part 2 result: {}", part_2(&moves));

    Ok(())
}

type Pos = (i64, i64);

#[derive(Debug, PartialEq)]
struct Move(Direction, i64);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new_pos(&self, (from_x, from_y): Pos) -> Pos {
        match self {
            Direction::Up => (from_x, from_y + 1),
            Direction::Down => (from_x, from_y - 1),
            Direction::Left => (from_x - 1, from_y),
            Direction::Right => (from_x + 1, from_y),
        }
    }
}

fn move_tail((x_tail, y_tail): Pos, (x_head, y_head): Pos) -> Pos {
    if is_adjacent(x_tail, x_head) && is_adjacent(y_tail, y_head) {
        (x_tail, y_tail)
    } else if x_tail == x_head {
        (x_tail, calculate_move(y_tail, y_head))
    } else if y_tail == y_head {
        (calculate_move(x_tail, x_head), y_tail)
    } else if is_adjacent(x_tail, x_head) {
        (x_head, calculate_move(y_tail, y_head))
    } else if is_adjacent(y_tail, y_head) {
        (calculate_move(x_tail, x_head), y_head)
    } else {
        (calculate_move(x_tail, x_head), calculate_move(y_tail, y_head))
    }
}

fn is_adjacent(x1: i64, x2: i64) -> bool {
    let diff = x1 -x2;
    -1 <= diff && diff <= 1
}

fn calculate_move(from: i64, target: i64) -> i64 {
    let distance = target - from;
    if distance > 0 {
        from + distance - 1
    } else if distance < 0 {
        from + distance + 1
    } else {
        from
    }
}

fn part_1(moves: &[Move]) -> usize {
    let mut previous_tail_positions = HashSet::new();
    previous_tail_positions.insert((0, 0));

    let mut current_head_position: Pos = (0, 0);
    let mut current_tail_position: Pos = (0, 0);

    for Move(direction, distance) in moves {
        for _ in 0..*distance {
            current_head_position = direction.new_pos(current_head_position);
            current_tail_position = move_tail(current_tail_position, current_head_position);
            previous_tail_positions.insert(current_tail_position);
            println!(
                "head {:?}, tail {:?}",
                current_head_position, current_tail_position
            );
        }
        println!();
    }
    previous_tail_positions.len()
}

fn part_2(moves: &[Move]) -> usize {
    let mut previous_tail_positions = HashSet::new();
    previous_tail_positions.insert((0, 0));

    let mut current_positions = [(0,0); 10];
    for Move(direction, distance) in moves {
        for _ in 0..*distance {
            current_positions[0] = direction.new_pos(current_positions[0]); // move head
            for i in 1..10 {
                current_positions[i] = move_tail(current_positions[i], current_positions[i - 1]);
                previous_tail_positions.insert(current_positions[9]);
            }
            println!(
                "positions {:?}",
                current_positions
            );
        }
    }
    previous_tail_positions.len()
}

// fn part_1(forest: &Forest) -> usize {
// }
//
// fn part_2(forest: &Forest) -> usize {
// }

fn parse_input(input: &str) -> Result<Vec<Move>, String> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let part_1 = split.next().ok_or(format!("Invalid {}", line))?;
            let distance = split
                .next()
                .ok_or(format!("Invalid {}", line))?
                .parse::<i64>()
                .map_err(|_| format!("Invalid {}", line))?;
            match part_1 {
                "U" => Ok(Move(Direction::Up, distance)),
                "D" => Ok(Move(Direction::Down, distance)),
                "L" => Ok(Move(Direction::Left, distance)),
                "R" => Ok(Move(Direction::Right, distance)),
                _ => Err(format!("Invalid {}", line)),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_moves() -> Vec<Move> {
        let test_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        parse_input(test_input).unwrap()
    }

    #[test]
    fn test_parse_input() {
        let expected = vec![
            Move(Direction::Right, 4),
            Move(Direction::Up, 4),
            Move(Direction::Left, 3),
            Move(Direction::Down, 1),
            Move(Direction::Right, 4),
            Move(Direction::Down, 1),
            Move(Direction::Left, 5),
            Move(Direction::Right, 2),
        ];
        assert_eq!(test_moves(), expected)
    }

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(part_1(&test_moves()), 13);
    // }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_moves()), 1);
    }
}
