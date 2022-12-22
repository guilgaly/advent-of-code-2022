use std::collections::HashMap;

pub type Monkeys = HashMap<String, Job>;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Job {
    Value(i64),
    Op(String, Operation, String),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    pub fn apply(&self, v1: i64, v2: i64) -> i64 {
        match self {
            Operation::Add => v1 + v2,
            Operation::Sub => v1 - v2,
            Operation::Mul => v1 * v2,
            Operation::Div => v1 / v2,
        }
    }
}
