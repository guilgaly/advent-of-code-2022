use crate::part_2::part_2;
use crate::valves::{parse_valves, Valve};
use common::itertools::Itertools;
use common::time_execution;
use std::collections::{HashMap, HashSet};
use std::error::Error;

mod part_2;
mod valves;

static INPUT: &str = include_str!("input");
static START: &str = "AA";

type Flows = HashMap<String, u64>;
type Graph = HashMap<String, Vec<String>>;

fn main() -> Result<(), Box<dyn Error>> {
    let valves = parse_valves(INPUT)?;
    let flows: Flows = valves
        .iter()
        .map(|v| (v.name.clone(), v.flow_rate))
        .collect();
    let graph: Graph = valves
        .iter()
        .map(|v| (v.name.clone(), v.leads_to.clone()))
        .collect();

    println!(
        "Part 1 result: {}",
        time_execution("Part 1", || part_1(&flows, &graph))
    );
    println!(
        "Part 2 result: {}",
        time_execution("Part 2", || part_2(INPUT))
    );

    Ok(())
}

fn part_1(flows: &Flows, graph: &Graph) -> u64 {
    // (pressure, path, opened)
    let mut frontier: Vec<(u64, Vec<String>, HashSet<String>)> =
        vec![(0, vec![START.to_owned()], HashSet::new())];

    for i in 1..=30 {
        let mut new_frontier = vec![];
        for (pressure, path, opened) in frontier {
            let loc = path.last().unwrap();

            let new_pressure =
                pressure + opened.iter().fold(0, |sum, o| sum + flows.get(o).unwrap());

            // at this step, we could travel to a neighbor
            for neighbor in graph.get(loc).unwrap() {
                let mut new_path = path.clone();
                new_path.push(neighbor.clone());
                new_frontier.push((new_pressure, new_path, opened.clone()));
            }

            // alternately, we could release the pressure
            if *flows.get(loc).unwrap() > 0 && !opened.contains(loc) {
                let mut new_opened = opened.clone();
                new_opened.insert(loc.clone());
                new_frontier.push((new_pressure, path.clone(), new_opened));
            }
        }
        sort_and_truncate(&mut new_frontier, 3000);
        frontier = new_frontier;
    }

    frontier.first().unwrap().0
}

fn sort_and_truncate<P, O>(frontier: &mut Vec<(u64, P, O)>, max: usize) {
    frontier
        .sort_by(|(pressure_left, _, _), (pressure_right, _, _)| pressure_right.cmp(pressure_left));
    frontier.truncate(max);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnels lead to valves GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnels lead to valves II";

    fn test_valves() -> (Flows, Graph) {
        let valves = parse_valves(TEST_INPUT).unwrap();
        let flows: Flows = valves
            .iter()
            .map(|v| (v.name.clone(), v.flow_rate))
            .collect();
        let graph: Graph = valves
            .iter()
            .map(|v| (v.name.clone(), v.leads_to.clone()))
            .collect();
        (flows, graph)
    }

    #[test]
    fn test_part_1() {
        let (flows, graph) = test_valves();
        assert_eq!(part_1(&flows, &graph), 1651);
    }

    #[test]
    fn test_part_2() {
        let (flows, graph) = test_valves();
        assert_eq!(part_2(TEST_INPUT), 1707);
    }
}
