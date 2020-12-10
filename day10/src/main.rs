use std::{
    error::Error,
    io::{BufRead, stdin},
};

fn part1(input: &[usize]) -> usize {
    let mut diffs = [0; 4];
    for w in input.windows(2) {
        diffs[w[1] - w[0]] += 1;
    }
    diffs[1] * diffs[3]
}

fn part2(input: &[usize]) -> usize {
    let mut cache = vec![0; *input.last().unwrap()];
    cache.push(1);

    for x in input.into_iter().rev().skip(1) {
        cache[*x] = cache[x+1] + cache[x+2] + cache[x+3];
    }
    cache[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    // Adapters
    let mut input = stdin
        .lines()
        .map(|l| Ok(l?.parse::<usize>()?))
        .collect::<Result<Vec<usize>, Box<dyn Error>>>()?;

    // Outlet
    input.push(0);

    // Device
    input.sort();
    input.push(input.last().unwrap() + 3);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}