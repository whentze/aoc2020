use std::{
    error::Error,
    io::{BufRead, stdin},
};

fn is_valid(line: &str) -> Result<bool, Box<dyn Error>> {
    let mut pieces = line.split_whitespace();

    let mut counts = pieces.next()
        .ok_or("Line is empty")?
        .split("-");
    let min = counts.next()
    .ok_or("Missing min part of policy")?
        .parse::<usize>()?;
    let max = counts.next()
        .ok_or("Missing max part of policy")?
        .parse::<usize>()?;

    let letter = pieces.next()
        .ok_or("Missing letter part of policy")?
        .strip_suffix(":").ok_or("Missing ':' after policy")?;

    let passwd = pieces.next()
        .ok_or("Missing password")?;

    let occurences = passwd.matches(letter).count();

    Ok((min..=max).contains(&occurences))
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut count = 0;
    for line in stdin.lines() {
        if is_valid(&line?)? {
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}