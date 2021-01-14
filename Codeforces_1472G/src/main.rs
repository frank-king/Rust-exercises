use std::collections::{LinkedList, VecDeque};
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Graph {
    dist: Vec<usize>,
    edges: Vec<LinkedList<usize>>,
}

fn solve(n: usize, mut graph: Graph) -> Vec<usize> {
    fn shortest_path(graph: &mut Graph) {
        let mut queue = VecDeque::new();
        graph.dist[0] = 0;
        queue.push_back(0);
        while let Some(u) = queue.pop_front() {
            for v in graph.edges[u].iter() {
                if *v != 0 && graph.dist[*v] == 0 {
                    graph.dist[*v] = graph.dist[u] + 1;
                    queue.push_back(*v);
                }
            }
        }
    }
    shortest_path(&mut graph);
    let mut nearest = vec![usize::max_value(); n];
    fn find_nearest(graph: &Graph, nearest: &mut Vec<usize>, u: usize) -> usize {
        if nearest[u] < usize::max_value() {
            return nearest[u];
        }
        nearest[u] = graph.dist[u];
        for v in graph.edges[u].iter() {
            nearest[u] = nearest[u].min(if graph.dist[u] < graph.dist[*v] {
                find_nearest(graph, nearest, *v)
            } else {
                graph.dist[*v]
            });
        }
        nearest[u]
    }
    find_nearest(&graph, &mut nearest, 0);
    nearest
}

fn run<R: BufRead, W: Write>(mut input: Scanner<R>, mut output: W) -> Result<(), Box<dyn Error>> {
    let t: usize = input.scan();
    for _ in 0..t {
        let n: usize = input.scan();
        let m: usize = input.scan();
        let dest = vec![0; n];
        let edges = vec![LinkedList::new(); n];
        let mut graph = Graph { dist: dest, edges };
        for _ in 0..m {
            let u: usize = input.scan();
            let v: usize = input.scan();
            graph.edges[u - 1].push_back(v - 1);
        }
        let result = solve(n, graph);
        for item in result {
            write!(output, "{} ", item)?;
        }
        writeln!(output)?;
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
