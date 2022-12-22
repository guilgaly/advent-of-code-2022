use crate::models::{Job, Monkeys, Operation};
use peg::error::ParseError;
use peg::str::LineCol;

pub fn parse_monkeys(input: &str) -> Result<Monkeys, ParseError<LineCol>> {
    parser::monkeys(input)
}

peg::parser! {
    grammar parser() for str {
        rule name() -> String = n:$(['a'..='z']+) { n.to_owned() }
        rule int() -> i64 = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule add_operation() -> Operation = "+" { Operation::Add }
        rule sub_operation() -> Operation = "-" { Operation::Sub }
        rule mul_operation() -> Operation = "*" { Operation::Mul }
        rule div_operation() -> Operation = "/" { Operation::Div }
        rule operation() -> Operation = add_operation() / sub_operation() / mul_operation() / div_operation()
        rule value_job() -> Job = n:int() { Job::Value(n) }
        rule op_job() -> Job = n1:name() " " o:operation() " " n2:name() { Job::Op(n1, o, n2) }
        rule job() -> Job = value_job() / op_job()
        rule monkey() -> (String, Job) = name:name() ": " job:job() { (name, job) }
        pub rule monkeys() -> Monkeys = n:(monkey() ** "\n") { n.into_iter().collect() }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_paths() {
        let mut expected = HashMap::new();
        expected.insert("root".to_owned(), Job::Op("pppw".to_owned(), Operation::Add, "sjmn".to_owned()));
        expected.insert("dbpl".to_owned(), Job::Value(5));
        expected.insert("ptdq".to_owned(), Job::Op("humn".to_owned(), Operation::Sub, "dvpt".to_owned()));
        expected.insert("pppw".to_owned(), Job::Op("cczh".to_owned(), Operation::Div, "lfqf".to_owned()));
        expected.insert("lgvd".to_owned(), Job::Op("ljgn".to_owned(), Operation::Mul, "ptdq".to_owned()));

        assert_eq!(
            parse_monkeys(
                "root: pppw + sjmn
dbpl: 5
ptdq: humn - dvpt
pppw: cczh / lfqf
lgvd: ljgn * ptdq"
            )
            .unwrap(),
            expected
        )
    }
}
