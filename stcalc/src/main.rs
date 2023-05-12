use std::io::{self, BufReader};

mod stack;

use crate::stack::TapeStack;

fn main() -> Result<(), String> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut stack = TapeStack::default();
    stack.parse(&mut reader)
}
