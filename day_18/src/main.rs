use common::itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::error::Error;

use crate::parser_generator::{parse_positions, Pos};

mod parser_generator;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let cubes = parse_positions(INPUT)?;

    println!("Part 1 result: {}", part_1(&cubes));
    println!("Part 2 result: {}", part_2(&cubes));

    Ok(())
}

fn part_1(cubes: &HashSet<Pos>) -> i64 {
    cubes.iter().fold(0, |count, p| {
        count
            + neighbors(*p)
                .iter()
                .fold(0, |acc, n| if cubes.contains(n) { acc } else { acc + 1 })
    })
}

fn part_2(initial_cubes: &HashSet<Pos>) -> i64 {
    let mut cubes = initial_cubes.clone();
    let initial_surface_area = part_1(&cubes);

    let min_x = cubes.iter().map(|cube| cube.x).min().unwrap() - 2;
    let max_x = cubes.iter().map(|cube| cube.x).max().unwrap() + 2;
    let min_y = cubes.iter().map(|cube| cube.y).min().unwrap() - 2;
    let max_y = cubes.iter().map(|cube| cube.y).max().unwrap() + 2;
    let min_z = cubes.iter().map(|cube| cube.z).min().unwrap() - 2;
    let max_z = cubes.iter().map(|cube| cube.z).max().unwrap() + 2;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            cubes.insert(Pos { x, y, z: min_z });
            cubes.insert(Pos { x, y, z: max_z });
        }
    }
    for x in min_x..=max_x {
        for z in min_z..=max_z {
            cubes.insert(Pos { x, y: min_y, z });
            cubes.insert(Pos { x, y: max_y, z });
        }
    }
    for y in min_y..=max_y {
        for z in min_z..=max_z {
            cubes.insert(Pos { x: min_x, y, z });
            cubes.insert(Pos { x: max_x, y, z });
        }
    }

    let start = Pos { x: min_x + 1, y: min_y + 1, z: min_z + 1 };
    let mut queue = vec![start];
    while let Some(cube) = queue.pop() {
        if cubes.insert(cube) {
            for neighbor in neighbors(cube) {
                if !cubes.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }
    }

    let expected_new_external_surface_area = 2 * (max_x - min_x + 1) * (max_y - min_y + 1)
        + 2 * (max_x - min_x + 1) * (max_z - min_z + 1)
        + 2 * (max_y - min_y + 1) * (max_z - min_z + 1);
    let internal_surface_area = part_1(&cubes) - expected_new_external_surface_area;
    initial_surface_area - internal_surface_area
}

fn neighbors(Pos { x, y, z }: Pos) -> [Pos; 6] {
    [
        Pos { x: x - 1, y, z },
        Pos { x: x + 1, y, z },
        Pos { x, y: y - 1, z },
        Pos { x, y: y + 1, z },
        Pos { x, y, z: z - 1 },
        Pos { x, y, z: z + 1 },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_positions() -> HashSet<Pos> {
        parse_positions(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
        )
        .unwrap()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_positions()), 64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_positions()), 58);
    }
}
