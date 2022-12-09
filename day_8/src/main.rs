use common::itertools::Itertools;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let trees = parse_input(INPUT)?;

    println!("Part 1 result: {}", part_1(&trees));
    println!("Part 2 result: {}", part_2(&trees));

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Forest {
    trees: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Forest {
    fn get_tree(&self, x: usize, y: usize) -> u32 {
        *self.trees.get(y).and_then(|l| l.get(x)).unwrap()
    }
    fn get_left_of(&self, x: usize, y: usize) -> Vec<u32> {
        self.trees.get(y).unwrap().iter().take(x).copied().collect()
    }
    fn get_right_of(&self, x: usize, y: usize) -> Vec<u32> {
        self.trees
            .get(y)
            .unwrap()
            .iter()
            .skip(x + 1)
            .copied()
            .collect()
    }
    fn get_top_of(&self, x: usize, y: usize) -> Vec<u32> {
        self.trees
            .iter()
            .take(y)
            .map(|l| l.get(x).unwrap())
            .copied()
            .collect()
    }
    fn get_bottom_of(&self, x: usize, y: usize) -> Vec<u32> {
        self.trees
            .iter()
            .skip(y + 1)
            .map(|l| l.get(x).unwrap())
            .copied()
            .collect()
    }
}

fn part_1(forest: &Forest) -> usize {
    let mut count: usize = 0;
    for y in 0..forest.height {
        for x in 0..forest.width {
            if is_visible(forest, x, y) {
                count += 1;
            }
        }
    }
    count
}

fn is_visible(forest: &Forest, x: usize, y: usize) -> bool {
    let tree = forest.get_tree(x, y);

    let hidden_left = forest
        .get_left_of(x, y)
        .into_iter()
        .find(|t| *t >= tree)
        .is_some();
    let hidden_right = forest
        .get_right_of(x, y)
        .into_iter()
        .find(|t| *t >= tree)
        .is_some();
    let hidden_top = forest
        .get_top_of(x, y)
        .into_iter()
        .find(|t| *t >= tree)
        .is_some();
    let hidden_bottom = forest
        .get_bottom_of(x, y)
        .into_iter()
        .find(|t| *t >= tree)
        .is_some();

    !(hidden_left && hidden_right && hidden_top && hidden_bottom)
}

fn part_2(forest: &Forest) -> usize {
    let mut max = 0;
    for y in 0..forest.height {
        for x in 0..forest.width {
            let score = scenic_score(forest, x, y);
            if score > max {
                max = score;
            }
        }
    }
    max
}

fn scenic_score(forest: &Forest, x: usize, y: usize) -> usize {
    fn count_visible(trees: &[u32], tree: u32) -> usize {
        match trees.iter().find_position(|t| **t >= tree) {
            None => trees.len(),
            Some((idx, _)) => idx + 1,
        }
    }

    let tree = forest.get_tree(x, y);
    let mut left = forest.get_left_of(x, y);
    left.reverse();
    let right = forest.get_right_of(x, y);
    let mut top = forest.get_top_of(x, y);
    top.reverse();
    let bottom = forest.get_bottom_of(x, y);

    count_visible(&left, tree)
        * count_visible(&right, tree)
        * count_visible(&top, tree)
        * count_visible(&bottom, tree)
}

fn parse_input(input: &str) -> Result<Forest, String> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).ok_or(format!("{} is not a digit", c)))
                .collect()
        })
        .collect::<Result<Vec<Vec<u32>>, String>>()?;
    let height = trees.len();
    let width = trees
        .get(0)
        .map(|l| l.len())
        .ok_or("Must contain at least one line")?;
    Ok(Forest { trees, height, width })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_trees() -> Forest {
        let test_input = "30373
25512
65332
33549
35390";
        parse_input(test_input).unwrap()
    }

    #[test]
    fn test_parse_input() {
        let expected = Forest {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
            height: 5,
            width: 5,
        };
        assert_eq!(test_trees(), expected)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_trees()), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_trees()), 8);
    }
}
