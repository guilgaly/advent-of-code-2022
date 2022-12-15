use crate::parser_generator::{eval_packet, Data};
use common::itertools::{EitherOrBoth, Itertools};
use std::cmp::Ordering;
use std::error::Error;

mod parser_generator;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let packet_pairs = parse_input(INPUT)?;
    println!("Part 1 result: {}", part_1(&packet_pairs));
    println!("Part 2 result: {}", part_2(&packet_pairs));

    Ok(())
}

fn part_1(packet_pairs: &[(Data, Data)]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter_map(
            |(idx, (left, right))| match is_in_right_order(left, right) {
                Some(true) => Some(idx + 1),
                _ => None,
            },
        )
        .sum()
}

fn part_2(packet_pairs: &[(Data, Data)]) -> usize {
    let div1 = Data::List(vec![Data::List(vec![Data::Int(2)])]);
    let div2 = Data::List(vec![Data::List(vec![Data::Int(6)])]);

    let mut packets: Vec<Data> = packet_pairs
        .iter()
        .flat_map(|(left, right)| [left, right].into_iter())
        .cloned()
        .collect();
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(|left, right| match is_in_right_order(left, right) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let idx1 = packets
        .iter()
        .position(|p| p == &div1)
        .map(|i| i + 1)
        .unwrap_or(0);
    let idx2 = packets
        .iter()
        .position(|p| p == &div2)
        .map(|i| i + 1)
        .unwrap_or(0);

    idx1 * idx2
}

fn is_in_right_order(left: &Data, right: &Data) -> Option<bool> {
    match (left, right) {
        (Data::Int(l), Data::Int(r)) => {
            if l < r {
                Some(true)
            } else if l > r {
                Some(false)
            } else {
                None
            }
        }
        (Data::Int(l), r @ Data::List(_)) => is_in_right_order(&Data::List(vec![Data::Int(*l)]), r),
        (l @ Data::List(_), Data::Int(r)) => is_in_right_order(l, &Data::List(vec![Data::Int(*r)])),
        (Data::List(left_list), Data::List(right_list)) => left_list
            .iter()
            .zip_longest(right_list.iter())
            .find_map(|x| match x {
                EitherOrBoth::Both(l, r) => is_in_right_order(l, r),
                EitherOrBoth::Left(_) => Some(false),
                EitherOrBoth::Right(_) => Some(true),
            }),
    }
}

fn parse_input(input: &str) -> Result<Vec<(Data, Data)>, Box<dyn Error>> {
    input
        .split("\n\n")
        .map(|pair_str| {
            let mut lines = pair_str.lines();
            let first = lines.next().ok_or("Missing first line".to_owned())?;
            let second = lines.next().ok_or("Missing second line".to_owned())?;
            let left = eval_packet(first)?;
            let right = eval_packet(second)?;
            Ok((left, right))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = parse_input(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]
",
        )
        .unwrap();
        let expected = vec![
            (
                Data::List(vec![
                    Data::Int(1),
                    Data::Int(1),
                    Data::Int(3),
                    Data::Int(1),
                    Data::Int(1),
                ]),
                Data::List(vec![
                    Data::Int(1),
                    Data::Int(1),
                    Data::Int(5),
                    Data::Int(1),
                    Data::Int(1),
                ]),
            ),
            (
                Data::List(vec![
                    Data::List(vec![Data::Int(1)]),
                    Data::List(vec![Data::Int(2), Data::Int(3), Data::Int(4)]),
                ]),
                Data::List(vec![Data::List(vec![Data::Int(1)]), Data::Int(4)]),
            ),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_in_right_order() -> Result<(), Box<dyn Error>> {
        assert_eq!(is_in_right_order(&Data::Int(1), &Data::Int(2)), Some(true));
        assert_eq!(is_in_right_order(&Data::Int(2), &Data::Int(1)), Some(false));
        assert_eq!(is_in_right_order(&Data::Int(1), &Data::Int(1)), None);

        assert_eq!(
            is_in_right_order(&eval_packet("[1,1,3,1,1]")?, &eval_packet("[1,1,5,1,1]")?),
            Some(true)
        );
        assert_eq!(
            is_in_right_order(&eval_packet("[[1],[2,3,4]]")?, &eval_packet("[[1],4]")?),
            Some(true)
        );
        assert_eq!(
            is_in_right_order(&eval_packet("[9]")?, &eval_packet("[[8,7,6]]")?),
            Some(false)
        );
        assert_eq!(
            is_in_right_order(&eval_packet("[[4,4],4,4]")?, &eval_packet("[[4,4],4,4,4]")?),
            Some(true)
        );
        assert_eq!(
            is_in_right_order(&eval_packet("[7,7,7,7]")?, &eval_packet("[7,7,7]")?),
            Some(false)
        );
        assert_eq!(
            is_in_right_order(&eval_packet("[]")?, &eval_packet("[3]")?),
            Some(true)
        );
        assert_eq!(
            is_in_right_order(&eval_packet("[[[]]]")?, &eval_packet("[[]]")?),
            Some(false)
        );
        assert_eq!(
            is_in_right_order(
                &eval_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]")?,
                &eval_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]")?
            ),
            Some(false)
        );

        Ok(())
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&sample()), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&sample()), 140);
    }

    fn sample() -> Vec<(Data, Data)> {
        parse_input(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
        .unwrap()
    }
}
