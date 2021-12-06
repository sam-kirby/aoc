use std::{fs::File, io::Read};

use anyhow::Context;
use aoc_lib::bingo::Hall;

fn main() -> anyhow::Result<()> {
    let mut bingo_data = String::new();
    let mut file = File::open("inputs/4.txt").context("input file not found")?;
    file.read_to_string(&mut bingo_data)
        .context("failed to read input")?;

    let mut bingo_hall: Hall = bingo_data.parse()?;

    println!(
        "The score of the first table to win is: {}",
        bingo_hall.next().expect("No tables win")
    );

    println!(
        "The score of the last table to win is {}",
        bingo_hall.last().expect("Only the first table won")
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use aoc_lib::bingo::Hall;

    const TEST_INPUT: &'static str = include_str!("test.txt");

    #[test]
    fn test() {
        let mut bingo_hall: Hall = TEST_INPUT.parse().expect("Failed to parse test input");
        assert_eq!(4512, bingo_hall.next().unwrap());
        assert_eq!(1924, bingo_hall.last().unwrap());
    }
}
