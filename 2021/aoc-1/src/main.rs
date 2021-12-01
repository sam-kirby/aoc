fn solve_part1(input: &[u32]) -> usize {
    input
        .iter()
        .fold((0, None), |(mut t, p), &c| {
            if matches!(p, Some(p) if c > p) {
                t += 1;
            }
            (t, Some(c))
        })
        .0
}

fn solve_part2(input: &[u32]) -> usize {
    input.windows(3).fold((0, None), |(mut t, p), c| {
        let s: u32 = c.iter().sum();
        if matches!(p, Some(p) if s > p) {
            t += 1;
        }
        (t, Some(s))
    }).0
}

fn main() -> anyhow::Result<()> {
    let input: Vec<_> = aoc_lib::input::load_simple_input("inputs/1.txt")?;

    println!("Depth decreases {} times", solve_part1(&input));
    println!("Depth of window decreases {} times", solve_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&INPUT), 5);
    }
}
