use std::{
    collections::HashMap,
    error::Error,
    io::{stdin, Read},
    str::FromStr,
};

#[derive(Debug, Copy, Clone)]
struct BitMask{
    ormask: u64,
    andmask: u64,
}

impl BitMask {
    fn apply(self, val: u64) -> u64 {
        (val | self.ormask) & self.andmask
    }
    fn new() -> Self {
        Self::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap()
    }
}

impl FromStr for BitMask {
    type Err = Box::<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut ormask  = 0b000000_000000_000000_000000_000000_000000u64;
        let mut andmask = 0b111111_111111_111111_111111_111111_111111u64;
        for (i, bit) in s.chars().enumerate() {
            match bit {
                'X' => {},
                '1' => { ormask |= 1 << (35 - i)},
                '0' => { andmask &= !(1 << (35 - i))},
                _ => Err(format!("weird mask bit: {}", bit))?,
            }
        }
        Ok(Self {
            ormask, andmask
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;

    let mut mem = HashMap::new();
    let mut mask = BitMask::new();

    for line in input.lines() {
        match &line[..3] {
            "mem" => {
                let addr =
                    line[4..(line.find(']').ok_or("missing ']' in mem instruction")?)].parse::<u64>()?;
                let val =
                    line[(line.find('=').ok_or("missing '=' in mem instruction")? + 2)..].parse::<u64>()?;

                mem.insert(addr, mask.apply(val));
            }
            "mas" => {
                mask = line[7..].parse::<BitMask>()?;
            }
            _ => Err(format!("unknown instruction: {}", line))?,
        }
    }

    println!("part 1: {}", mem.values().sum::<u64>());

    fn write_2(mem: &mut HashMap<u64, u64>, addr: u64, val: u64, mask: Vec<u8>) {
        match mask.iter().position(|&x| x == b'X') {
            Some(i) => {
                for bit in &[b'N', b'1'] {
                    let mut new_mask = mask.clone();
                    new_mask[i] = *bit;
                    write_2(mem, addr, val, new_mask);
                }
            },
            None => {
                let mut new_addr = addr;
                for (i, bit) in mask.iter().enumerate() {
                    match bit {
                        b'0' => {},
                        b'1' => { new_addr |= 1 << (35 - i)},
                        b'N' => { new_addr &= !(1 << (35 - i))},
                        _ => panic!("Weird mask bit: {}", bit),
                    }
                }
                mem.insert(new_addr, val);
            }
        }
    }

    let mut mem = HashMap::new();
    let mut mask = Vec::new();

    for line in input.lines() {
        match &line[..3] {
            "mem" => {
                let addr =
                    line[4..(line.find(']').ok_or("missing ']' in mem instruction")?)].parse::<u64>()?;
                let val =
                    line[(line.find('=').ok_or("missing '=' in mem instruction")? + 2)..].parse::<u64>()?;

                write_2(&mut mem, addr, val, mask.clone());
            }
            "mas" => {
                mask.clear();
                mask.extend_from_slice(&line[7..].as_bytes());
            }
            _ => Err(format!("unknown instruction: {}", line))?,
        }
    }

    println!("part 2: {}", mem.values().sum::<u64>());

    Ok(())
}
