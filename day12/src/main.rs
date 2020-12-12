use std::{
    error::Error,
    io::{stdin, Read},
    str::from_utf8,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

trait Ship {
    fn new() -> Self;
    fn north(&mut self, a: i64);
    fn east(&mut self, a: i64);
    fn south(&mut self, a: i64);
    fn west(&mut self, a: i64);
    fn forward(&mut self, a: i64);
    fn turn_right(&mut self, a: i64);
    fn manhattan_distance(&self) -> u64;
}

#[derive(Debug)]
struct Ship1 {
    x: i64,
    y: i64,
    dir: Direction,
}

impl Ship for Ship1 {
    fn new() -> Self {
        Self {
            x: 0, y: 0,
            dir: East,
        }
    }
    fn north(&mut self, a: i64) {
        self.y += a;
    }
    fn east(&mut self, a: i64) {
        self.x += a;
    }
    fn south(&mut self, a: i64) {
        self.y -= a;
    }
    fn west(&mut self, a: i64) {
        self.x -= a;
    }
    fn forward(&mut self, a: i64) {
        match self.dir {
            North => self.y += a,
            East  => self.x += a,
            South => self.y -= a,
            West  => self.x -= a,
        };
    }
    fn turn_right(&mut self, mut a: i64) {
        while a > 0 {
            self.dir = match self.dir {
                North => East,
                East  => South,
                South => West,
                West  => North,
            };
            a -= 90;
        }
    }
    fn manhattan_distance(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
}

#[derive(Debug)]
struct Ship2 {
    x: i64,
    y: i64,
    wx: i64,
    wy: i64,
}

impl Ship for Ship2 {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }
    fn north(&mut self, a: i64) {
        self.wy += a;
    }
    fn east(&mut self, a: i64) {
        self.wx += a;
    }
    fn south(&mut self, a: i64) {
        self.wy -= a;
    }
    fn west(&mut self, a: i64) {
        self.wx -= a;
    }
    fn forward(&mut self, a: i64) {
        self.x += self.wx * a;
        self.y += self.wy * a;
    }
    fn turn_right(&mut self, mut a: i64) {
        while a > 0 {
            std::mem::swap(&mut self.wx, &mut self.wy);
            self.wy = -self.wy;
            a -= 90;
        }
    }
    fn manhattan_distance(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
}

fn move_ship<S: Ship>(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut ship = S::new();

    for line in input.lines() {
        match line.as_bytes() {
            [command, amount @ ..] => {
                let amount = from_utf8(amount)?.parse::<i64>()?;

                match command {
                    b'R' => ship.turn_right(amount),
                    b'L' => ship.turn_right(360 - amount),
                    b'F' => ship.forward(amount),
                    b'N' => ship.north(amount),
                    b'E' => ship.east(amount),
                    b'S' => ship.south(amount),
                    b'W' => ship.west(amount),
                    x => Err(format!("Unknown command: {}", *x as char))?,
                };
            }
            b"" => Err("Empty input")?,
        };
    }
    Ok(ship.manhattan_distance())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;

    println!("part 1: {}", move_ship::<Ship1>(&input)?);
    println!("part 2: {}", move_ship::<Ship2>(&input)?);

    Ok(())
}