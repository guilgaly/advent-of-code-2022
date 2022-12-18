use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use common::time_execution;

static INPUT: &str = include_str!("input");
static MAX_X: usize = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let pushes = parse_input(INPUT)?;

    println!("Part 1 result: {}", time_execution("Part 1", || play(&pushes, 2022)));
    println!("Part 2 result: {}", time_execution("Part 2", || play(&pushes, 1000000000000)));

    Ok(())
}

fn play(pushes: &[Push], rocks_count: usize) -> usize {
    let rocks = [
        Rock::Horizontal,
        Rock::Cross,
        Rock::Angle,
        Rock::Vertical,
        Rock::Square,
    ];

    // i: rock index, j: push index
    let (mut i, mut j): (usize, usize) = (0, 0);
    let mut cave = CaveState::new();
    let mut cache: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for step in 0..rocks_count {
        let cache_key = (i, j);
        if let Some((cached_step, cached_height)) = cache.get(&cache_key) {
            let remaining_steps = rocks_count - step;
            let steps_since_cached = step - cached_step;
            let (d, m) = (remaining_steps / steps_since_cached, remaining_steps % steps_since_cached);
            if m == 0 {
                return cave.height + (cave.height - cached_height) * d;
            }
        } else {
            cache.insert(cache_key, (step, cave.height));
        }

        let mut rock_ref_point = cave.new_rock_ref_point();
        let rock = rocks[i];
        i = (i + 1) % rocks.len();

        loop {
            let push = pushes[j];
            j = (j + 1) % pushes.len();
            rock_ref_point = push_rock(&cave, rock, rock_ref_point, push);
            if let Some(new_point) = fall_down(&cave, rock, rock_ref_point) {
                rock_ref_point = new_point;
            } else {
                break;
            }
        }
        cave.add_stopped_rock(rock, rock_ref_point);
    }
    cave.height
}

fn push_rock(cave: &CaveState, rock: Rock, ref_point: Point, push: Push) -> Point {
    push.apply(ref_point, rock)
        .filter(|new_ref_point| {
            let rock_points = rock.points(*new_ref_point);
            !cave.contains_any(&rock_points)
        })
        .unwrap_or(ref_point)
}

fn fall_down(cave: &CaveState, rock: Rock, Point { x, y }: Point) -> Option<Point> {
    if y == 0 {
        None
    } else {
        let new_ref_point = Point { x, y: y - 1 };
        let rock_points = rock.points(new_ref_point);
        if cave.contains_any(&rock_points) {
            None
        } else {
            Some(new_ref_point)
        }
    }
}

// x: left to right, from 0 to 6
// y: bottom to top, from 0
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct CaveState {
    stopped_rocks: HashSet<Point>,
    height: usize
}

impl CaveState {
    fn new() -> CaveState {
        CaveState { stopped_rocks: HashSet::new(), height: 0 }
    }
    fn new_rock_ref_point(&self) -> Point {
        Point { x: 2, y: self.height + 3 }
    }
    fn contains_any(&self, points: &[Point]) -> bool {
        points.iter().any(|p| self.stopped_rocks.contains(p))
    }
    fn add_stopped_rock(&mut self, rock: Rock, ref_point: Point) {
        for pt in rock.points(ref_point) {
            self.stopped_rocks.insert(pt);
            self.height = max(self.height, ref_point.y + rock.height())
        }
    }
}

#[derive(Clone, Copy)]
enum Rock {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Square,
}

// Reference point of a rock is the bottom left point
impl Rock {
    fn width(&self) -> usize {
        match self {
            Rock::Horizontal => 4,
            Rock::Cross => 3,
            Rock::Angle => 3,
            Rock::Vertical => 1,
            Rock::Square => 2,
        }
    }
    fn height(&self) -> usize {
        match self {
            Rock::Horizontal => 1,
            Rock::Cross => 3,
            Rock::Angle => 3,
            Rock::Vertical => 4,
            Rock::Square => 2,
        }
    }
    fn points(&self, Point { x, y }: Point) -> Vec<Point> {
        match self {
            Rock::Horizontal => vec![
                Point { x, y },
                Point { x: x + 1, y },
                Point { x: x + 2, y },
                Point { x: x + 3, y },
            ],
            Rock::Cross => vec![
                Point { x: x + 1, y },
                Point { x, y: y + 1 },
                Point { x: x + 1, y: y + 1 },
                Point { x: x + 2, y: y + 1 },
                Point { x: x + 1, y: y + 2 },
            ],
            Rock::Angle => vec![
                Point { x, y },
                Point { x: x + 1, y },
                Point { x: x + 2, y },
                Point { x: x + 2, y: y + 1 },
                Point { x: x + 2, y: y + 2 },
            ],
            Rock::Vertical => vec![
                Point { x, y },
                Point { x, y: y + 1 },
                Point { x, y: y + 2 },
                Point { x, y: y + 3 },
            ],
            Rock::Square => vec![
                Point { x, y },
                Point { x: x + 1, y },
                Point { x, y: y + 1 },
                Point { x: x + 1, y: y + 1 },
            ],
        }
    }
}

#[derive(Clone, Copy)]
enum Push {
    Left,
    Right,
}

impl Push {
    fn apply(&self, Point { x, y }: Point, rock: Rock) -> Option<Point> {
        match self {
            Push::Left if x == 0 => None,
            Push::Left => Some(Point { x: x - 1, y }),
            Push::Right if (x + rock.width() - 1) == MAX_X => None,
            Push::Right => Some(Point { x: x + 1, y }),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Push>, String> {
    input
        .chars()
        .map(|c| match c {
            '<' => Ok(Push::Left),
            '>' => Ok(Push::Right),
            other => Err(format!("Invalid char {}", other)),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pushes() -> Vec<Push> {
        parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>").unwrap()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(play(&test_pushes(), 2022), 3068);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(play(&test_pushes(), 1000000000000), 1514285714288);
    }
}
