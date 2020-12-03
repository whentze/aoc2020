use std::{
    error::Error,
    io::{stdin, BufRead},
    str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    let map: Vec<Vec<u8>> = stdin
        .split(b'\n')
        .collect::<Result<Vec<_>, _>>()?;
    
    let width = map[0].len();
    let mut xpos = 0;
    let mut count = 0;
    
    for line in map {
        if line[xpos] == b'#' {
            count += 1;
        }
        xpos = (xpos + 3) % width;
    }

    println!("{}", count);
}
