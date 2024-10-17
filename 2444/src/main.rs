use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut br = BufReader::new(stdin.lock());
    let mut bw = BufWriter::new(stdout.lock());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();

    for i in 0..n {
        let space = " ".repeat(n - i - 1);
        let stars = "*".repeat(2 * i + 1);
        writeln!(bw, "{}{}", space, stars).unwrap();
    }

    for i in 1..n {
        let space = " ".repeat(i);
        let stars = "*".repeat(2 * (n - i) - 1);
        writeln!(bw, "{}{}", space, stars).unwrap();
    }
}
