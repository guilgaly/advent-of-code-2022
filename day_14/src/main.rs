extern crate core;

use crate::parser_generator::{parse_paths, Path, Point};
use common::itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;
use std::iter::Iterator;

mod parser_generator;

static INPUT: &str = include_str!("input");
static SAND_ORIGIN: Point = Point { x: 500, y: 0 };

fn main() -> Result<(), Box<dyn Error>> {
    let paths = parse_paths(INPUT)?;
    let cave = build_cave(&paths);
    println!("Part 1 result: {}", part_1(&cave));
    println!("Part 2 result: {}", part_2(&cave));

    Ok(())
}

fn part_2(init_cave: &Cave) -> usize {
    let mut cave = init_cave.clone();
    let mut counter = 0;
    while pour_sand_once_2(&mut cave, init_cave.max_y() + 2) {
        counter += 1;
    }
    print_cave(&cave);
    counter + 1
}

fn pour_sand_once_2(cave: &mut Cave, max_y: i64) -> bool {
    fn move_sand_from(p: Point, cave: &Cave, max_y: i64) -> Option<Point> {
        let try_point= |x: i64, y: i64| {
            if cave.get(x, y) == Cell::Empty { Some(Point { x, y }) } else { None }
        };

        if let Some(new_point) = try_point(p.x, p.y + 1).or_else(|| try_point(p.x - 1, p.y + 1)).or_else(|| try_point(p.x + 1, p.y + 1)) {
            if new_point.y == max_y {
                Some(p)
            } else {
                move_sand_from(new_point, cave, max_y)
            }
        } else if p == SAND_ORIGIN {
            None
        } else {
            Some(p)
        }
    }
    if let Some(settled_point) = move_sand_from(SAND_ORIGIN, cave, max_y) {
        cave.sand.insert(settled_point);
        true
    } else {
        false
    }
}

fn part_1(init_cave: &Cave) -> usize {
    let mut cave = init_cave.clone();
    let mut counter = 0;
    while pour_sand_once(&mut cave) {
        counter += 1;
    }
    print_cave(&cave);
    counter
}

fn pour_sand_once(cave: &mut Cave) -> bool {
    fn move_sand_from(p: Point, cave: &Cave, max_y: i64) -> Option<Point> {
        let try_point= |x: i64, y: i64| {
            if cave.get(x, y) == Cell::Empty { Some(Point { x, y }) } else { None }
        };
        if p.y == max_y {
            None
        } else {
            if let Some(new_point) = try_point(p.x, p.y + 1).or_else(|| try_point(p.x - 1, p.y + 1)).or_else(|| try_point(p.x + 1, p.y + 1)) {
                move_sand_from(new_point, cave, max_y)
            } else {
                Some(p)
            }
        }
    }
    if let Some(settled_point) = move_sand_from(SAND_ORIGIN, cave, cave.max_y()) {
        cave.sand.insert(settled_point);
        true
    } else {
        false
    }
}

#[derive(Clone)]
struct Cave {
    walls: HashSet<Point>,
    sand: HashSet<Point>,
}

impl Cave {
    fn get(&self, x: i64, y: i64) -> Cell {
        let point = Point {x, y};
        if self.walls.contains(&point) {
            Cell::Wall
        } else if self.sand.contains(&point) {
            Cell::Sand
        } else {
            Cell::Empty
        }
    }
    fn min_x(&self) -> i64 {
        self.walls
            .iter()
            .chain(self.sand.iter())
            .map(|p| p.x)
            .min()
            .unwrap()
    }
    fn max_x(&self) -> i64 {
        self.walls
            .iter()
            .chain(self.sand.iter())
            .map(|p| p.x)
            .max()
            .unwrap()
    }
    fn min_y(&self) -> i64 {
        self.walls
            .iter()
            .chain(self.sand.iter())
            .map(|p| p.y)
            .min()
            .unwrap()
    }
    fn max_y(&self) -> i64 {
        self.walls
            .iter()
            .chain(self.sand.iter())
            .map(|p| p.y)
            .max()
            .unwrap()
    }
}

#[derive(PartialEq, Debug)]
enum Cell {
    Empty,
    Wall,
    Sand,
}

fn build_cave(paths: &[Path]) -> Cave {
    let walls: HashSet<Point> = paths
        .iter()
        .flat_map(|path| {
            path.iter()
                .tuple_windows::<(_, _)>()
                .flat_map(|(p1, p2)| wall_points(*p1, *p2))
        })
        .collect();
    Cave { walls, sand: HashSet::new() }
}

fn wall_points(p1: Point, p2: Point) -> Vec<Point> {
    if p1.x == p2.x {
        (min(p1.y, p2.y)..=max(p1.y, p2.y))
            .map(|y| Point { x: p1.x, y })
            .collect()
    } else if p1.y == p2.y {
        (min(p1.x, p2.x)..=max(p1.x, p2.x))
            .map(|x| Point { x, y: p1.y })
            .collect()
    } else {
        panic!("Diagonal lines not supported")
    }
}

fn print_cave(cave: &Cave) {
    for y in 0..=cave.max_y() {
        for x in cave.min_x()..=cave.max_x() {
            match cave.get(x, y) {
                Cell::Empty => {
                    if x == SAND_ORIGIN.x && y == SAND_ORIGIN.y {
                        print!("+")
                    } else {
                        print!(".")
                    }
                }
                Cell::Wall => print!("#"),
                Cell::Sand => print!("o"),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_paths() -> Vec<Path> {
        parse_paths("498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9").unwrap()
    }

    fn test_cave() -> Cave {
        build_cave(&test_paths())
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_cave()), 24);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_cave()), 93);
    }
}
