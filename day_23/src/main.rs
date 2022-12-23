use common::itertools::Itertools;
use common::time_execution;
use std::collections::{HashMap, HashSet};
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let elves_positions = parse_elves_positions(INPUT);
    println!(
        "Part 1 result: {}",
        time_execution("Part 1", || part_1(&elves_positions))
    );

    println!(
        "Part 2 result: {}",
        time_execution("Part 2", || part_2(&elves_positions))
    );

    Ok(())
}

fn part_1(init_positions: &[Pos]) -> usize {
    let mut positions: HashSet<Pos> = init_positions.iter().copied().collect();
    let mut directions = [Direction::N, Direction::S, Direction::W, Direction::E];

    for _ in 1..=10 {
        play_round(&mut positions, &mut directions);
    }

    let Rectangle { x_min, x_max, y_min, y_max } = enclosing_rectangle(&positions);
    (y_min..=y_max)
        .into_iter()
        .flat_map(|y| (x_min..=x_max).into_iter().map(move |x| Pos { x, y }))
        .filter(|pos| !positions.contains(pos))
        .count()
}

fn part_2(init_positions: &[Pos]) -> usize {
    let mut positions: HashSet<Pos> = init_positions.iter().copied().collect();
    let mut directions = [Direction::N, Direction::S, Direction::W, Direction::E];

    let mut round = 0;
    loop {
        round += 1;
        let prev_positions = positions.clone();
        play_round(&mut positions, &mut directions);
        if positions == prev_positions {
            return round;
        }
    }
}

fn play_round(positions: &mut HashSet<Pos>, directions: &mut [Direction; 4]) {
    let moves = positions
        .iter()
        .map(|&pos| calculate_move(pos, positions, directions))
        .collect_vec();
    let mut dest_counts: HashMap<Pos, usize> = HashMap::new();
    for m in moves.iter() {
        *dest_counts.entry(m.to).or_default() += 1;
    }
    let conflicts: HashSet<Pos> = dest_counts
        .into_iter()
        .filter_map(|(pos, count)| if count > 1 { Some(pos) } else { None })
        .collect();
    *positions = moves
        .into_iter()
        .map(|Move { from, to }| if conflicts.contains(&to) { from } else { to })
        .collect();
    *directions = [directions[1], directions[2], directions[3], directions[0]];
}

fn calculate_move(from: Pos, positions: &HashSet<Pos>, directions: &[Direction]) -> Move {
    let nbr_n = neighbor_n(from, positions);
    let nbr_ne = neighbor_ne(from, positions);
    let nbr_e = neighbor_e(from, positions);
    let nbr_se = neighbor_se(from, positions);
    let nbr_s = neighbor_s(from, positions);
    let nbr_sw = neighbor_sw(from, positions);
    let nbr_w = neighbor_w(from, positions);
    let nbr_nw = neighbor_nw(from, positions);

    let to = if !(nbr_n || nbr_ne || nbr_e || nbr_se || nbr_s || nbr_sw || nbr_w || nbr_nw) {
        from
    } else {
        directions
            .iter()
            .find_map(|dir| match dir {
                Direction::N if !(nbr_n || nbr_nw || nbr_ne) => Some(from.north()),
                Direction::S if !(nbr_s || nbr_sw || nbr_se) => Some(from.south()),
                Direction::W if !(nbr_w || nbr_nw || nbr_sw) => Some(from.west()),
                Direction::E if !(nbr_e || nbr_ne || nbr_se) => Some(from.east()),
                _ => None,
            })
            .unwrap_or(from)
    };

    Move { from, to }
}

fn print_positions(positions: &HashSet<Pos>) {
    let Rectangle { x_min, x_max, y_min, y_max } = enclosing_rectangle(positions);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if positions.contains(&Pos { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn enclosing_rectangle(positions: &HashSet<Pos>) -> Rectangle {
    Rectangle {
        x_min: positions.iter().map(|p| p.x).min().unwrap(),
        x_max: positions.iter().map(|p| p.x).max().unwrap(),
        y_min: positions.iter().map(|p| p.y).min().unwrap(),
        y_max: positions.iter().map(|p| p.y).max().unwrap(),
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Move {
    from: Pos,
    to: Pos,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Rectangle {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

fn neighbor_n(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&pos.north())
}
fn neighbor_ne(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&Pos { x: pos.x + 1, y: pos.y - 1 })
}
fn neighbor_e(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&pos.east())
}
fn neighbor_se(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&Pos { x: pos.x + 1, y: pos.y + 1 })
}
fn neighbor_s(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&pos.south())
}
fn neighbor_sw(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&Pos { x: pos.x - 1, y: pos.y + 1 })
}
fn neighbor_w(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&pos.west())
}
fn neighbor_nw(pos: Pos, all_pos: &HashSet<Pos>) -> bool {
    all_pos.contains(&Pos { x: pos.x - 1, y: pos.y - 1 })
}

impl Pos {
    fn north(&self) -> Pos {
        Pos { x: self.x, y: self.y - 1 }
    }
    fn south(&self) -> Pos {
        Pos { x: self.x, y: self.y + 1 }
    }
    fn west(&self) -> Pos {
        Pos { x: self.x - 1, y: self.y }
    }
    fn east(&self) -> Pos {
        Pos { x: self.x + 1, y: self.y }
    }
}

fn parse_elves_positions(input: &str) -> Vec<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| {
                if c == '#' {
                    Some(Pos { x: x as i64, y: y as i64 })
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashset;

    #[test]
    fn test_parse_elves_positions() {
        assert_eq!(
            HashSet::from_iter(
                parse_elves_positions(
                    "......#.....
..........#.
.#.#..#....."
                )
                .into_iter()
            ),
            hashset! {
                Pos { x: 6, y: 0 },
                Pos { x: 10, y: 1 },
                Pos { x: 1, y: 2 },
                Pos { x: 3, y: 2 },
                Pos { x: 6, y: 2 },
            }
        )
    }

    fn test_init_positions() -> Vec<Pos> {
        parse_elves_positions(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_init_positions()), 110);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_init_positions()), 20);
    }
}
