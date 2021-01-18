use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

#[cfg(test)]
mod tests;

fn run<R: BufRead, W: Write>(mut input: Scanner<R>, mut output: W) -> Result<(), Box<dyn Error>> {
    let t: u32 = input.scan();
    for _ in 0..t {
        let n: usize = input.scan();
        let arr: Vec<usize> = (0..n).map(|_| input.scan()).collect();
        let mut max_jump = arr.clone();
        let mut ans = 0;
        for i in (0..n).rev() {
            if i + arr[i] < n {
                max_jump[i] = max_jump[i].max(arr[i] + max_jump[i + arr[i]]);
            }
            ans = ans.max(max_jump[i]);
        }
        writeln!(output, "{}", ans)?;
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
