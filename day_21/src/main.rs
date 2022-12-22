use crate::models::{Job, Monkeys, Operation};
use common::time_execution;
use std::error::Error;
use crate::parsers::parse_monkeys;

mod models;
mod parsers;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let monkeys = parse_monkeys(INPUT)?;

    println!(
        "Part 1 result: {}",
        time_execution("Part 1", || part_1(&monkeys))
    );
    println!(
        "Part 2 result: {}",
        time_execution("Part 2", || part_2(&monkeys))
    );

    Ok(())
}

fn part_1(monkeys: &Monkeys) -> i64 {
    recurs_eval("root", &monkeys).unwrap()
}

fn part_2(monkeys: &Monkeys) -> i64 {
    let mut m1 = monkeys.clone();
    m1.remove("humn");
    match m1.get("root") {
        Some(Job::Op(root1, _, root2)) => m1.insert("root".to_owned(), Job::Op(root1.clone(), Operation::Sub, root2.clone())),
        _ => panic!("Invalid root"),
    };
    recurs_solve("root", &m1, 0)
}

fn recurs_eval(name: &str, monkeys: &Monkeys) -> Option<i64> {
    match monkeys.get(name) {
        Some(Job::Value(x)) => Some(*x),
        Some(Job::Op(n1, op, n2)) => {
            match (recurs_eval(n1, monkeys), recurs_eval(n2, monkeys)) {
                (Some(v1), Some(v2)) => Some(op.apply(v1, v2)),
                (_, _) => None
            }
        }
        None => None
    }
}

fn recurs_solve(name: &str, monkeys: &Monkeys, expected_res: i64) -> i64 {
    if name == "humn" {
        expected_res
    } else {
        match monkeys.get(name).unwrap() {
            Job::Value(_) => panic!("Cannot solve value"),
            Job::Op(n1, op, n2) => {
                match (recurs_eval(n1, monkeys), recurs_eval(n2, monkeys)) {
                    (Some(v1), None) => {
                        match op {
                            Operation::Add => recurs_solve(n2, monkeys, expected_res - v1),
                            Operation::Sub => recurs_solve(n2, monkeys, v1 - expected_res),
                            Operation::Mul => recurs_solve(n2, monkeys, expected_res / v1),
                            Operation::Div => recurs_solve(n2, monkeys, v1 / expected_res),
                        }
                    },
                    (None, Some(v2)) => {
                        match op {
                            Operation::Add => recurs_solve(n1, monkeys, expected_res - v2),
                            Operation::Sub => recurs_solve(n1, monkeys, expected_res + v2),
                            Operation::Mul => recurs_solve(n1, monkeys, expected_res / v2),
                            Operation::Div => recurs_solve(n1, monkeys, expected_res * v2),
                        }
                    },
                    _ => panic!("Cannot solve")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
