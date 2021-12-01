use crate::password::{Password, SledPassword, TobogganPassword};
use aoc_lib::load_simple_input;

mod password;

fn count_valid(inputs: Vec<impl Password>) -> usize {
    inputs.iter().filter(|pass| pass.validate()).count()
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let inputs: Vec<SledPassword> = load_simple_input("inputs/2.txt")?;
    println!(
        "Number of valid Sled Company passwords: {}",
        count_valid(inputs)
    );

    let inputs: Vec<TobogganPassword> = load_simple_input("inputs/2.txt")?;
    println!(
        "Number of valid Toboggan Corp passwords: {}",
        count_valid(inputs)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count_valid, SledPassword, TobogganPassword};
    use aoc_lib::load_simple_input;

    #[test]
    fn test1() {
        let inputs: Vec<SledPassword> = load_simple_input("test.txt").unwrap();
        assert_eq!(count_valid(inputs), 2);
    }

    #[test]
    fn test2() {
        let inputs: Vec<TobogganPassword> = load_simple_input("test.txt").unwrap();
        assert_eq!(count_valid(inputs), 1);
    }
}
