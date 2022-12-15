use peg::error::ParseError;
use std::fmt::Display;

pub fn eval_packet(expression: &str) -> Result<Data, String> {
    packet::list(expression).map_err(|e| fmt_err(expression, &e))
}

fn fmt_err<T: Display>(expression: &str, error: &ParseError<T>) -> String {
    format!("Cannot evaluate '{}'; {}", expression, error)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Int(u8),
    List(Vec<Data>),
}

peg::parser! {
    grammar packet() for str {
        pub rule int() -> Data = n:$(['0'..='9']+) { Data::Int(n.parse().unwrap()) }
        pub rule list() -> Data = "[" n:(data() ** ",") "]" { Data::List(n) }
        pub rule data() -> Data = int() / list()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(packet::int("0").unwrap(), Data::Int(0));
        assert!(packet::int("a").is_err());
    }

    #[test]
    fn test_parse_int_list() {
        assert_eq!(
            packet::list("[0,1,2]").unwrap(),
            Data::List(vec![Data::Int(0), Data::Int(1), Data::Int(2)])
        );
    }

    #[test]
    fn test_parse_packet() {
        assert_eq!(
            packet::list("[0,[1,[2,3],4]]").unwrap(),
            Data::List(vec![
                Data::Int(0),
                Data::List(vec![
                    Data::Int(1),
                    Data::List(vec![Data::Int(2), Data::Int(3)]),
                    Data::Int(4)
                ])
            ])
        );
    }
}
