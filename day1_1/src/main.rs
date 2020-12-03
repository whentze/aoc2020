use std::{
    error::Error,
    io::{BufRead, stdin},
    str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut hitmap = [false; 2021];

    for line in stdin.lines() {
        let num = usize::from_str(&line?)?;

        if hitmap[2020-num] {
            println!("{}", num * (2020 - num));
            return Ok(())
        } else if num <= 2020 {
            hitmap[num] = true;
        }
    }

    Err("No pair of number sums to 2020".into())
}
