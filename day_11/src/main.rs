use common::itertools::Itertools;
use std::collections::VecDeque;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 result: {}", part_1(input()));
    println!("Part 2 result: {}", part_2(input()));

    Ok(())
}

fn part_1(m: Vec<Monkey>) -> usize {
    let mut monkeys = m;
    for r in 0..20 {
        play_round(&mut monkeys, 3);
    }
    monkey_business(&monkeys)
}

fn part_2(m: Vec<Monkey>) -> usize {
    let mut monkeys = m;
    for r in 0..10000 {
        play_round(&mut monkeys, 1);
    }
    monkey_business(&monkeys)
}

fn monkey_business(monkeys: &[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn play_round(monkeys: &mut Vec<Monkey>, divide_by: usize) {
    // The test divisors are all different primes, so we can just multiply them to get the LCM
    let lcm: usize = monkeys.iter().map(|m| m.test_divisible_by).product();
    for m in 0..monkeys.len() {
        let items = monkeys[m].items.clone();
        monkeys[m].inspection_count += items.len();
        for item in items {
            let new_worry_level = (monkeys[m].operation)(item) / divide_by % lcm;
            let throw_to: usize = if new_worry_level % monkeys[m].test_divisible_by == 0 {
                monkeys[m].test_dest_true
            } else {
                monkeys[m].test_dest_false
            };
            monkeys[throw_to].items.push(new_worry_level);
        }
        monkeys[m].items = vec![];
    }
    // monkeys
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, m)| println!("Monkey {} inspected items {} times.", i, m.inspection_count));
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test_divisible_by: usize,
    test_dest_true: usize,
    test_dest_false: usize,
    inspection_count: usize,
}

fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![54, 61, 97, 63, 74],
            operation: Box::new(|old| old * 7),
            test_divisible_by: 17,
            test_dest_true: 5,
            test_dest_false: 3,
            inspection_count: 0,
        },
        Monkey {
            items: vec![61, 70, 97, 64, 99, 83, 52, 87],
            operation: Box::new(|old| old + 8),
            test_divisible_by: 2,
            test_dest_true: 7,
            test_dest_false: 6,
            inspection_count: 0,
        },
        Monkey {
            items: vec![60, 67, 80, 65],
            operation: Box::new(|old| old * 13),
            test_divisible_by: 5,
            test_dest_true: 1,
            test_dest_false: 6,
            inspection_count: 0,
        },
        Monkey {
            items: vec![61, 70, 76, 69, 82, 56],
            operation: Box::new(|old| old + 7),
            test_divisible_by: 3,
            test_dest_true: 5,
            test_dest_false: 2,
            inspection_count: 0,
        },
        Monkey {
            items: vec![79, 98],
            operation: Box::new(|old| old + 2),
            test_divisible_by: 7,
            test_dest_true: 0,
            test_dest_false: 3,
            inspection_count: 0,
        },
        Monkey {
            items: vec![72, 79, 55],
            operation: Box::new(|old| old + 1),
            test_divisible_by: 13,
            test_dest_true: 2,
            test_dest_false: 1,
            inspection_count: 0,
        },
        Monkey {
            items: vec![63],
            operation: Box::new(|old| old + 4),
            test_divisible_by: 19,
            test_dest_true: 7,
            test_dest_false: 4,
            inspection_count: 0,
        },
        Monkey {
            items: vec![72, 51, 93, 63, 80, 86, 81],
            operation: Box::new(|old| old * old),
            test_divisible_by: 11,
            test_dest_true: 0,
            test_dest_false: 4,
            inspection_count: 0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                items: vec![79, 98],
                operation: Box::new(|old| old * 19),
                test_divisible_by: 23,
                test_dest_true: 2,
                test_dest_false: 3,
                inspection_count: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Box::new(|old| old + 6),
                test_divisible_by: 19,
                test_dest_true: 2,
                test_dest_false: 0,
                inspection_count: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Box::new(|old| old * old),
                test_divisible_by: 13,
                test_dest_true: 1,
                test_dest_false: 3,
                inspection_count: 0,
            },
            Monkey {
                items: vec![74],
                operation: Box::new(|old| old + 3),
                test_divisible_by: 17,
                test_dest_true: 0,
                test_dest_false: 1,
                inspection_count: 0,
            },
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(test_monkeys()), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(test_monkeys()), 2713310158);
    }
}
