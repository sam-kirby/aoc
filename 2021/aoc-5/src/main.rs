use aoc_lib::{
    input::load_simple_input,
    vents::{Field, Vent},
};

fn solve_part1(vents: &[Vent]) -> usize {
    let field = vents
        .iter()
        .filter(|v| v.is_aligned())
        .fold(Field::default(), |mut f, v| {
            f.add_vent(v);
            f
        });

    field.danger_zones()
}

fn solve_part2(vents: &[Vent]) -> usize {
    let field = vents.iter().fold(Field::default(), |mut f, v| {
        f.add_vent(v);
        f
    });

    field.danger_zones()
}

fn main() -> anyhow::Result<()> {
    let vents: Vec<_> = load_simple_input("inputs/5.txt")?;
    println!(
        "Danger zones (considering only aligned vents): {}",
        solve_part1(&vents)
    );
    println!(
        "Danger zones (considering all vents): {}",
        solve_part2(&vents)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_lib::{input::parse_test_input, vents::Vent};

    use crate::{solve_part1, solve_part2};

    const TEST_INPUT: &'static str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let vents: Vec<Vent> = parse_test_input(&TEST_INPUT);

        assert_eq!(5, solve_part1(&vents));
    }

    #[test]
    fn test_part2() {
        let vents: Vec<Vent> = parse_test_input(&TEST_INPUT);

        assert_eq!(12, solve_part2(&vents));
    }
}
