use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let strategy_1: Vec<(Move, Move)> = parse_input_1(INPUT)?;
    println!("Part 1 result: {}", first_part(&strategy_1));

    let strategy_2: Vec<(Move, RoundResult)> = parse_input_2(INPUT)?;
    println!("Part 2 result: {}", second_part(&strategy_2));

    Ok(())
}

fn first_part(strategy: &[(Move, Move)]) -> u64 {
    strategy
        .iter()
        .map(|(op_move, my_move)| round_score(op_move, my_move))
        .sum()
}

fn second_part(strategy: &[(Move, RoundResult)]) -> u64 {
    strategy
        .iter()
        .map(|(op_move, expected_res)| {
            let my_move = match (op_move, expected_res) {
                (op_move, RoundResult::Draw) => *op_move,
                (op_move, RoundResult::Win) => op_move.loses_against(),
                (op_move, RoundResult::Lose) => op_move.wins_against(),
            };
            round_score(op_move, &my_move)
        })
        .sum()
}

fn round_score(op_move: &Move, my_move: &Move) -> u64 {
    my_move.result_against(op_move).value() + my_move.value()
}

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn wins_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
    fn loses_against(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn result_against(&self, op: &Move) -> RoundResult {
        if self == op {
            RoundResult::Draw
        } else if self.wins_against() == *op {
            RoundResult::Win
        } else {
            RoundResult::Lose
        }
    }
}

enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl RoundResult {
    fn value(&self) -> u64 {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

fn parse_input_1(input: &str) -> Result<Vec<(Move, Move)>, String> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let op_move_char = chars.next().ok_or(format!("Invalid line {}", input))?;
            let op_move = parse_move(&op_move_char)?;
            chars.next().ok_or(format!("Invalid line {}", input))?;
            let my_move_char = chars.next().ok_or(format!("Invalid line {}", input))?;
            let my_move = parse_move(&my_move_char)?;
            Ok((op_move, my_move))
        })
        .collect()
}

fn parse_input_2(input: &str) -> Result<Vec<(Move, RoundResult)>, String> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let op_move_char = chars.next().ok_or(format!("Invalid line {}", input))?;
            let op_move = parse_move(&op_move_char)?;
            chars.next().ok_or(format!("Invalid line {}", input))?;
            let round_res_char = chars.next().ok_or(format!("Invalid line {}", input))?;
            let round_res = parse_result(&round_res_char)?;
            Ok((op_move, round_res))
        })
        .collect()
}

fn parse_move(m: &char) -> Result<Move, String> {
    if *m == 'A' || *m == 'X' {
        Ok(Move::Rock)
    } else if *m == 'B' || *m == 'Y' {
        Ok(Move::Paper)
    } else if *m == 'C' || *m == 'Z' {
        Ok(Move::Scissors)
    } else {
        Err(format!("Invalid move {}", m))
    }
}

fn parse_result(r: &char) -> Result<RoundResult, String> {
    if *r == 'X' {
        Ok(RoundResult::Lose)
    } else if *r == 'Y' {
        Ok(RoundResult::Draw)
    } else if *r == 'Z' {
        Ok(RoundResult::Win)
    } else {
        Err(format!("Invalid result {}", r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
