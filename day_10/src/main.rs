use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = parse_input(INPUT)?;

    println!("Part 1 result: {}", execute(&instructions));
    // println!("Part 2 result: {}", part_2(&moves));

    Ok(())
}

fn execute(instructions: &[Instruction]) -> i64 {
    let mut cycle = 0;
    let mut register = 1;
    let mut counter = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                step(&mut cycle, register, &mut counter);
            }
            Instruction::Addx(v) => {
                step(&mut cycle, register, &mut counter);
                step(&mut cycle, register, &mut counter);
                register += v;
            }
        }
    }
    counter
}

fn step(cycle: &mut i64, register: i64, counter: &mut i64) {
    let beam = *cycle % 40;
    if beam >= register - 1 && beam <= register + 1 {
        print!("#");
    } else {
        print!(".");
    }
    if beam == 39 {
        println!();
    }
    *cycle += 1;
    if (*cycle + 20) % 40 == 0 {
        *counter += register * *cycle;
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, String> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("noop") {
                Ok(Instruction::Noop)
            } else if line.starts_with("addx ") {
                let value = line
                    .split_at(5)
                    .1
                    .parse::<i64>()
                    .map_err(|_| format!("Invalid: {}", line))?;
                Ok(Instruction::Addx(value))
            } else {
                Err(format!("Invalid: {}", line))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_instructions() -> Vec<Instruction> {
        let test_input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        parse_input(test_input).unwrap()
    }

    #[test]
    fn test_parse_input() {
        let actual = parse_input(
            "noop
addx 3
addx -5",
        )
        .unwrap();
        let expected = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(execute(&test_instructions()), 13140);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&test_moves()), 1);
    // }
}
