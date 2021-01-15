use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

#[cfg(test)]
mod tests;

fn run<R: BufRead, W: Write>(mut input: Scanner<R>, mut output: W) -> Result<(), Box<dyn Error>> {
    let t: u32 = input.scan();
    for _ in 0..t {
        let _n: usize = input.scan();
        let m: u32 = input.scan();
        let code: String = input.scan();
        // println!("{}", code);
        fn scan((current, min, max): &mut (i32, i32, i32), code: char) -> Option<(i32, i32, i32)> {
            match code {
                '+' => *current += 1,
                '-' => *current -= 1,
                _ => unreachable!(),
            }
            *min = (*min).min(*current);
            *max = (*max).max(*current);
            Some((*current, *min, *max))
        };
        let range_left: Vec<_> = std::iter::once((0, 0, 0))
            .chain(code.chars().scan((0, 0, 0), scan))
            .collect();
        let mut range_right: Vec<_> = std::iter::once((0, 0, 0))
            .chain(code.chars().rev().scan((0, 0, 0), scan))
            .map(|(current, min, max)| (-current, -max, -min))
            .collect();
        range_right.reverse();
        // println!("range_left = {:?}", range_left);
        // println!("range_right = {:?}", range_right);
        for _ in 0..m {
            let begin = input.scan::<usize>() - 1;
            let end = input.scan::<usize>();
            let left = (range_left[begin].1, range_left[begin].2);
            let diff = range_right[end].0 - range_left[begin].0;
            let right = (range_right[end].1 - diff, range_right[end].2 - diff);
            let min = left.0.min(right.0);
            let max = left.1.max(right.1);
            let len = max - min + 1;
            writeln!(output, "{}", len)?;
        }
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
