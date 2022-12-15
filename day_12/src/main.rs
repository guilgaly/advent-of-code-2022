use ascii::{AsAsciiStr, AsAsciiStrError, AsciiChar};
use std::collections::{HashSet, VecDeque};
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let map = parse_input(INPUT)?;
    println!("Part 1 result: {}", part_1(&map)?);
    println!("Part 2 result: {}", part_2(&map)?);

    Ok(())
}

fn part_1(map: &HeightMap) -> Result<usize, String> {
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();

    explored.insert(map.start);
    queue.push_back((0, map.start));
    while let Some((depth, current_point)) = queue.pop_front() {
        if current_point == map.end {
            return Ok(depth);
        }
        for neighbor in map.accessible_neighbors(current_point) {
            if !explored.contains(&neighbor) {
                explored.insert(neighbor);
                queue.push_back((depth + 1, neighbor));
            }
        }
    }
    Err("Not found".to_owned())
}

fn part_2(map: &HeightMap) -> Result<usize, String> {
    // Smarter solution would be to search from the end...
    (0..=map.x_max)
        .flat_map(|x| (0..=map.y_max).map(move |y| Coords { x, y }))
        .filter(|coord| map.get(*coord) == AsciiChar::a)
        .flat_map(|start| part_1(&HeightMap { start, ..map.clone() }).ok())
        .min()
        .ok_or("No result".to_owned())
}

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct HeightMap {
    squares: Vec<Vec<AsciiChar>>,
    start: Coords,
    end: Coords,
    x_max: usize,
    y_max: usize,
}

impl HeightMap {
    fn get(&self, coords: Coords) -> AsciiChar {
        self.squares[coords.y][coords.x]
    }
    fn accessible_neighbors(&self, Coords { x, y }: Coords) -> Vec<Coords> {
        let current_height = self.get(Coords { x, y }).as_byte();
        let mut directions = vec![];
        let mut maybe_add = |dest: Coords| {
            let dest_height = self.get(dest).as_byte();
            if dest_height <= current_height || dest_height == current_height + 1 {
                directions.push(dest);
            }
        };
        if x > 0 {
            maybe_add(Coords { x: x - 1, y });
        }
        if x < self.x_max {
            maybe_add(Coords { x: x + 1, y });
        }
        if y > 0 {
            maybe_add(Coords { x, y: y - 1 });
        }
        if y < self.y_max {
            maybe_add(Coords { x, y: y + 1 });
        }
        directions
    }
}

fn parse_input(input: &str) -> Result<HeightMap, Box<dyn Error>> {
    let mut start_found: Option<Coords> = None;
    let mut end_found: Option<Coords> = None;

    let squares = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_ascii_str().map(|ascii_line| {
                ascii_line
                    .as_slice()
                    .iter()
                    .enumerate()
                    .map(|(x, letter)| match letter {
                        AsciiChar::S => {
                            start_found = Some(Coords { x, y });
                            AsciiChar::a
                        }
                        AsciiChar::E => {
                            end_found = Some(Coords { x, y });
                            AsciiChar::z
                        }
                        other => *other,
                    })
                    .collect()
            })
        })
        .collect::<Result<Vec<Vec<AsciiChar>>, AsAsciiStrError>>()?;
    let start = start_found.ok_or("Start not found")?;
    let end = end_found.ok_or("Start not found")?;
    let x_max = input.lines().next().ok_or("Empty input")?.len() - 1;
    let y_max = input.lines().count() - 1;
    Ok(HeightMap { squares, start, end, x_max, y_max })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> HeightMap {
        parse_input(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        )
        .unwrap()
    }

    #[test]
    fn test_parse_input() {
        let expected = HeightMap {
            squares: vec![
                vec![
                    AsciiChar::a,
                    AsciiChar::a,
                    AsciiChar::b,
                    AsciiChar::q,
                    AsciiChar::p,
                    AsciiChar::o,
                    AsciiChar::n,
                    AsciiChar::m,
                ],
                vec![
                    AsciiChar::a,
                    AsciiChar::b,
                    AsciiChar::c,
                    AsciiChar::r,
                    AsciiChar::y,
                    AsciiChar::x,
                    AsciiChar::x,
                    AsciiChar::l,
                ],
                vec![
                    AsciiChar::a,
                    AsciiChar::c,
                    AsciiChar::c,
                    AsciiChar::s,
                    AsciiChar::z,
                    AsciiChar::z,
                    AsciiChar::x,
                    AsciiChar::k,
                ],
                vec![
                    AsciiChar::a,
                    AsciiChar::c,
                    AsciiChar::c,
                    AsciiChar::t,
                    AsciiChar::u,
                    AsciiChar::v,
                    AsciiChar::w,
                    AsciiChar::j,
                ],
                vec![
                    AsciiChar::a,
                    AsciiChar::b,
                    AsciiChar::d,
                    AsciiChar::e,
                    AsciiChar::f,
                    AsciiChar::g,
                    AsciiChar::h,
                    AsciiChar::i,
                ],
            ],
            start: Coords { x: 0, y: 0 },
            end: Coords { x: 5, y: 2 },
            x_max: 7,
            y_max: 4,
        };
        assert_eq!(test_input(), expected);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_input()).unwrap(), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_input()).unwrap(), 29);
    }

    fn vec_to_set(vec: Vec<Coords>) -> HashSet<Coords> {
        HashSet::from_iter(vec)
    }
}
