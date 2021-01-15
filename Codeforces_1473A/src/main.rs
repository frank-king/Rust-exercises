use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

#[cfg(test)]
mod tests;

fn run<R: BufRead, W: Write>(mut input: Scanner<R>, mut output: W) -> Result<(), Box<dyn Error>> {
    let t: u32 = input.scan();
    for _ in 0..t {
        let n: usize = input.scan();
        let d: u32 = input.scan();
        let mut arr = Vec::with_capacity(n);
        for _ in 0..n {
            arr.push(input.scan());
        }
        println!("{:?}", arr);
        let yes = *arr.iter().max().unwrap_or(&0) <= d || {
            let (min, min2) = arr.iter().fold(
                (u32::max_value(), u32::max_value()),
                |(mut min, mut min2), number| {
                    if *number <= min {
                        min2 = min;
                        min = *number;
                    } else if *number <= min2 {
                        min2 = *number;
                    }
                    println!("min = {}, min2 = {}, number = {}", min, min2, *number);
                    (min, min2)
                },
            );
            min + min2 <= d
        };
        writeln!(output, "{}", if yes { "YES" } else { "NO" })?;
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
