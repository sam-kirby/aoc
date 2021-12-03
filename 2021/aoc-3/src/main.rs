use aoc_lib::input::BinaryEncodedInt;

fn calc_gamma_epsilon(input: &[BinaryEncodedInt], digits: usize) -> (u64, u64) {
    let counts = input.iter().fold(vec![0; digits], |c, i| {
        c.into_iter()
            .enumerate()
            .map(|(d, c)| if **i & (1 << d) != 0 { c + 1 } else { c })
            .collect()
    });

    let mut gamma = 0;
    let mut epsilon = 0;
    let threshold = (input.len() as f64 / 2.).ceil() as u64;
    for (i, c) in counts.into_iter().enumerate() {
        if c >= threshold {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    (gamma, epsilon)
}

fn solve_part1(input: &[BinaryEncodedInt], digits: usize) -> u64 {
    let (gamma, epsilon) = calc_gamma_epsilon(input, digits);

    gamma * epsilon
}

fn solve_part2(input: &[BinaryEncodedInt], digits: usize) -> u64 {
    let (gamma, _) = calc_gamma_epsilon(input, digits);

    let (mut o2_candidates, mut co2_candidates): (Vec<_>, Vec<_>) = input
        .iter()
        .copied()
        .partition(|&e| (*e & (1 << (digits - 1))) as u64 == gamma & (1 << (digits - 1)));

    for i in (0..digits - 1).rev() {
        if o2_candidates.len() == 1 {
            break;
        }

        let (gamma, _) = calc_gamma_epsilon(&o2_candidates, digits);

        o2_candidates = o2_candidates
            .into_iter()
            .filter(|&e| (*e & (1 << i)) as u64 == gamma & (1 << i))
            .collect();
    }

    let o2 = *o2_candidates[0];

    for i in (0..digits - 1).rev() {
        if co2_candidates.len() == 1 {
            break;
        }

        let (_, epsilon) = calc_gamma_epsilon(&co2_candidates, i + 1);

        co2_candidates = co2_candidates
            .into_iter()
            .filter(|&e| (*e & (1 << i)) as u64 == epsilon & (1 << i))
            .collect();
    }

    let co2 = *co2_candidates[0];

    println!("O2: {}, CO2: {}", o2, co2);

    o2 as u64 * co2 as u64
}

fn main() -> anyhow::Result<()> {
    let input: Vec<_> = aoc_lib::input::load_simple_input("inputs/3.txt")?;

    println!("Sub power is: {}", solve_part1(&input, 12));
    println!("Life support rating is: {}", solve_part2(&input, 12));

    Ok(())
}

#[cfg(test)]
mod test {
    use aoc_lib::input::{parse_test_input, BinaryEncodedInt};

    use super::{solve_part1, solve_part2};

    const TEST_INPUT: &'static str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let input: Vec<BinaryEncodedInt> = parse_test_input(TEST_INPUT);
        assert_eq!(198, solve_part1(&input, 5));
    }

    #[test]
    fn test_part2() {
        let input: Vec<BinaryEncodedInt> = parse_test_input(TEST_INPUT);
        assert_eq!(230, solve_part2(&input, 5));
    }
}
