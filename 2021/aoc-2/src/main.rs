use aoc_lib::{
    input::load_simple_input,
    motion::{CartVec, MotionVec, Submarine},
};

/// Convert submarine's motion vectors to cartesian coordinates then sum
fn solve_part1(path: &[MotionVec]) -> i64 {
    let travel_vec: CartVec = path.iter().map(|m| m.dis * m.dir.to_cart_vec()).sum();
    println!("Final coordinates: {}, {}", travel_vec.x, travel_vec.z);
    travel_vec.x * travel_vec.z
}

/// Iteratively apply motion vectors to submarine
fn solve_part2(path: &[MotionVec]) -> i64 {
    let sub: Submarine = path.iter().fold(Submarine::default(), |s, v| s + v);
    println!("Final coordinates: {}, {}", sub.x, sub.d);
    sub.x * sub.d
}

fn main() -> anyhow::Result<()> {
    let sub_path: Vec<MotionVec> = load_simple_input("inputs/2.txt")?;

    // Why not ask for the scalar distance?!
    println!(
        "Meaningless answer for part one is: {}",
        solve_part1(&sub_path)
    );
    println!(
        "Similarly meaningless answer for part two is: {}",
        solve_part2(&sub_path)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_lib::{input::parse_test_input, motion::MotionVec};

    use super::{solve_part1, solve_part2};

    const TEST_INPUT: &'static str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let path: Vec<MotionVec> = parse_test_input(TEST_INPUT);
        assert_eq!(150, solve_part1(&path));
    }

    #[test]
    fn test_part2() {
        let path: Vec<MotionVec> = parse_test_input(TEST_INPUT);
        assert_eq!(900, solve_part2(&path));
    }
}
