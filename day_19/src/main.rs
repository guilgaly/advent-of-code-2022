use crate::models::{Blueprint, Resources};
use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use common::itertools::Itertools;
use common::time_execution;

use crate::parsers::parse_blueprints;

mod models;
mod parsers;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let blueprints = parse_blueprints(INPUT)?;
    println!(
        "Part 1 result: {}",
        time_execution("Part 1", || part_1(&blueprints))
    );
    println!(
        "Part 2 result: {}",
        time_execution("Part 2", || part_2(&blueprints))
    );

    Ok(())
}

fn part_1(blueprints: &[Blueprint]) -> u64 {
    blueprints.iter()
        .map(|bp| {
            let quality_level = (evaluate_blueprint(bp, 24) as u64) * bp.id;
            println!("BP {} quality level: {}", bp.id, quality_level);
            quality_level
        })
        .sum()
}

fn part_2(blueprints: &[Blueprint]) -> u64 {
    blueprints.iter().take(3)
        .map(|bp| {
            let geodes = evaluate_blueprint(bp, 32);
            println!("BP {} geodes: {}", bp.id, geodes);
            geodes as u64
        })
        .product()
}

fn evaluate_blueprints(blueprints: &[Blueprint], time: u16) -> u16 {
    blueprints.iter()
        .map(|bp| {
            let quality_level = evaluate_blueprint(bp, time);
            println!("BP {} quality level: {}", bp.id, quality_level);
            quality_level
        })
        .sum()
}

fn evaluate_blueprint(blueprint: &Blueprint, time: u16) -> u16 {
    fn recurs_geodes_count(bp: &Blueprint, s: State, cache: &mut HashMap<State, u16>) -> u16 {
        if s.remaining_time == 0 {
            0
        } else {
            if !cache.contains_key(&s) {
                let mut max_geodes = recurs_geodes_count(
                    bp,
                    s.next_state(Resources::empty(), Resources::empty()),
                    cache
                );

                if s.resources.exceeds(bp.ore_robot) {
                    let next_state = s.next_state(bp.ore_robot, Resources::of_ore(1));
                    let geodes = recurs_geodes_count(bp, next_state, cache);
                    max_geodes = max(max_geodes, geodes);
                }

                if s.resources.exceeds(bp.clay_robot) {
                    let next_state = s.next_state(bp.clay_robot, Resources::of_clay(1));
                    let geodes = recurs_geodes_count(bp, next_state, cache);
                    max_geodes = max(max_geodes, geodes);
                }

                if s.resources.exceeds(bp.obsidian_robot) {
                    let next_state = s.next_state(bp.obsidian_robot, Resources::of_obsidian(1));
                    let geodes = recurs_geodes_count(bp, next_state, cache);
                    max_geodes = max(max_geodes, geodes);
                }

                if s.resources.exceeds(bp.geode_robot) {
                    let next_state = s.next_state(bp.geode_robot, Resources::empty());
                    let geodes = recurs_geodes_count(bp, next_state, cache) + s.remaining_time - 1;
                    max_geodes = max(max_geodes, geodes);
                }

                cache.insert(s, max_geodes);
            }

            *cache.get(&s).unwrap()
        }
    }

    time_execution(&format!("Evaluate BP {}", blueprint.id), || {
        let mut cache = HashMap::new();
        recurs_geodes_count(blueprint, State::new(time), &mut cache)
    })
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct State {
    remaining_time: u16,
    resources: Resources,
    robots: Resources,
}

impl State {
    fn next_state(&self, resources_spent: Resources, robots_built: Resources) -> State {
        State {
            remaining_time: self.remaining_time - 1,
            resources: self.resources.add(self.robots).remove(resources_spent),
            robots: self.robots.add(robots_built),
        }
    }
    fn new(time: u16) -> State {
        State {
            remaining_time: time,
            resources: Resources { ore: 0, clay: 0, obsidian: 0 },
            robots: Resources { ore: 1, clay: 0, obsidian: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_blueprints() -> Vec<Blueprint> {
        parse_blueprints("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.")
            .unwrap()
    }

    #[test]
    fn test_evaluate_blueprint_24() {
        assert_eq!(evaluate_blueprint(&test_blueprints()[0], 24), 9)
    }

    #[test]
    fn test_evaluate_blueprint_32() {
        assert_eq!(evaluate_blueprint(&test_blueprints()[0], 32), 9)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(evaluate_blueprints(&test_blueprints(), 24), 33);
    }
}
