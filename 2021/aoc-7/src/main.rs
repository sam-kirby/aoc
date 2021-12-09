use std::collections::HashMap;

use aoc_lib::input::load_comma_sep_input;

// int_abs_diff not stable
pub const fn abs_diff(first: u32, second: u32) -> u32 {
    if first < second {
        second - first
    } else {
        first - second
    }
}

struct Crabs {
    pos: HashMap<u32, u32>,
}

impl Crabs {
    fn align_to_naive(&self, target_pos: u32) -> u32 {
        let mut fuel_cost = 0;
        for (pos, count) in &self.pos {
            fuel_cost += abs_diff(*pos, target_pos) * count;
        }

        fuel_cost
    }

    fn align_to(&self, target_pos: u32) -> u32 {
        let mut fuel_cost = 0;

        for (pos, count) in &self.pos {
            let diff = abs_diff(*pos, target_pos);
            fuel_cost += count * diff * (diff + 1) / 2
        }

        fuel_cost
    }

    fn minimum_cost(&self, naive: bool) -> u32 {
        let minimum = self.pos.keys().min().expect("There should be crabs");
        let maximum = self.pos.keys().max().expect("There should be crabs");

        let mut costs = Vec::with_capacity(self.pos.keys().len());

        for pos in *minimum..=*maximum {
            costs.push(if naive {
                self.align_to_naive(pos)
            } else {
                self.align_to(pos)
            });
        }

        *costs.iter().min().expect("There should be crabs")
    }
}

impl FromIterator<u32> for Crabs {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut pos = HashMap::new();

        for crab in iter {
            pos.entry(crab).and_modify(|c| *c += 1).or_insert(1);
        }

        Crabs { pos }
    }
}

fn main() -> anyhow::Result<()> {
    let crabs: Crabs = load_comma_sep_input("inputs/7.txt")?;
    println!(
        "The minimum required fuel to move into alignment is: {}",
        crabs.minimum_cost(true)
    );
    println!(
        "Using improved crab maths, the fuel required is: {}",
        crabs.minimum_cost(false)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_lib::input::parse_comma_sep_test;

    use super::Crabs;

    const TEST_INPUT: &'static str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let crabs: Crabs = parse_comma_sep_test(TEST_INPUT);
        assert_eq!(37, crabs.minimum_cost(true));
    }

    #[test]
    fn test_part2() {
        let crabs: Crabs = parse_comma_sep_test(TEST_INPUT);
        assert_eq!(168, crabs.minimum_cost(false));
    }
}
