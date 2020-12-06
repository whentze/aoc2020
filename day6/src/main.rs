use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut sum_any = 0;
    let mut sum_all = 0;

    let mut lines = stdin.split(b'\n');
    while let Some(Ok(mut line)) = lines.next() {
        let mut hitmap = [0u8; 26];
        let mut num_lines = 0;
        while line != b"" {
            num_lines += 1;
            for c in line {
                hitmap[(c - b'a') as usize] += 1;
            }
            line = match lines.next() {
                Some(Ok(line)) => line,
                _ => break,
            };
        }
        sum_any += hitmap.iter().filter(|&&x| x > 0).count();
        sum_all += hitmap.iter().filter(|&&x| x == num_lines).count();
    }

    println!("sum_any: {}", sum_any);
    println!("sum_all: {}", sum_all);
}
