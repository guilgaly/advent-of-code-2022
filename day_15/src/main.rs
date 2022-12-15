use std::cmp::{max, min};
use std::collections::HashSet;
use crate::parser_generator::{parse_sensors};
use crate::sensors::{Point, Sensor};
use std::error::Error;

mod parser_generator;
mod sensors;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let sensors = parse_sensors(INPUT)?;

    // println!("Part 1 result: {}", part_1(&sensors, 2000000));
    println!("Part 2 result: {}", part_2(&sensors, 4000000));

    Ok(())
}

fn part_1(sensors: &[Sensor], y: i64) -> usize {
    fn positions(sensor: &Sensor, y: i64) -> HashSet<Point> {
        let base_x = sensor.position.x;
        let base_dist = sensor.position.distance_to(&Point{ x: base_x, y });
        if base_dist > sensor.beacon_distance {
            HashSet::new()
        } else {
            let mut positions = HashSet::new();
            for i in 0..=(sensor.beacon_distance - base_dist) {
                positions.insert(Point { x: base_x + i, y });
                positions.insert(Point { x: base_x - i, y });
            }
            positions.remove(&sensor.closest_beacon);
            positions
        }
    }

    let mut acc = HashSet::new();
    for sensor in sensors {
        acc.extend(positions(sensor, y));
    }
    acc.len()
}

fn part_2(sensors: &[Sensor], max_coord: i64) -> i64 {

    fn maybe_add(pts: &mut Vec<Point>, x: i64, y: i64, max_coord: i64) {
        if x >= 0 && x <= max_coord && y >= 0 && y <= max_coord {
            pts.push(Point { x, y });
        }
    }

    fn perimeter(sensor: &Sensor, max_coord: i64) -> Vec<Point> {
        let mut pts = Vec::new();
        let dist = sensor.beacon_distance + 1;
        for x_dist in -dist..dist {
            let x = sensor.position.x + x_dist;
            let y_dist = dist - x_dist.abs();
            if y_dist == 0 {
                maybe_add(&mut pts, x, sensor.position.y, max_coord);
            } else {
                maybe_add(&mut pts, x, sensor.position.y + y_dist, max_coord);
                maybe_add(&mut pts, x, sensor.position.y - y_dist, max_coord);
            }
        }
        pts
    }

    for pt in sensors.iter().flat_map(|s| perimeter(s, max_coord)) {
        if sensors.iter().all(|s| s.position.distance_to(&pt) > s.beacon_distance) {
            return pt.x * 4000000 + pt.y;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sensors() -> Vec<Sensor> {
        parse_sensors("Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3").unwrap()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_sensors(), 10), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_sensors(), 20), 56000011);
    }
}
