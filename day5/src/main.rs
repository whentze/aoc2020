use std::{
    cmp::max,
    io::{BufRead, stdin},
};

fn seat_id(s: &str) -> u32 {
    s.chars().map(|c| match c {
        'B' | 'R' => 1,
        'F' | 'L' => 0,
        _ => panic!("unexpected character {}", c),
    }).fold(0, |acc, x| acc * 2 + x)
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut max_id = 0;
    let mut min_id = 1024;
    let mut mask = 0x0;
    for line in stdin.lines() {
        let id = seat_id(&line.unwrap());

        mask ^= id;

        max_id = max(id, max_id);
        min_id = max(id, min_id);
    }

    for id in (0..min_id)
        .chain((max_id+1)..1024) {
        mask ^= id
    }

    println!("max id: {}", max_id);
    println!("free seat: {}", mask);
}
