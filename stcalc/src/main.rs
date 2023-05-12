use std::io::{self, BufReader};

mod stack;

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut stack = stack::TapeStack::default();
    stack.parse(&mut reader);
}
