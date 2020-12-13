use ring_algorithm::chinese_remainder_theorem;
use std::{
    error::Error,
    io::{stdin, Read},
};

fn wait_for(bus: i64, departure: i64) -> i64 {
    bus - (departure % bus)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let departure : i64 = lines.next().ok_or("empty input")?.parse()?;
    let busses : Vec<(i64, i64)> = lines
        .next()
        .ok_or("no second line in input")?
        .split(",")
        .enumerate()
        .filter_map(|(i, n)| n.parse().ok().map(|n| (i as i64, n)))
        .collect();

    let earliest = busses.iter().min_by_key(|&&(_, bus)| wait_for(bus, departure))
        .ok_or("no bus?")?.1;

    println!("part 1: {}", earliest * wait_for(earliest, departure));

    // We need to find a nonnegative result, so we will need M from the
    // chinese remainder theorem, which `ring_algorithm` doesn't give us.
    // Instead, we can be lazy and just multiply up all the m,
    // reyling on the fact that they're coprime (which they are, luckily).
    let mut m_all = 1;

    let (u, m) : (Vec<i64>, Vec<i64>) = busses.iter().map(|&(i, b)| {
        // sneak this in so we don't need to iterate twice
        m_all *= b;

        // We're looking for t s.t. t + i ≡ 0 mod b_i for all i,
        // so we demand t ≡ -i mod b_i. 
        (b-i, b)
    }).unzip();

    let mut t = chinese_remainder_theorem(&u, &m).ok_or("CRT failed?!")?;

    while t < 0 {
        t += m_all;
    }

    println!("part 2: {}", t);

    Ok(())
}
