use std::{
    error::Error,
    io::{stdin, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    let input = stdin
        .lines()
        .map(|l| Ok(l?.parse::<u64>()?))
        .collect::<Result<Vec<u64>, Box<dyn Error>>>()?;

    let part1 = *input
        .windows(26)
        .find_map(|w| {
            if let [previous @ .., current] = w {
                if previous.into_iter().enumerate().any(|(i, x)| {
                    previous[i..]
                        .into_iter()
                        .any(|y| (x + y) == *current && x != y)
                }) {
                    None
                } else {
                    Some(current)
                }
            } else {
                panic!("Iterator::windows is broken!")
            }
        })
        .ok_or("No result in part 1 :(")?;

    println!("part 1: {}", part1);

    let part2 = (0..input.len())
        .map(|i| {
            (i..input.len())
                .map(|j| &input[i..=j])
                .take_while(|s| s.iter().sum::<u64>() <= part1)
                .find_map(|s| {
                    if s.iter().sum::<u64>() == part1 {
                        Some(s.iter().min().unwrap() + s.iter().max().unwrap())
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .next()
        .ok_or("No result in part 2 :(")?;

    println!("part 2: {}", part2);

    Ok(())
}