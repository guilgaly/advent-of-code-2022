#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct Sensor {
    pub position: Point,
    pub closest_beacon: Point,
    pub beacon_distance: i64
}

impl Sensor {
    pub fn new(position: Point, closest_beacon: Point) -> Sensor {
        let distance = position.distance_to(&closest_beacon);
        Sensor { position, closest_beacon, beacon_distance: distance }
    }
}