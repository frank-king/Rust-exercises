use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

#[cfg(test)]
mod tests;

fn solve(mut bad_tiles: Vec<(u32, u32)>) -> bool {
    if bad_tiles.len() % 2 != 0 {
        return false;
    }
    bad_tiles.sort();
    let mut last: Option<(u32, u32)> = None;
    let mut iter = bad_tiles.into_iter().peekable();
    while let Some((column, row)) = iter.next() {
        if row == 1 {
            if let Some((next_column, _)) = iter.peek() {
                if *next_column == column {
                    if last.is_some() {
                        return false;
                    }
                    iter.next();
                    continue;
                }
            }
        }
        last = match last.take() {
            None => Some((column, row)),
            Some((last_column, last_row)) => {
                if (last_row == row) == ((column - last_column) % 2 == 0) {
                    return false;
                }
                None
            }
        };
    }
    last.is_none()
}

fn run<R: BufRead, W: Write>(mut input: Scanner<R>, mut output: W) -> Result<(), Box<dyn Error>> {
    let t: u32 = input.scan();
    for _ in 0..t {
        let _n: u32 = input.scan();
        let m: usize = input.scan();
        let bad_tiles: Vec<(u32, u32)> = (0..m)
            .map(|_| (input.scan(), input.scan()))
            .map(|(a, b)| (b, a))
            .collect();
        writeln!(output, "{}", if solve(bad_tiles) { "YES" } else { "NO" })?;
    }
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
