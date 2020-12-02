use std::{
    error::Error,
    io::{BufRead, stdin},
};

fn is_valid(line: &str) -> Result<bool, Box<dyn Error>> {
    let mut pieces = line.split_whitespace();

    let mut counts = pieces.next()
        .ok_or("Line is empty")?
        .split("-");
    let first_idx = counts.next()
    .ok_or("Missing first_idx part of policy")?
        .parse::<usize>()?;
    let second_idx = counts.next()
        .ok_or("Missing second_idx part of policy")?
        .parse::<usize>()?;

    let letter = pieces.next()
        .ok_or("Missing letter part of policy")?
        .strip_suffix(":").ok_or("Missing ':' after policy")?;

    let passwd = pieces.next()
        .ok_or("Missing password")?;

    let first = passwd.chars().nth(first_idx - 1)
        .ok_or("Password too short for first idx")?;

    let second = passwd.chars().nth(second_idx - 1)
        .ok_or("Password too short for second idx")?;
    Ok((first.to_string() == letter) ^ (second.to_string() == letter))
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