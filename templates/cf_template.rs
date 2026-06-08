use std::io::{self, BufRead, Write, BufWriter};

fn solve(input: &[i64]) -> i64 {
    0
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let input: Vec<i64> = line.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        writeln!(out, "{}", solve(&input)).unwrap();
    }
}
