use peg::error::ParseError;
use peg::str::LineCol;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Instr {
    Forward(usize),
    Left,
    Right,
}

pub fn parse_instructions(input: &str) -> Result<Vec<Instr>, ParseError<LineCol>> {
    instr_parser::instructions(input)
}

peg::parser! {
    grammar instr_parser() for str {
        rule int() -> Instr = n:$(['0'..='9']+) { Instr::Forward(n.parse().unwrap()) }
        rule left() -> Instr = "L" { Instr::Left }
        rule right() -> Instr = "R" { Instr::Right }
        rule instr() -> Instr = int() / left() / right()
        pub rule instructions() -> Vec<Instr> = (instr() ** "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            parse_instructions("10R5L").unwrap(),
            vec![
                Instr::Forward(10),
                Instr::Right,
                Instr::Forward(5),
                Instr::Left
            ]
        )
    }
}
