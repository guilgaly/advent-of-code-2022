use peg::error::ParseError;
use peg::str::LineCol;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Valve {
    pub name: String,
    pub flow_rate: u64,
    pub leads_to: Vec<String>,
}

pub fn parse_valves(input: &str) -> Result<Vec<Valve>, ParseError<LineCol>> {
    parser::valves(input)
}

peg::parser! {
    grammar parser() for str {
        rule name() -> String = n:$(['A'..='Z']*<2>) { n.to_owned() }
        rule int() -> u64 = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule names_list() -> Vec<String> = (name() ** ", ")
        rule valve() -> Valve = "Valve " name:name() " has flow rate=" flow_rate:int() "; tunnels lead to valves " leads_to:names_list() { Valve { name, flow_rate, leads_to } }
        pub rule valves() -> Vec<Valve> = (valve() ** "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paths() {
        assert_eq!(
            parser::valves(
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA"
            )
            .unwrap(),
            vec![
                Valve {
                    name: "AA".to_owned(),
                    flow_rate: 0,
                    leads_to: vec!["DD".to_owned(), "II".to_owned(), "BB".to_owned()]
                },
                Valve {
                    name: "BB".to_owned(),
                    flow_rate: 13,
                    leads_to: vec!["CC".to_owned(), "AA".to_owned()]
                },
            ]
        );
    }
}
