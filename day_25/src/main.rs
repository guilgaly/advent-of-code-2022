use common::itertools::Itertools;
use common::time_execution;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 result: {}", part_1(INPUT));
    Ok(())
}

fn part_1(input: &str) -> String {
    let sum = snafu_sum(&input.lines().collect_vec());
    to_snafu(sum)
}

fn snafu_sum(snafu_numbers: &[&str]) -> u64 {
    snafu_numbers.iter().map(|n| from_snafu(n)).sum()
}

fn from_snafu(snafu: &str) -> u64 {
    snafu.chars().rev().enumerate().fold(0, |acc, (idx, n)| {
        let v = match n {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown SNAFU digit"),
        };
        acc + v * i64::pow(5, idx as u32)
    }) as u64
}

fn to_snafu(n: u64) -> String {
    fn to_snafu_digit(d: u64) -> String {
        match d {
            0 => "=",
            1 => "-",
            2 => "0",
            3 => "1",
            4 => "2",
            _ => panic!("Invalid SNAFU digit"),
        }.to_owned()
    }

    if n == 0 {
        String::new()
    } else {
        let res = (n + 2) / 5;
        let remainder = (n + 2) % 5;
        to_snafu(res) + &to_snafu_digit(remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_snafu() {
        assert_eq!(from_snafu("1=-0-2"), 1747);
        assert_eq!(from_snafu("12111"), 906);
        assert_eq!(from_snafu("2=0="), 198);
        assert_eq!(from_snafu("21"), 11);
        assert_eq!(from_snafu("2=01"), 201);
        assert_eq!(from_snafu("111"), 31);
        assert_eq!(from_snafu("20012"), 1257);
        assert_eq!(from_snafu("112"), 32);
        assert_eq!(from_snafu("1=-1="), 353);
        assert_eq!(from_snafu("1-12"), 107);
        assert_eq!(from_snafu("12"), 7);
        assert_eq!(from_snafu("1="), 3);
        assert_eq!(from_snafu("122"), 37);
    }

    fn test_snafu_sum() {
        assert_eq!(
            snafu_sum(&vec![
                "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12",
                "12", "1=", "122",
            ]),
            4890
        )
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(1747), "1=-0-2");
        assert_eq!(to_snafu(906), "12111");
        assert_eq!(to_snafu(198), "2=0=");
        assert_eq!(to_snafu(11), "21");
        assert_eq!(to_snafu(201), "2=01");
        assert_eq!(to_snafu(31), "111");
        assert_eq!(to_snafu(1257), "20012");
        assert_eq!(to_snafu(32), "112");
        assert_eq!(to_snafu(353), "1=-1=");
        assert_eq!(to_snafu(107), "1-12");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(37), "122");
    }
}
