extern crate core;

use common::lazy_static::lazy_static;
use common::regex::Regex;
use std::error::Error;
use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let assignments = parse_input(INPUT)?;

    println!("Part 1 result: {}", first_part(&assignments));
    println!("Part 2 result: {}", second_part(&assignments));

    Ok(())
}

fn first_part(assignments: &[AssignmentsPair]) -> usize {
    assignments.iter()
        .filter(|(assignment_1, assignment_2)| {
            assignment_1.contains(assignment_2) || assignment_2.contains(assignment_1)
        })
        .count()
}

fn second_part(assignments: &[AssignmentsPair]) -> usize {
    assignments.iter()
        .filter(|(assignment_1, assignment_2)| {
            assignment_1.overlaps(assignment_2)
        })
        .count()
}

type AssignmentsPair = (Assignment, Assignment);
#[derive(Debug, PartialEq)]
struct Assignment { from: u64, to: u64 }
impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.from <= other.from && self.to >= other.to
    }
    fn overlaps(&self, other: &Assignment) -> bool {
        (self.to >= other.from && self.from <= other.from) || (self.from <= other.to && self.to >= other.from)
    }
}

fn parse_input(
    input: &str,
) -> Result<Vec<AssignmentsPair>, String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    input
        .lines()
        .map(|line| {
            REGEX
                .captures(line)
                .and_then(|cap| {
                    let r1_start = cap.get(1)?.as_str().parse::<u64>().ok()?;
                    let r1_end = cap.get(2)?.as_str().parse::<u64>().ok()?;
                    let r2_start = cap.get(3)?.as_str().parse::<u64>().ok()?;
                    let r2_end = cap.get(4)?.as_str().parse::<u64>().ok()?;
                    Some((Assignment {from: r1_start, to: r1_end}, Assignment {from: r2_start, to:r2_end}))
                })
                .ok_or(format!("Failed to parse line {}", line))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<AssignmentsPair> {
        let test_input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        parse_input(test_input).unwrap()
    }

    #[test]
    fn test_parse() {
        let expected = vec![
            (Assignment{ from: 2, to: 4 }, Assignment { from: 6, to: 8}),
            (Assignment{ from: 2, to: 3 }, Assignment { from: 4, to: 5}),
            (Assignment{ from: 5, to: 7 }, Assignment { from: 7, to: 9}),
            (Assignment{ from: 2, to: 8 }, Assignment { from: 3, to: 7}),
            (Assignment{ from: 6, to: 6 }, Assignment { from: 4, to: 6}),
            (Assignment{ from: 2, to: 6 }, Assignment { from: 4, to: 8}),
        ];
        assert_eq!(test_data(), expected);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(&test_data()), 2);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(&test_data()), 4);
    }
}
