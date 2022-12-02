use common::itertools::Itertools;
use std::error::Error;
use std::num::ParseIntError;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let elves = parse_input(INPUT)?;

    let max = find_max_calories(&elves);
    println!("Part 1 result: {}", max);

    let top_three_sum = find_top_three_sum(&elves);
    println!("Part 2 result: {}", top_three_sum);
    Ok(())
}

fn find_max_calories(elves: &[Vec<u64>]) -> u64 {
    elves.iter().map(|elf| elf.iter().sum()).max().unwrap_or(0)
}

fn find_top_three_sum(elves: &[Vec<u64>]) -> u64 {
    elves
        .iter()
        .map(|elf| elf.iter().sum())
        .sorted_by(|a: &u64, b: &u64| Ord::cmp(&b, &a))
        .take(3)
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Vec<u64>>, ParseIntError> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u64>()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
}
