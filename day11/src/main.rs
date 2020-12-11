use std::{
    fmt::{self, Formatter, Display},
    io::{stdin, BufRead},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Occupied,
    Seat,
    Nothing,
}
use State::*;
impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Occupied => write!(f, "#"),
            Seat => write!(f, "L"),
            Nothing => write!(f, "."),
        }
    }
}

#[derive(Clone)]
struct World(Vec<Vec<State>>);

const DIRECTIONS : [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
impl World {
    fn get(&self, (y, x): (usize, usize)) -> State {
        *self
            .0
            .get(y)
            .map(|l| l.get(x).unwrap_or(&Nothing))
            .unwrap_or(&Nothing)
    }
    fn occupied_neighbors1(&self, (y, x): (usize, usize)) -> u8 {
        DIRECTIONS.iter().map(|(yoff, xoff)| {
                    self.get(((y as isize + yoff) as usize, (x as isize + xoff) as usize))
                })
            .filter(|&s| s == Occupied)
            .count() as u8
    }

    fn occupied_neighbors2(&self, (y, x): (usize, usize)) -> u8 {
        DIRECTIONS.iter().map(|(yoff, xoff)| {
            for dist in 1.. {
                match self.0.get((y as isize + dist * yoff) as usize).map(|l| l.get((x as isize + dist * xoff) as usize)) {
                    Some(Some(Occupied)) => return true,
                    Some(Some(Nothing)) => continue,
                    Some(Some(Seat)) | Some(None) | None => break,
                }
            }
            false
        })
            .filter(|&b| b)
            .count() as u8
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.0 {
            for s in line {
                write!(f, "{}", s)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Automaton {
    front: World,
    back: World,
}

impl Automaton {
    fn new(world: &World) -> Self {
        Self {
            front: world.clone(),
            back: world.clone(),
        }
    }
    fn run1(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for (y, line) in self.front.0.iter().enumerate() {
                for (x, state) in line.iter().enumerate() {
                    self.back.0[y][x] = match *state {
                        Nothing => Nothing,
                        Seat => {
                            if self.front.occupied_neighbors1((y, x)) == 0 {
                                changed = true;
                                Occupied
                            } else {
                                Seat
                            }
                        }
                        Occupied => {
                            if self.front.occupied_neighbors1((y, x)) >= 4 {
                                changed = true;
                                Seat
                            } else {
                                Occupied
                            }
                        }
                    };
                }
            }
            std::mem::swap(&mut self.front, &mut self.back);
        }
        println!("\n{}", self.front);
    }
    fn run2(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for (y, line) in self.front.0.iter().enumerate() {
                for (x, state) in line.iter().enumerate() {
                    self.back.0[y][x] = match *state {
                        Nothing => Nothing,
                        Seat => {
                            if self.front.occupied_neighbors2((y, x)) == 0 {
                                changed = true;
                                Occupied
                            } else {
                                Seat
                            }
                        }
                        Occupied => {
                            if self.front.occupied_neighbors2((y, x)) >= 5 {
                                changed = true;
                                Seat
                            } else {
                                Occupied
                            }
                        }
                    };
                }
            }
            std::mem::swap(&mut self.front, &mut self.back);
        }
        println!("\n{}", self.front);
    }
}

fn part1(world: &World) -> usize {
    let mut auto = Automaton::new(world);
    auto.run1();
    auto.front.0.iter().flatten().filter(|&&s| s == Occupied).count()
}

fn part2(world: &World) -> usize {
    let mut auto = Automaton::new(world);
    auto.run2();
    auto.front.0.iter().flatten().filter(|&&s| s == Occupied).count()
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let world = World(
        stdin
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        'L' => Seat,
                        _ => Nothing,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    println!("part 1: {}", part1(&world));
    println!("part 2: {}", part2(&world));
}