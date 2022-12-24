use common::itertools::Itertools;
use common::time_execution;
use maplit::hashset;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::repeat;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let (blizzards, max_x, max_y) = parse_input(INPUT)?;

    println!(
        "Part 1 result: {}",
        time_execution("Part 1", || part_1(&blizzards, max_x, max_y))
    );

    println!(
        "Part 2 result: {}",
        time_execution("Part 2", || part_2(&blizzards, max_x, max_y))
    );

    Ok(())
}

fn part_1(init_blizzards: &[Blizzard], max_x: usize, max_y: usize) -> usize {
    let start = Pos { x: 1, y: 0 };
    let goal = Pos { x: max_x, y: max_y + 1 };

    travel(start, goal, init_blizzards, max_x, max_y).0
}

fn part_2(init_blizzards: &[Blizzard], max_x: usize, max_y: usize) -> usize {
    let start = Pos { x: 1, y: 0 };
    let goal = Pos { x: max_x, y: max_y + 1 };

    let (dur1, blizzards_1) = travel(start, goal, init_blizzards, max_x, max_y);
    let (dur2, blizzards_2) = travel(goal, start, &blizzards_1, max_x, max_y);
    let (dur3, _) = travel(start, goal, &blizzards_2, max_x, max_y);

    dur1 + dur2 + dur3
}

fn travel(
    from: Pos,
    to: Pos,
    starting_blizzards: &[Blizzard],
    max_x: usize,
    max_y: usize,
) -> (usize, Vec<Blizzard>) {
    let mut blizzards = starting_blizzards.iter().cloned().collect_vec();
    let mut possible_pos = hashset! { from };
    let mut depth = 0;

    loop {
        depth += 1;
        blizzards = move_blizzards(&blizzards, max_x, max_y);
        let blocked_moves: HashSet<Pos> = blizzards.iter().map(|b| b.pos).collect();
        possible_pos = possible_pos
            .iter()
            .flat_map(|from| {
                possible_moves(*from, max_x, max_y)
                    .into_iter()
                    .filter(|p| !blocked_moves.contains(p))
            })
            .collect();
        if possible_pos.contains(&to) {
            return (depth, blizzards);
        }
    }
}

fn move_blizzards(blizzards: &[Blizzard], max_x: usize, max_y: usize) -> Vec<Blizzard> {
    blizzards
        .iter()
        .map(|b| move_blizzard(b, max_x, max_y))
        .collect_vec()
}

fn move_blizzard(blizzard: &Blizzard, max_x: usize, max_y: usize) -> Blizzard {
    let Pos { x, y } = blizzard.pos;
    let pos = match blizzard.dir {
        Dir::N => {
            let y = if y == 1 { max_y } else { y - 1 };
            Pos { x, y }
        }
        Dir::S => {
            let y = if y == max_y { 1 } else { y + 1 };
            Pos { x, y }
        }
        Dir::W => {
            let x = if x == 1 { max_x } else { x - 1 };
            Pos { x, y }
        }
        Dir::E => {
            let x = if x == max_x { 1 } else { x + 1 };
            Pos { x, y }
        }
    };
    Blizzard { pos, dir: blizzard.dir }
}

fn possible_moves(current_pos @ Pos { x, y }: Pos, max_x: usize, max_y: usize) -> Vec<Pos> {
    if x == 1 && y == 0 {
        // from start point
        vec![current_pos, Pos { x: 1, y: 1 }]
    } else if x == max_x && y == max_y + 1 {
        // from goal point
        vec![current_pos, Pos { x: max_x, y: max_y }]
    } else {
        let mut pm = vec![current_pos];
        if x > 1 {
            pm.push(Pos { x: x - 1, y });
        }
        if x < max_x {
            pm.push(Pos { x: x + 1, y });
        }
        if y > 1 {
            pm.push(Pos { x, y: y - 1 });
        }
        if y < max_y {
            pm.push(Pos { x, y: y + 1 });
        }
        if x == 1 && y == 1 {
            pm.push(Pos { x: 1, y: 0 });
        }
        if x == max_x && y == max_y {
            pm.push(Pos { x, y: y + 1 });
        }
        pm
    }
}

/// Returns (blizzards, max_x, max_y)
fn parse_input(input: &str) -> Result<(Vec<Blizzard>, usize, usize), String> {
    let max_y = input.lines().count();
    let max_x = input.lines().next().ok_or("Empty input")?.chars().count();

    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| {
                let pos = Pos { x: x + 1, y: y + 1 };
                match c {
                    '^' => Some(Blizzard { pos, dir: Dir::N }),
                    'v' => Some(Blizzard { pos, dir: Dir::S }),
                    '<' => Some(Blizzard { pos, dir: Dir::W }),
                    '>' => Some(Blizzard { pos, dir: Dir::E }),
                    _ => None,
                }
            })
        })
        .collect_vec();

    Ok((blizzards, max_x, max_y))
}

// Note: inside the valley, x from 1 until max_x; y from 1 until max_y.
// y=0 reserved for the starting point: Pos { x: 0, y: 0 }
// y=max_y+1 reserved for the destination point: Pos { x: max_x, y: max_y + 1 }
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Valley {
    max_x: usize,
    max_y: usize,
    expedition: Pos,
    blizzards: Vec<Blizzard>,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Blizzard {
    pos: Pos,
    dir: Dir,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_valley() -> (Vec<Blizzard>, usize, usize) {
        parse_input(
            ">>.<^<
.<..<<
>v.><>
<^v^^>",
        )
        .unwrap()
    }

    #[test]
    fn test_part_1() {
        let (blizzards, max_x, max_y) = test_valley();
        assert_eq!(part_1(&blizzards, max_x, max_y), 18);
    }

    #[test]
    fn test_part_2() {
        let (blizzards, max_x, max_y) = test_valley();
        assert_eq!(part_2(&blizzards, max_x, max_y), 54);
    }
}
