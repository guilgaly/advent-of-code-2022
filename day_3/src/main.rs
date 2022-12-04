extern crate core;

use std::collections::HashSet;
use std::error::Error;
use std::iter::Chain;
use std::slice::Iter;
use ascii::{AsAsciiStr, AsciiChar};
use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let rucksacks = parse_input(INPUT)?;

    println!("Part 1 result: {}", first_part(&rucksacks)?);
    println!("Part 2 result: {}", second_part(&rucksacks)?);

    Ok(())
}

fn first_part(rucksacks: &[Rucksack]) -> Result<u64, String> {
    rucksacks.iter()
        .map(|rucksack| {
            let common_items: HashSet<&Item> = rucksack.0.iter()
                .filter(|i| rucksack.1.contains(i))
                .collect();
            let common_item = common_items.iter().next().ok_or("No intersection found")?;
            Ok(common_item.priority())
        })
        .sum()
}

fn second_part(rucksacks: &[Rucksack]) -> Result<u64, String> {
    rucksacks.into_iter()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let common_items =
                group
                    .map(|r| r.all_items())
                    .reduce(|first, second| first.intersection(&second).copied().collect())
                    .ok_or("Empty group")?;
            let common_item = common_items.iter().next().ok_or("No intersection found")?;
            Ok(common_item.priority())
        })
        .sum()
}

struct Rucksack(Vec<Item>, Vec<Item>);

impl Rucksack {
    fn all_items(&self) -> HashSet<Item> {
        self.0.iter().chain(self.1.iter()).copied().collect()
    }
}

#[derive(PartialEq, Copy, Clone, Eq, Hash, Debug)]
struct Item(AsciiChar);

impl Item {
    fn priority(&self) -> u64 {
        let v =
            if self.0.is_ascii_uppercase() {
                self.0.as_byte() - AsciiChar::A.as_byte() + 27
            } else if self.0.is_ascii_lowercase() {
                self.0.as_byte() - AsciiChar::a.as_byte() + 1
            } else {
                0
            };
        v as u64
    }
}

fn parse_input(input: &str) -> Result<Vec<Rucksack>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let ascii_line = line.as_ascii_str()?;
            let rucksack_size = ascii_line.len() / 2;
            let slice = ascii_line.as_slice();
            let compartment_1:Vec<Item> = slice.iter().take(rucksack_size).map(|item| Item(*item)).collect();
            let compartment_2:Vec<Item> = slice.iter().skip(rucksack_size).map(|item| Item(*item)).collect();
            Ok(Rucksack(compartment_1, compartment_2))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Rucksack> {
        let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        parse_input(test_input).unwrap()
    }

    #[test]
    fn test_priority() {
        assert_eq!(Item(AsciiChar::a).priority(), 1);
        assert_eq!(Item(AsciiChar::z).priority(), 26);
        assert_eq!(Item(AsciiChar::A).priority(), 27);
        assert_eq!(Item(AsciiChar::Z).priority(), 52);
    }

    #[test]
    fn test_1() {
        let res = first_part(&test_data()).unwrap();
        assert_eq!(res, 157)
    }
}
