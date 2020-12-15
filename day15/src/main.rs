fn game(starting_numbers: &[u32], num_turns: u32) -> u32 {
    let mut turn = 1;
    let mut last_spoken = 0;
    const NOT_SPOKEN: u32 = 0;
    let mut previously_spoken = vec![NOT_SPOKEN; num_turns as usize];

    // In this game, the players take turns saying numbers.
    macro_rules! say {
        ($n:expr) => {
            previously_spoken[last_spoken as usize] = turn;
            last_spoken = $n;
            if turn == num_turns {
                return $n;
            }
            turn += 1;
        };
    }

    // They begin by taking turns reading from a list of starting numbers (your puzzle input).
    for &n in starting_numbers {
        say!(n);
    }

    // Then, each turn consists of considering the most recently spoken number:
    loop {
        let n = match previously_spoken[last_spoken as usize] {
            // If that was the first time the number has been spoken, 
            NOT_SPOKEN => {
                // the current player says 0.
                0
            }
            // Otherwise, the number had been spoken before;
            previous_turn => {
                // the current player announces how many turns apart the number is
                // from when it was previously spoken.
                turn - previous_turn
            }
        };
        say!(n);
    }
    // (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)
}

fn main() {
    let input = &include!("../input.txt");

    println!("part 1: {}", game(input, 2020));
    println!("part 2: {}", game(input, 30_000_000));
}
