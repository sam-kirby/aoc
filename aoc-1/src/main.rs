use aoc_lib::load_simple_input;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

/// Solution for Advent of Code 2020 day 1
/// Makes use of O(1) lookup for HashSets to give approximately O(n) behaviour for both parts
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let inputs: HashSet<usize> = load_simple_input("inputs/1.txt")?;

    for &input in inputs.iter() {
        let remainder = 2020 - input;
        if inputs.contains(&remainder) {
            println!(
                "Answer to part 1: {} * {} = {}",
                input,
                remainder,
                input * remainder
            );
            break;
        }
    }

    let remainder_pairs: HashMap<_, (_, _)> = inputs
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(&a, &b)| (a + b, (a, b)))
        .collect();

    for &a in inputs.iter() {
        let remainder = 2020 - a;
        if let Some(&(b, c)) = remainder_pairs.get(&remainder) {
            println!("Answer to part 2: {} * {} * {} = {}", a, b, c, a * b * c);
            break;
        }
    }

    Ok(())
}
