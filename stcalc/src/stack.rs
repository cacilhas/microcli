use std::{
    io::{prelude::*, BufReader},
    num::ParseFloatError,
};

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
                self.parse_token(token);
            }
            buffer.clear();
        }
    }

    fn parse_token(&mut self, token: &str) {
        match token {
            "+" => self.run_add(),
            "-" => self.run_invert_signal(),
            "*" => self.run_multiply(),
            "/" => self.run_invert_number(),
            "!" => self.run_pop(),
            "." => self.run_print_character(),
            "=" => self.run_print(),
            _ => {
                let float: Result<f32, ParseFloatError> = token.parse();
                let float = float.unwrap();
                self.0.push(float);
            }
        }
    }

    fn run_add(&mut self) {
        if self.0.len() < 2 {
            Err::<(), &str>("stack is empty").unwrap();
        }

        let value = self.0.pop().unwrap();
        let last = self.0.len() - 1;
        self.0[last] += value;
    }

    fn run_multiply(&mut self) {
        if self.0.len() < 2 {
            Err::<(), &str>("stack is empty").unwrap();
        }

        let value = self.0.pop().unwrap();
        let last = self.0.len() - 1;
        self.0[last] *= value;
    }

    fn run_print(&self) {
        print!("{}", self.0.last().expect("stack is empty"));
    }

    fn run_pop(&mut self) {
        let _ = self.0.pop();
    }

    fn run_invert_signal(&mut self) {
        let len = self.0.len();
        let value = self.0.last().expect("stack is empty");
        self.0[len - 1] = value * -1.0;
    }

    fn run_invert_number(&mut self) {
        let len = self.0.len();
        let value = self.0.last().expect("stack is empty");
        self.0[len - 1] = 1.0 / value;
    }

    fn run_print_character(&self) {
        let byte = *self.0.last().expect("stack is empty") as u8 as char;
        print!("{byte}");
    }
}

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
        stack.parse_token("123");
        stack.parse_token("24.5");
        stack.parse_token("-5.25");
        assert_eq!(stack.0.len(), 3);
        assert_eq!(stack.0[0], 123.0);
        assert_eq!(stack.0[1], 24.5);
        assert_eq!(stack.0[2], -5.25);
    }

    #[test]
    fn it_should_unstack_adding() {
        let mut stack = TapeStack::default();
        stack.parse_token("1");
        stack.parse_token("2");
        stack.parse_token("3");
        stack.parse_token("+");
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 1.0);
        assert_eq!(stack.0[1], 5.0);
    }

    #[test]
    fn it_should_invert_signal() {
        let mut stack = TapeStack::default();
        stack.parse_token("1");
        stack.parse_token("2");
        stack.parse_token("-");
        stack.parse_token("3.5");
        stack.parse_token("-");
        assert_eq!(stack.0.len(), 3);
        assert_eq!(stack.0[0], 1.0);
        assert_eq!(stack.0[1], -2.0);
        assert_eq!(stack.0[2], -3.5);
    }

    #[test]
    fn it_should_invert_number() {
        let mut stack = TapeStack::default();
        stack.parse_token("2");
        stack.parse_token("4");
        stack.parse_token("/");
        assert_eq!(stack.0.len(), 2);
        assert_eq!(stack.0[0], 2.0);
        assert_eq!(stack.0[1], 0.25);
    }
}
