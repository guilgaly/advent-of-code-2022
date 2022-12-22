use common::time_execution;
use std::error::Error;
use std::num::ParseIntError;
use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input(INPUT)?;
    println!(
        "Part 1 result: {}",
        time_execution("Part 1", || part_1(&input))
    );
    println!(
        "Part 2 result: {}",
        time_execution("Part 2", || part_2(&input))
    );

    Ok(())
}

fn part_1(input: &[i64]) -> i64 {
    let mixed_values = mix(input, 1, 1);
    get_result(&mixed_values)
}

fn part_2(input: &[i64]) -> i64 {
    let mixed_values = mix(input, 811589153, 10);
    get_result(&mixed_values)
}

fn mix(input: &[i64], key: i64, cycles: usize) -> Vec<i64> {
    let input = input.iter().map(|v| v * key).collect_vec();

    let mut positions = (0..input.len()).collect_vec();
    for _ in 0..cycles {
        for (orig_idx, &value) in input.iter().enumerate() {
            let pos = positions.iter().position(|&i| i == orig_idx).unwrap();
            positions.remove(pos);
            let new_pos = (pos as i64 + value).rem_euclid(positions.len() as i64) as usize;
            positions.insert(new_pos, orig_idx);
        }
    }

    positions_to_values(&input, &positions)
}

fn positions_to_values(values: &[i64], positions: &[usize]) -> Vec<i64> {
    positions.iter().map(|&orig_idx| values[orig_idx]).collect_vec()
}

fn get_result(mixed_values: &[i64]) -> i64 {
    let zero_idx = mixed_values.iter().position(|&v| v == 0).unwrap();
    let get_value = |n: usize| {
        mixed_values[(zero_idx + n) % mixed_values.len()]
    };
    get_value(1000) + get_value(2000) + get_value(3000)
}

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.lines().map(|l| l.parse::<i64>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<i64> {
        vec![1, 2, -3, 3, -2, 0, 4]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_input()), 3)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_input()), 1623178306)
    }
}
