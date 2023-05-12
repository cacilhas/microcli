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
