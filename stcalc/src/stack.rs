use std::{
    fmt,
    io::{prelude::*, BufReader},
    num::ParseFloatError,
};

use IO::*;

#[derive(Debug, Default)]
pub struct TapeStack(Vec<f32>);

impl TapeStack {
    pub fn parse(&mut self, reader: &mut BufReader<impl Read>) {
        let mut buffer = String::new();
        while let Ok(input) = reader.read_line(&mut buffer) {
            if input == 0 {
                break;
            }
            for token in buffer.split_whitespace() {
                match self.parse_token(token).unwrap() {
                    Print(something) => print!("{}", something),
                    Nop => (),
                }
            }
            buffer.clear();
        }
    }

    fn parse_token(&mut self, token: &str) -> SRes<IO> {
        match token {
            "+" => {
                self.run_add()?;
                Ok(Nop)
            }
            "-" => {
                self.run_invert_signal()?;
                Ok(Nop)
            }
            "*" => {
                self.run_multiply()?;
                Ok(Nop)
            }
            "/" => {
                self.run_invert_number()?;
                Ok(Nop)
            }
            "!" => {
                self.run_pop()?;
                Ok(Nop)
            }
            "=" => self.run_print(),
            "." => self.run_print_character(),
            _ => {
                let float: Result<f32, ParseFloatError> = token.parse();
                let float = float.unwrap();
                self.0.push(float);
                Ok(Nop)
            }
        }
    }

    fn run_add(&mut self) -> SRes<()> {
        if self.0.len() < 2 {
            return Err("stack is empty");
        }

        let value = self.0.pop().ok_or("stack is empty")?;
        let last = self.0.len() - 1;
        self.0[last] += value;
        Ok(())
    }

    fn run_multiply(&mut self) -> SRes<()> {
        if self.0.len() < 2 {
            return Err("stack is empty");
        }

        let value = self.0.pop().ok_or("stack is empty")?;
        let last = self.0.len() - 1;
        self.0[last] *= value;
        Ok(())
    }

    fn run_pop(&mut self) -> SRes<()> {
        self.0.pop().ok_or("stack is empty")?;
        Ok(())
    }

    fn run_invert_signal(&mut self) -> SRes<()> {
        let len = self.0.len();
        let value = self.0.last().ok_or("stack is empty")?;
        self.0[len - 1] = value * -1.0;
        Ok(())
    }

    fn run_invert_number(&mut self) -> SRes<()> {
        let len = self.0.len();
        let value = self.0.last().ok_or("stack is empty")?;
        self.0[len - 1] = 1.0 / value;
        Ok(())
    }

    fn run_print(&self) -> SRes<IO> {
        let value = *self.0.last().ok_or("stack is empty")?;
        Ok(Print(Box::new(value)))
    }

    fn run_print_character(&self) -> SRes<IO> {
        let byte = *self.0.last().ok_or("stack is empty")? as u8 as char;
        Ok(Print(Box::new(byte)))
    }
}

enum IO {
    Nop,
    Print(Box<dyn fmt::Display>),
}

type SRes<T> = Result<T, &'static str>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_an_empty_stack() {
        let stack = TapeStack::default();
        assert_eq!(stack.0.len(), 0);
    }

    #[test]
    fn it_should_push_number_into_stack_top() {
        let mut stack = TapeStack::default();
        stack.parse_token("123").unwrap();
        stack.parse_token("24.5").unwrap();
        stack.parse_token("-5.25").unwrap();
        assert_eq!(stack.0.len(), 3);
        assert_eq!(stack.0[0], 123.0);
        assert_eq!(stack.0[1], 24.5);
        assert_eq!(stack.0[2], -5.25);
    }

    #[test]
    fn it_should_unstack_adding() {
        let mut stack = TapeStack::default();
        stack.parse_token("1").unwrap();
        stack.parse_token("2").unwrap();
        stack.parse_token("3").unwrap();
        stack.parse_token("+").unwrap();
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 1.0);
        assert_eq!(stack.0[1], 5.0);
    }

    #[test]
    fn it_should_invert_signal() {
        let mut stack = TapeStack::default();
        stack.parse_token("1").unwrap();
        stack.parse_token("2").unwrap();
        stack.parse_token("-").unwrap();
        stack.parse_token("3.5").unwrap();
        stack.parse_token("-").unwrap();
        assert_eq!(stack.0.len(), 3);
        assert_eq!(stack.0[0], 1.0);
        assert_eq!(stack.0[1], -2.0);
        assert_eq!(stack.0[2], -3.5);
    }

    #[test]
    fn it_should_invert_number() {
        let mut stack = TapeStack::default();
        stack.parse_token("2").unwrap();
        stack.parse_token("4").unwrap();
        stack.parse_token("/").unwrap();
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 2.0);
        assert_eq!(stack.0[1], 0.25);
    }

    #[test]
    fn it_should_pop_from_stack() {
        let mut stack = TapeStack::default();
        stack.parse_token("2").unwrap();
        stack.parse_token("4").unwrap();
        stack.parse_token("5").unwrap();
        stack.parse_token("!").unwrap();
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 2.0);
        assert_eq!(stack.0[1], 4.0);
    }

    #[test]
    fn it_should_return_last_number() {
        let mut stack = TapeStack::default();
        stack.parse_token("2").unwrap();
        stack.parse_token("4").unwrap();
        match stack.parse_token("=").unwrap() {
            Print(value) => {
                let s = format!("{value}");
                assert_eq!(s, "4");
            }
            Nop => Err("received Nop, expected Print(4.0)").unwrap(),
        }
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 2.0);
        assert_eq!(stack.0[1], 4.0);
    }

    #[test]
    fn it_should_return_float() {
        let mut stack = TapeStack::default();
        stack.parse_token("2.25").unwrap();
        match stack.parse_token("=").unwrap() {
            Print(value) => {
                let s = format!("{value}");
                assert_eq!(s, "2.25");
            }
            Nop => Err("received Nop, expected Print(2.25)").unwrap(),
        }
        assert_eq!(stack.0.len(), 1);
        assert_eq!(stack.0[0], 2.25);
    }

    #[test]
    fn it_should_return_last_value_as_character() {
        let mut stack = TapeStack::default();
        stack.parse_token("0").unwrap();
        stack.parse_token("65").unwrap();
        match stack.parse_token(".").unwrap() {
            Print(value) => {
                let s = format!("{value}");
                assert_eq!(s, "A");
            }
            Nop => Err("received Nop, expected Print('A')").unwrap(),
        }
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 0.0);
        assert_eq!(stack.0[1], 65.0);
    }

    #[test]
    fn it_should_return_ignore_decimal() {
        let mut stack = TapeStack::default();
        stack.parse_token("0").unwrap();
        stack.parse_token("66.75").unwrap();
        match stack.parse_token(".").unwrap() {
            Print(value) => {
                let s = format!("{value}");
                assert_eq!(s, "B");
            }
            Nop => Err("received Nop, expected Print('B')").unwrap(),
        }
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 0.0);
        assert_eq!(stack.0[1], 66.75);
    }
}
