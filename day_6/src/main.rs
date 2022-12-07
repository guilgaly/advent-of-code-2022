extern crate core;

use std::collections::HashSet;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 result: {}", find_marker(INPUT, 4)?);
    println!("Part 2 result: {}", find_marker(INPUT, 14)?);

    Ok(())
}

fn find_marker(buffer: &str, length: usize) -> Result<usize, String> {
    let chars: Vec<_> = buffer.char_indices().collect();
    chars
        .windows(length)
        .find_map(|window| {
            let set: HashSet<_> = window.iter().map(|(_, c)| c).collect();
            if set.len() == length {
                window.last().map(|(idx, _)| idx + 1)
            } else {
                None
            }
        })
        .ok_or(format!("Marker of length {} not found", length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<(), String> {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)?, 7);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)?, 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4)?, 6);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)?, 10);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)?, 11);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), String> {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)?, 19);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14)?, 23);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14)?, 23);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)?, 29);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)?, 26);
        Ok(())
    }
}
