use ndarray::Array3;
use std::collections::HashMap;
use std::env;
use std::fs;

/// Shamelessly copied from https://www.reddit.com/r/adventofcode/comments/zn6k1l/comment/j0gmocd/?utm_source=share&utm_medium=web2x&context=3
pub fn part_2(input: &str) -> u16 {
    let mut valves = Vec::<(&str, u16, Vec<&str>)>::new();
    for line in input.trim().split('\n') {
        let (valve, flow, _, tunnels) = sscanf::sscanf!(
            line,
            "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/} {str}"
        )
        .unwrap();
        let tunnels = tunnels.split(", ").collect::<Vec<_>>();
        valves.push((valve, flow, tunnels));
    }

    // compute indices so that valves with positive flow have indices 0..m
    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];
    for v in valves.iter() {
        let i = lab2idx[v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(lab2idx[w]);
        }
    }
    let aa = lab2idx["AA"];

    let mm = 1 << m; // m = number of valves with positive flow
                     // dynamic programming [time left, current node, bitset of available valves]
    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = o;
            }
        }
    }

    // elephant and human open disjoint sets of valves
    let mut best = 0;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(opt[(25, aa, x)] + opt[(25, aa, y)]);
    }

    best
}
