use aoc_lib::input::load_comma_sep_input;

struct School {
    fish: [u64; 9],
}

impl School {
    fn tick(&mut self) {
        self.fish.rotate_left(1);
        self.fish[6] += self.fish[8];
    }

    fn count(&self) -> u64 {
        self.fish.iter().sum()
    }
}

impl FromIterator<u64> for School {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut fish = [0; 9];
        for age in iter {
            match age {
                a @ 0..=8 => fish[a as usize] += 1,
                a @ _ => panic!("Unexpected age: {} out of lifecylce", a),
            }
        }

        School {fish}
    }
}

fn solve_part1(school: &mut School) -> u64 {
    for _ in 0..80 {
        school.tick();
    }

    school.count()
}

fn solve_part2(school: &mut School) -> u64 {
    for _ in 80..256 {
        school.tick();
    }

    school.count()
}

fn main() -> anyhow::Result<()> {
    let mut school: School = load_comma_sep_input("inputs/6.txt")?;
    println!(
        "After 80 days there would be: {} fish",
        solve_part1(&mut school)
    );
    println!(
        "After 256 days there would be: {} fish",
        solve_part2(&mut school)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, School};
    use aoc_lib::input::parse_comma_sep_test;

    const TEST_INPUT: &'static str = include_str!("test.txt");

    #[test]
    fn test() {
        let mut school: School = parse_comma_sep_test(TEST_INPUT);
        assert_eq!(5934, solve_part1(&mut school));
        assert_eq!(26984457539, solve_part2(&mut school));
    }
}
