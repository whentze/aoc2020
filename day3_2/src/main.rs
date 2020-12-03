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

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let res : usize = slopes.into_iter().map(|(right, down)| {
        let mut xpos = 0;
        let mut count = 0;
        
        for line in map.iter().step_by(*down) {
            if line[xpos] == b'#' {
                count += 1;
            }
            xpos = (xpos + right) % width;
        }

        count
    }).product();

    println!("{}", res);

    Ok(())
}
