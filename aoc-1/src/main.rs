use aoc_lib::load_simple_input;

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use itertools::Itertools;

/// Solution for Advent of Code 2020 day 1
/// Makes use of O(1) lookup for HashSets to give approximately O(n) behaviour for both parts
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let inputs: HashSet<usize> = load_simple_input("inputs/1.txt")?;

    let mut out = None;

    let part1_start = Instant::now();
    for &input in inputs.iter() {
        let remainder = 2020 - input;
        if inputs.contains(&remainder) {
            out.replace((input, remainder));
            break;
        }
    }
    let part1_dur = part1_start.elapsed();

    match out {
        Some((a, b)) => println!(
            "Answer to part 1: {} * {} = {}\nTook {}us",
            a,
            b,
            a * b,
            part1_dur.as_micros()
        ),
        None => println!("No pair adds to 2020"),
    }

    let mut out = None;

    let part2_start = Instant::now();
    let remainder_pairs: HashMap<_, (_, _)> = inputs
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(&a, &b)| (a + b, (a, b)))
        .collect();

    for &a in inputs.iter() {
        let remainder = 2020 - a;
        if let Some(&(b, c)) = remainder_pairs.get(&remainder) {
            out.replace((a, b, c));
            break;
        }
    }
    let part2_dur = part2_start.elapsed();

    match out {
        Some((a, b, c)) => println!(
            "Answer to part 2: {} * {} * {} = {}\nTook {}us",
            a,
            b,
            c,
            a * b * c,
            part2_dur.as_micros()
        ),
        None => println!("No triple adds to 2020"),
    }

    Ok(())
}
