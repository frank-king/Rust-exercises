use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

fn run<R: BufRead, W: Write>(mut input: Scanner<R>, mut output: W) -> Result<(), Box<dyn Error>> {
    let _input = input;
    let _output = output;
    Ok(())
}

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let input = Scanner::from(stdin.lock());
    let output = stdout.lock();
    run(input, output).unwrap();
}

pub struct Scanner<B> {
    reader: B,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

impl<R: BufRead> From<R> for Scanner<R> {
    fn from(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }
}
