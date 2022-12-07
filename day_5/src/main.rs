extern crate core;

use common::lazy_static::lazy_static;
use common::regex::Regex;
use std::error::Error;
use std::fmt::format;

static INPUT_1: &str = include_str!("input_1");
static INPUT_2: &str = include_str!("input_2");

fn main() -> Result<(), Box<dyn Error>> {
    let stacks = stacks();
    let moves = parse_moves(INPUT_2)?;

    println!("Part 1 result: {}", first_part(&stacks, &moves)?);
    println!("Part 2 result: {}", second_part(&stacks, &moves)?);

    Ok(())
}

fn first_part(init_stacks: &Stacks, move_orders: &[MoveOrder]) -> Result<String, String> {
    fn apply_moves(stacks: &mut Stacks, orders: &[MoveOrder]) -> Result<(), String> {
        for order in orders {
            for _ in 0..order.qty {
                let c = stacks.get_mut(order.from)?.pop().ok_or("from_stack empty")?;
                stacks.get_mut(order.to)?.push(c);
            }
        }
        Ok(())
    }

    let mut stacks = init_stacks.clone();
    apply_moves(&mut stacks, move_orders)?;
    Ok(top_crates(&stacks))
}

fn second_part(init_stacks: &Stacks, move_orders: &[MoveOrder]) -> Result<String, String> {
    fn apply_moves(stacks: &mut Stacks, orders: &[MoveOrder]) -> Result<(), String> {
        for order in orders {
            let mut crates = vec![];
            for _ in 0..order.qty {
                crates.push(stacks.get_mut(order.from)?.pop().ok_or("from_stack empty")?);
            }
            for c in crates.iter().rev() {
                stacks.get_mut(order.to)?.push(*c);
            }
        }
        Ok(())
    }

    let mut stacks = init_stacks.clone();
    apply_moves(&mut stacks, move_orders)?;
    Ok(top_crates(&stacks))
}

fn top_crates(stacks: &Stacks) -> String {
    stacks.all.iter()
        .fold(
            String::new(),
            |acc, stack| acc + &stack.last().map(|c| c.0.to_string()).unwrap_or(String::new())
        )
}

#[derive(Clone, Copy)]
struct Crate(char);

#[derive(Clone)]
struct Stacks {
    all: Vec<Vec<Crate>>
}

impl Stacks {
    fn get_mut(&mut self, idx: usize) -> Result<&mut Vec<Crate>, String> {
        self.all.get_mut(idx - 1).ok_or(format!("Stack {} not found", idx))
    }
}

#[derive(Debug, PartialEq)]
struct MoveOrder {
    qty: u64,
    from: usize,
    to: usize
}

fn stacks() -> Stacks {
    let chars = vec![
        vec!['G', 'D', 'V', 'Z', 'J', 'S', 'B'],
        vec!['Z', 'S', 'M', 'G', 'V', 'P'],
        vec!['C', 'L', 'B', 'S', 'W', 'T', 'Q', 'F'],
        vec!['H', 'J', 'G', 'W', 'M', 'R', 'V', 'Q'],
        vec!['C', 'L', 'S', 'N', 'F', 'M', 'D'],
        vec!['R', 'G', 'C', 'D'],
        vec!['H', 'G', 'T', 'R', 'J', 'D', 'S', 'Q'],
        vec!['P', 'F', 'V'],
        vec!['D', 'R', 'S', 'T', 'J'],
    ];
    Stacks {
        all: chars.iter().map(|s| s.iter().map(|c| Crate(*c)).collect()).collect()
    }

}

fn parse_moves(
    input: &str,
) -> Result<Vec<MoveOrder>, String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    input
        .lines()
        .map(|line| {
            REGEX
                .captures(line)
                .and_then(|cap| {
                    let qty = cap.get(1)?.as_str().parse::<u64>().ok()?;
                    let from = cap.get(2)?.as_str().parse::<usize>().ok()?;
                    let to = cap.get(3)?.as_str().parse::<usize>().ok()?;
                    Some(MoveOrder { qty, from, to })
                })
                .ok_or(format!("Failed to parse line {}", line))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_moves() -> Vec<MoveOrder> {
        let test_input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        parse_moves(test_input).unwrap()
    }

    fn test_stacks() -> Stacks {
        Stacks {
            all: vec![
                vec![Crate('Z'), Crate('N')],
                vec![Crate('M'), Crate('C'), Crate('D')],
                vec![Crate('P')],
            ]
        }
    }

    #[test]
    fn test_parse() {
        let expected = vec![
            MoveOrder { qty: 1, from: 2, to: 1 },
            MoveOrder { qty: 3, from: 1, to: 3 },
            MoveOrder { qty: 2, from: 2, to: 1 },
            MoveOrder { qty: 1, from: 1, to: 2 },
        ];
        assert_eq!(test_moves(), expected);
    }

    #[test]
    fn test_first_part() -> Result<(), String> {
        let actual = first_part(&test_stacks(), &test_moves())?;
        assert_eq!(actual, "CMZ");
        Ok(())
    }

    #[test]
    fn test_second_part() -> Result<(), String> {
        let actual = second_part(&test_stacks(), &test_moves())?;
        assert_eq!(actual, "MCD");
        Ok(())
    }
}
