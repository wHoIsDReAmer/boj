use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut br = BufReader::new(stdin.lock());
    let mut bw = BufWriter::new(stdout.lock());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();

    let n = line.trim().parse::<i32>().unwrap();

    let mut result = 0;
    'outer: for i in 0..=n/5 {
        for j in 0..n/3 {
            if j*3 + i*5 == n {
                result = i+j;
                break 'outer
            }
        }
    }

    if result == 0 {
        bw.write_all(b"-1").unwrap();
    } else {
        bw.write_all(format!("{}", result).as_bytes()).unwrap();
    }
}
