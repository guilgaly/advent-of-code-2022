use crate::models::{Blueprint, Resources};
use peg::error::ParseError;
use peg::str::LineCol;

pub fn parse_blueprints(input: &str) -> Result<Vec<Blueprint>, ParseError<LineCol>> {
    parser::blueprints(input)
}

peg::parser! {
    grammar parser() for str {
        rule int64() -> u64 = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule int16() -> u16 = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule blueprint() -> Blueprint = "Blueprint " id:int64() ": "
            "Each ore robot costs " ore_r_ore:int16() " ore. "
            "Each clay robot costs " clay_r_ore:int16() " ore. "
            "Each obsidian robot costs " obs_r_ore:int16() " ore and " obs_r_clay:int16() " clay. "
            "Each geode robot costs " geod_r_ore:int16() " ore and " geod_r_obs:int16() " obsidian." {
                Blueprint {
                    id,
                    ore_robot: Resources::of_ore(ore_r_ore),
                    clay_robot: Resources::of_ore(clay_r_ore),
                    obsidian_robot: Resources { ore: obs_r_ore, clay: obs_r_clay, obsidian: 0 },
                    geode_robot: Resources { ore: geod_r_ore, clay: 0, obsidian: geod_r_obs }
                }
            }
        pub rule blueprints() -> Vec<Blueprint> = n:(blueprint() ** "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paths() {
        assert_eq!(
            parser::blueprints("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 18 obsidian.
Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 15 obsidian.").unwrap(),
            vec![
                Blueprint {
                    id: 1,
                    ore_robot: Resources { ore: 4, clay: 0, obsidian: 0 },
                    clay_robot: Resources { ore: 4, clay: 0, obsidian: 0 },
                    obsidian_robot: Resources { ore: 4, clay: 8, obsidian: 0},
                    geode_robot: Resources { ore: 2, clay: 0, obsidian: 18}
                },
                Blueprint {
                    id: 2,
                    ore_robot: Resources { ore: 4, clay: 0, obsidian: 0 },
                    clay_robot: Resources { ore: 4, clay: 0, obsidian: 0 },
                    obsidian_robot: Resources { ore: 3, clay: 19, obsidian: 0},
                    geode_robot: Resources { ore: 4, clay:0, obsidian: 15}
                },
            ]
        )
    }
}
